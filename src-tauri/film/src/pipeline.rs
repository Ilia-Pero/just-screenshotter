//! What `gst-launch-1.0` is told to do.

use std::path::Path;

use crate::{Area, Ears};

/// Where pictures come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scene {
    /// The portal of the desktop: the descriptor of `PipeWire` and the number of the stream.
    Portal { remote: i32, node: u32 },
    /// The X server.
    Display,
}

/// A chain of elements that makes a `WebM` file: video in VP8, sound in Opus. Ubuntu has everything
/// for them out of the box.
pub struct Pipeline<'a> {
    pub scene: Scene,
    pub monitor: Area,
    pub area: Area,
    pub ears: Ears,
    pub file: &'a Path,
}

impl Pipeline<'_> {
    /// The arguments of `gst-launch-1.0`.
    pub fn words(&self) -> Vec<String> {
        [
            "-e".to_owned(),
            self.pictures(),
            // Dithering and colours of every pixel are slow and not seen.
            "! videoconvert chroma-mode=none dither=none n-threads=4".to_owned(),
            "! video/x-raw,format=I420 ! queue".to_owned(),
            "! vp8enc deadline=1 cpu-used=16 threads=4 max-quantizer=20".to_owned(),
            "static-threshold=1000 keyframe-max-dist=150".to_owned(),
            "! queue ! mux.".to_owned(),
            self.sounds(),
            "webmmux name=mux ! filesink".to_owned(),
        ]
        .iter()
        .flat_map(|part| part.split_whitespace())
        .map(str::to_owned)
        .chain(std::iter::once(format!("location={}", self.file.display())))
        .collect()
    }

    fn pictures(&self) -> String {
        match self.scene {
            Scene::Display => {
                let left = i64::from(self.monitor.left) + i64::from(self.area.left);
                let top = i64::from(self.monitor.top) + i64::from(self.area.top);
                format!(
                    "ximagesrc use-damage=false startx={left} starty={top} endx={} endy={} \
                     ! video/x-raw,framerate=30/1",
                    left + i64::from(self.area.width) - 1,
                    top + i64::from(self.area.height) - 1,
                )
            }
            // The desktop gives the picture of the whole monitor, and it may be of another size.
            Scene::Portal { remote, node } => format!(
                "pipewiresrc fd={remote} path={node} do-timestamp=true keepalive-time=500 \
                 ! videoscale ! video/x-raw,width={},height={} \
                 ! videocrop left={} top={} right={} bottom={}",
                self.monitor.width,
                self.monitor.height,
                self.area.left,
                self.area.top,
                i64::from(self.monitor.width)
                    - i64::from(self.area.left)
                    - i64::from(self.area.width),
                i64::from(self.monitor.height)
                    - i64::from(self.area.top)
                    - i64::from(self.area.height),
            ),
        }
    }

    fn sounds(&self) -> String {
        const SOUND: &str = "pulsesrc device=@DEFAULT_MONITOR@";
        const VOICE: &str = "pulsesrc";
        const EVEN: &str = "! queue ! audioconvert ! audioresample";
        const OPUS: &str = "! audio/x-raw,rate=48000,channels=2 ! opusenc ! queue ! mux.";
        match (self.ears.sound, self.ears.voice) {
            (false, false) => String::new(),
            (true, false) => format!("{SOUND} {EVEN} {OPUS}"),
            (false, true) => format!("{VOICE} {EVEN} {OPUS}"),
            (true, true) => {
                format!("audiomixer name=mix {OPUS} {SOUND} {EVEN} ! mix. {VOICE} {EVEN} ! mix.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{Pipeline, Scene};
    use crate::{Area, Ears};

    /// The arguments for the scene and the sounds, with spaces between them.
    fn words(scene: Scene, sound: bool, voice: bool) -> String {
        Pipeline {
            scene,
            monitor: Area {
                left: 1920,
                top: 0,
                width: 1920,
                height: 1080,
            },
            area: Area {
                left: 100,
                top: 50,
                width: 800,
                height: 600,
            },
            ears: Ears { sound, voice },
            file: Path::new("a film.webm"),
        }
        .words()
        .join(" ")
    }

    #[test]
    fn films_the_area_of_the_display() {
        assert!(words(Scene::Display, false, false)
            .contains("startx=2020 starty=50 endx=2819 endy=649"));
    }

    #[test]
    fn cuts_the_area_out_of_the_monitor() {
        assert!(words(
            Scene::Portal {
                remote: 7,
                node: 42
            },
            false,
            false
        )
        .contains("videocrop left=100 top=50 right=1020 bottom=430"));
    }

    #[test]
    fn takes_the_stream_of_the_portal() {
        assert!(words(
            Scene::Portal {
                remote: 7,
                node: 42
            },
            false,
            false
        )
        .contains("pipewiresrc fd=7 path=42"));
    }

    #[test]
    fn keeps_spaces_in_the_name_of_the_file() {
        assert!(Pipeline {
            scene: Scene::Display,
            monitor: Area {
                left: 0,
                top: 0,
                width: 2,
                height: 2
            },
            area: Area {
                left: 0,
                top: 0,
                width: 2,
                height: 2
            },
            ears: Ears::default(),
            file: Path::new("a film.webm"),
        }
        .words()
        .contains(&"location=a film.webm".to_owned()));
    }

    #[test]
    fn has_no_sound_when_not_asked() {
        assert!(!words(Scene::Display, false, false).contains("pulsesrc"));
    }

    #[test]
    fn records_the_sound_of_the_computer() {
        assert!(words(Scene::Display, true, false).contains("pulsesrc device=@DEFAULT_MONITOR@"));
    }

    #[test]
    fn mixes_both_sounds() {
        assert_eq!(
            words(Scene::Display, true, true).matches("! mix.").count(),
            2
        );
    }
}
