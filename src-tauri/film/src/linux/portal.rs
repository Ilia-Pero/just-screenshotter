//! The portal through which the desktop shows the screen.

use std::os::fd::{AsRawFd, OwnedFd};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

use ashpd::desktop::screencast::{
    CursorMode, OpenPipeWireRemoteOptions, Screencast, SelectSourcesOptions, SourceType,
    StartCastOptions,
};
use ashpd::desktop::{CreateSessionOptions, PersistMode, Session};
use ashpd::zbus::Connection;
use ashpd::Error;
use futures_util::StreamExt;
use rustix::io::{fcntl_setfd, FdFlags};
use tokio::sync::oneshot;

use crate::pipeline::Scene;
use crate::Failure;

/// The screen that a person has agreed to show.
pub struct Portal {
    remote: OwnedFd,
    node: u32,
    stop: oneshot::Sender<()>,
    keeper: JoinHandle<()>,
}

impl Portal {
    /// Asks for the screen. The bell rings when a person forbids to film any further.
    pub fn open(bell: Sender<()>) -> Result<Portal, Failure> {
        let (tell, told) = mpsc::channel();
        let (stop, stopped) = oneshot::channel();
        let keeper = thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();
            match runtime {
                Ok(runtime) => runtime.block_on(keep(tell, stopped, bell)),
                Err(failure) => {
                    let _ = tell.send(Err(failure.into()));
                }
            }
        });
        let (remote, node) = told.recv()??;
        // The program that films inherits the descriptor.
        fcntl_setfd(&remote, FdFlags::empty())?;
        Ok(Portal {
            remote,
            node,
            stop,
            keeper,
        })
    }

    /// Where the pictures come from.
    pub fn scene(&self) -> Scene {
        Scene::Portal {
            remote: self.remote.as_raw_fd(),
            node: self.node,
        }
    }

    /// Tells the desktop that the screen is not needed any more.
    pub fn close(self) {
        let _ = self.stop.send(());
        let _ = self.keeper.join();
    }
}

type Shown = Result<(OwnedFd, u32), Failure>;

/// Keeps the screen open until it is not needed or a person closes it.
async fn keep(tell: Sender<Shown>, stopped: oneshot::Receiver<()>, bell: Sender<()>) {
    match show().await {
        Err(failure) => {
            let _ = tell.send(Err(failure));
        }
        Ok((session, remote, node)) => {
            let _ = tell.send(Ok((remote, node)));
            if let Ok(mut closed) = session.receive_closed().await {
                tokio::select! {
                    _ = closed.next() => {
                        // If nobody listens to the bell, filming is being finished already.
                        let _ = bell.send(());
                    }
                    _ = stopped => {}
                }
            }
            let _ = session.close().await;
        }
    }
}

/// Asks a person what monitor to show.
async fn show() -> Result<(Session<Screencast>, OwnedFd, u32), Failure> {
    let portal = Screencast::with_connection(Connection::session().await?).await?;
    let session = portal
        .create_session(CreateSessionOptions::default())
        .await?;
    portal
        .select_sources(
            &session,
            SelectSourcesOptions::default()
                .set_cursor_mode(CursorMode::EMBEDDED)
                .set_sources(SourceType::MONITOR)
                .set_multiple(false)
                .set_persist_mode(PersistMode::DoNot),
        )
        .await?;
    let answer = portal
        .start(&session, None, StartCastOptions::default())
        .await?
        .response();
    let streams = match answer {
        // A person has changed their mind: there is nothing to tell them.
        Err(Error::Response(_)) => return Err("".into()),
        other => other?,
    };
    let node = streams
        .streams()
        .first()
        .ok_or("The desktop has shown nothing")?
        .pipe_wire_node_id();
    let remote = portal
        .open_pipe_wire_remote(&session, OpenPipeWireRemoteOptions::default())
        .await?;
    Ok((session, remote, node))
}
