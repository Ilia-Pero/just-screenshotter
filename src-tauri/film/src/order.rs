//! What to film.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A rectangle on the screen, in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Area {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

impl Area {
    /// The part of the area that lies within a screen of the given size, with even sides:
    /// video encoders accept no others.
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

    /// True if there is nothing to film.
    #[must_use]
    pub fn blank(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// The beginning and the even length of the part of a segment that lies from zero to the limit.
fn cut(from: i32, length: u32, limit: u32) -> (i32, u32) {
    let start = i64::from(from).clamp(0, i64::from(limit));
    let end = (i64::from(from) + i64::from(length)).clamp(start, i64::from(limit));
    (i32::try_from(start).unwrap_or(0), span(start, end) / 2 * 2)
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
