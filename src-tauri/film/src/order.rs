//! What to film.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The shortest side of a film: the encoder of Windows does not take pictures smaller than 48
/// pixels, and encoders of video cards want more.
const LEAST: i64 = 160;

/// A rectangle on the screen, in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Area {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

impl Area {
    /// The part of the area that lies within a screen of the given size. It has even sides,
    /// because video encoders accept no others, and it grows if it is too small for them.
    #[must_use]
    pub fn within(&self, width: u32, height: u32) -> Area {
        let (left, wide) = cut(self.left, self.width, width);
        let (top, high) = cut(self.top, self.height, height);
        Area {
            left,
            top,
            width: wide,
            height: high,
        }
    }

    /// The same area of a picture where everything is larger or smaller, for example because
    /// the picture of the screen does not match its size.
    #[must_use]
    pub fn scaled(&self, from: (u32, u32), to: (u32, u32)) -> Area {
        let across = |x: i64| x * i64::from(to.0) / i64::from(from.0.max(1));
        let down = |y: i64| y * i64::from(to.1) / i64::from(from.1.max(1));
        let left = across(i64::from(self.left));
        let top = down(i64::from(self.top));
        Area {
            left: i32::try_from(left).unwrap_or(0),
            top: i32::try_from(top).unwrap_or(0),
            width: span(left, across(i64::from(self.left) + i64::from(self.width))),
            height: span(top, down(i64::from(self.top) + i64::from(self.height))),
        }
    }

    /// Where a window of the given size stands next to the area: under it, above it, or inside it
    /// if there is no other place on the monitor. Counts from the corner of the monitor.
    #[must_use]
    pub fn beside(&self, monitor: (u32, u32), window: (u32, u32), gap: u32) -> (i32, i32) {
        let top = i64::from(self.top);
        let bottom = top + i64::from(self.height);
        let room = i64::from(window.1) + i64::from(gap);
        let down = if bottom + room <= i64::from(monitor.1) {
            bottom + i64::from(gap)
        } else if top >= room {
            top - room
        } else {
            bottom - room
        };
        let across = (i64::from(self.left) + (i64::from(self.width) - i64::from(window.0)) / 2)
            .min(i64::from(monitor.0) - i64::from(window.0))
            .max(0);
        (
            i32::try_from(across).unwrap_or(0),
            i32::try_from(down).unwrap_or(0),
        )
    }

    /// The bits of video in a second: enough for text to stay sharp.
    #[must_use]
    pub fn bits(&self, pace: u32) -> u32 {
        let pixels = u64::from(self.width) * u64::from(self.height) * u64::from(pace);
        u32::try_from((pixels * 8 / 100).clamp(1_000_000, 24_000_000)).unwrap_or(24_000_000)
    }

    /// The even size of a picture of the area that is not larger than the limit.
    #[must_use]
    pub fn shrunk(&self, limit: (u32, u32)) -> (u32, u32) {
        let (width, height) = (u64::from(self.width), u64::from(self.height));
        let (wide, high) = (u64::from(limit.0), u64::from(limit.1));
        let (across, down) = if width <= wide && height <= high {
            (width, height)
        } else if width * high > height * wide {
            (wide, height * wide / width)
        } else {
            (width * high / height, high)
        };
        (even(across), even(down))
    }

    /// True if there is nothing to film.
    #[must_use]
    pub fn blank(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// The beginning and the even length of the part of a segment that lies from zero to the limit.
/// If the part is too short, it grows, but not beyond the limit.
fn cut(from: i32, length: u32, limit: u32) -> (i32, u32) {
    let limit = i64::from(limit);
    let start = i64::from(from).clamp(0, limit);
    let end = (i64::from(from) + i64::from(length)).clamp(start, limit);
    let long = if start == end {
        0
    } else {
        (end - start).max(LEAST.min(limit)) / 2 * 2
    };
    (
        i32::try_from(start.min(limit - long)).unwrap_or(0),
        span(0, long),
    )
}

/// The nearest even length that is not longer.
fn even(length: u64) -> u32 {
    u32::try_from(length / 2 * 2).unwrap_or(u32::MAX - 1)
}

/// The length of a segment.
fn span(from: i64, to: i64) -> u32 {
    u32::try_from((to - from).max(0)).unwrap_or(u32::MAX)
}

/// An order for filming.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// The monitor on the desktop.
    pub monitor: Area,
    /// How many physical pixels there are in one logical pixel of the monitor.
    pub scale: f64,
    /// What to film, counting from the corner of the monitor.
    pub area: Area,
    /// Whether to record what is heard from the computer.
    pub sound: bool,
    /// Whether to record the microphone.
    pub voice: bool,
    /// The file that does not exist yet.
    pub file: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::Area;

    const AREA: Area = Area {
        left: 10,
        top: 20,
        width: 301,
        height: 200,
    };

    #[test]
    fn has_even_sides_within_the_screen() {
        assert_eq!(AREA.within(1920, 1080).width, 300);
    }

    #[test]
    fn is_cut_by_the_right_edge_of_the_screen() {
        assert_eq!(AREA.within(200, 1080).width, 190);
    }

    #[test]
    fn is_cut_by_the_left_edge_of_the_screen() {
        assert_eq!(
            Area { left: -100, ..AREA }.within(1920, 1080),
            Area {
                left: 0,
                width: 200,
                ..AREA
            }
        );
    }

    #[test]
    fn grows_if_it_is_too_small_for_video() {
        assert_eq!(Area { height: 20, ..AREA }.within(1920, 1080).height, 160);
    }

    #[test]
    fn grows_away_from_the_edge_of_the_screen() {
        assert_eq!(
            Area {
                left: 1900,
                width: 20,
                ..AREA
            }
            .within(1920, 1080)
            .left,
            1760
        );
    }

    #[test]
    fn does_not_grow_beyond_a_small_screen() {
        assert_eq!(Area { width: 20, ..AREA }.within(101, 1080).width, 100);
    }

    #[test]
    fn is_blank_outside_the_screen() {
        assert!(AREA.within(5, 5).blank());
    }

    /// Where a window of 200 by 40 stands on a monitor of 1920 by 1080, next to the area.
    fn window(left: i32, top: i32, height: u32) -> (i32, i32) {
        Area {
            left,
            top,
            width: 400,
            height,
        }
        .beside((1920, 1080), (200, 40), 10)
    }

    #[test]
    fn has_a_window_under_it() {
        assert_eq!(window(100, 100, 300), (200, 410));
    }

    #[test]
    fn has_a_window_above_it_at_the_bottom_of_the_screen() {
        assert_eq!(window(100, 700, 380).1, 650);
    }

    #[test]
    fn has_a_window_inside_it_when_it_takes_the_whole_screen() {
        assert_eq!(window(100, 0, 1080).1, 1030);
    }

    #[test]
    fn has_a_window_that_stays_on_the_screen() {
        assert_eq!(window(1800, 100, 300).0, 1720);
    }

    const SCREEN: Area = Area {
        left: 0,
        top: 0,
        width: 5120,
        height: 2880,
    };

    #[test]
    fn gives_a_small_area_enough_bits() {
        assert_eq!(AREA.bits(30), 1_000_000);
    }

    #[test]
    fn gives_a_large_area_more_bits() {
        assert_eq!(
            Area {
                width: 1920,
                height: 1080,
                ..AREA
            }
            .bits(30),
            4_976_640
        );
    }

    #[test]
    fn is_shown_in_its_size_if_it_is_not_too_large() {
        assert_eq!(AREA.shrunk((4096, 2304)), (300, 200));
    }

    #[test]
    fn is_shrunk_if_it_is_too_large() {
        assert_eq!(SCREEN.shrunk((4096, 2304)), (4096, 2304));
    }

    #[test]
    fn is_shrunk_by_its_width_if_it_is_wide() {
        assert_eq!(
            Area {
                height: 1000,
                ..SCREEN
            }
            .shrunk((4096, 2304)),
            (4096, 800)
        );
    }

    #[test]
    fn is_shrunk_by_its_height_if_it_is_high() {
        assert_eq!(
            Area {
                width: 1000,
                ..SCREEN
            }
            .shrunk((4096, 2304)),
            (800, 2304)
        );
    }

    #[test]
    fn doubles_on_a_picture_twice_as_large() {
        assert_eq!(
            AREA.scaled((1000, 500), (2000, 1000)),
            Area {
                left: 20,
                top: 40,
                width: 602,
                height: 400
            }
        );
    }
}
