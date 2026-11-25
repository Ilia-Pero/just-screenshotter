//! Keys taken from X11.

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    ClientMessageEvent, ConnectionExt, CreateWindowAux, EventMask, GrabMode, ModMask, WindowClass,
};
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;

use crate::{Failure, Key};

/// Keys that must not matter: Caps Lock and Num Lock.
const LOCKS: [u16; 4] = [0, 1 << 1, 1 << 4, (1 << 1) | (1 << 4)];

/// The bits of held keys among the bits of a press: buttons of the mouse are there too.
const HELD: u16 = 0xff;

/// Keys that other programs do not hear. Dropping it lets them go.
pub struct Grab {
    connection: Arc<RustConnection>,
    /// A window nobody sees. A message to it ends the thread.
    window: u32,
    thread: Option<JoinHandle<()>>,
}

/// A taken key: its code, the keys held with it, and its number.
struct Taken {
    code: u8,
    held: u16,
    number: usize,
}

impl Grab {
    /// Takes the keys. The bell rings with the number of a key in a thread of the grab. Returns
    /// also the numbers of keys that other programs have taken, or that are not on the keyboard.
    ///
    /// # Errors
    ///
    /// There is no X11.
    pub fn start(
        keys: &[Key],
        ring: impl Fn(usize) + Send + 'static,
    ) -> Result<(Grab, Vec<usize>), Failure> {
        let (connection, screen) = x11rb::connect(None)?;
        let root = connection
            .setup()
            .roots
            .get(screen)
            .ok_or("There is no screen")?
            .root;
        let window = connection.generate_id()?;
        connection
            .create_window(
                0,
                window,
                root,
                0,
                0,
                1,
                1,
                0,
                WindowClass::INPUT_ONLY,
                0,
                &CreateWindowAux::new(),
            )?
            .check()?;
        let mut taken = Vec::new();
        let mut busy = Vec::new();
        for (number, key) in keys.iter().enumerate() {
            match code(&connection, key.symbol)? {
                Some(code) if seized(&connection, root, code, key.held)? => taken.push(Taken {
                    code,
                    held: key.held,
                    number,
                }),
                _ => busy.push(number),
            }
        }
        connection.flush()?;
        let connection = Arc::new(connection);
        let thread = thread::spawn({
            let connection = Arc::clone(&connection);
            move || listen(&connection, &taken, &ring)
        });
        Ok((
            Grab {
                connection,
                window,
                thread: Some(thread),
            },
            busy,
        ))
    }
}

impl Drop for Grab {
    fn drop(&mut self) {
        let message = ClientMessageEvent::new(8, self.window, self.window, [0_u8; 20]);
        let sent = self
            .connection
            .send_event(false, self.window, EventMask::NO_EVENT, message)
            .is_ok()
            && self.connection.flush().is_ok();
        if let Some(thread) = self.thread.take().filter(|_| sent) {
            // A crash of the thread lets the keys go as well.
            let _ = thread.join();
        }
    }
}

/// Sleeps until the server says something. The end of the connection lets the keys go.
fn listen(connection: &RustConnection, taken: &[Taken], ring: &impl Fn(usize)) {
    while let Ok(event) = connection.wait_for_event() {
        match event {
            Event::KeyPress(press) => {
                let held = u16::from(press.state) & HELD & !LOCKS[3];
                taken
                    .iter()
                    .filter(|key| key.code == press.detail && key.held == held)
                    .for_each(|key| ring(key.number));
            }
            Event::ClientMessage(_) => break,
            _ => {}
        }
    }
}

/// The place of the key on the keyboard, in whatever layout it is.
fn code(connection: &RustConnection, symbol: u32) -> Result<Option<u8>, Failure> {
    let first = connection.setup().min_keycode;
    let count = connection.setup().max_keycode - first + 1;
    let mapping = connection.get_keyboard_mapping(first, count)?.reply()?;
    let width = usize::from(mapping.keysyms_per_keycode).max(1);
    let place = mapping
        .keysyms
        .chunks(width)
        .position(|symbols| symbols.contains(&symbol));
    Ok(place
        .and_then(|place| u8::try_from(place).ok())
        .map(|place| first + place))
}

/// Takes the key from all programs. False if one of them has taken it already.
fn seized(connection: &RustConnection, root: u32, code: u8, held: u16) -> Result<bool, Failure> {
    for locks in LOCKS {
        let answer = connection
            .grab_key(
                false,
                root,
                ModMask::from(held | locks),
                code,
                GrabMode::ASYNC,
                GrabMode::ASYNC,
            )?
            .check();
        if answer.is_err() {
            connection
                .ungrab_key(code, root, ModMask::from(held))?
                .check()?;
            return Ok(false);
        }
    }
    Ok(true)
}
