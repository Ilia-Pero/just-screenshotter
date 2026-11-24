//! A rectangular area of the desktop.

/// A rectangle on the desktop, in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Patch {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

impl Patch {
    /// The smallest area that holds both this one and the other.
    #[must_use]
    pub fn joined(&self, other: &Patch) -> Patch {
        let left = self.left.min(other.left);
        let top = self.top.min(other.top);
        Patch {
            left,
            top,
            width: span(i64::from(left), self.right().max(other.right())),
            height: span(i64::from(top), self.bottom().max(other.bottom())),
        }
    }

    /// Where this area lies on a picture of the desktop `whole`, if the picture is `width` by
    /// `height`. The picture may be larger or smaller than the desktop itself.
    #[must_use]
    pub fn on(&self, whole: &Patch, width: u32, height: u32) -> Patch {
        let across = |x: i64| scaled(x - i64::from(whole.left), whole.width, width);
        let down = |y: i64| scaled(y - i64::from(whole.top), whole.height, height);
        let left = across(i64::from(self.left));
        let top = down(i64::from(self.top));
        Patch {
            left: i32::try_from(left).unwrap_or(i32::MAX),
            top: i32::try_from(top).unwrap_or(i32::MAX),
            width: span(left, across(self.right())),
            height: span(top, down(self.bottom())),
        }
    }

    fn right(&self) -> i64 {
        i64::from(self.left) + i64::from(self.width)
    }

    fn bottom(&self) -> i64 {
        i64::from(self.top) + i64::from(self.height)
    }
}

/// The length of a segment. It is zero if the ends are swapped.
fn span(from: i64, to: i64) -> u32 {
    u32::try_from((to - from).max(0)).unwrap_or(u32::MAX)
}

/// A coordinate on a segment of length `from`, moved to a segment of length `to` and rounded.
fn scaled(value: i64, from: u32, to: u32) -> i64 {
    let from = i64::from(from).max(1);
    (value * i64::from(to) + from / 2).div_euclid(from)
}

#[cfg(test)]
mod tests {
    use super::Patch;

    const LEFT: Patch = Patch {
        left: -1920,
        top: 0,
        width: 1920,
        height: 1080,
    };
    const RIGHT: Patch = Patch {
        left: 0,
        top: 0,
        width: 2560,
        height: 1440,
    };

    #[test]
    fn spans_both_monitors_when_joined() {
        assert_eq!(
            LEFT.joined(&RIGHT),
            Patch {
                left: -1920,
                top: 0,
                width: 4480,
                height: 1440
            }
        );
    }

    #[test]
    fn starts_after_left_monitor_on_picture_of_same_size() {
        assert_eq!(
            RIGHT.on(&LEFT.joined(&RIGHT), 4480, 1440),
            Patch {
                left: 1920,
                top: 0,
                width: 2560,
                height: 1440
            }
        );
    }

    #[test]
    fn doubles_on_picture_twice_as_large() {
        assert_eq!(
            RIGHT.on(&LEFT.joined(&RIGHT), 8960, 2880),
            Patch {
                left: 3840,
                top: 0,
                width: 5120,
                height: 2880
            }
        );
    }

    #[test]
    fn leaves_no_gap_between_neighbours_on_uneven_scale() {
        let whole = LEFT.joined(&RIGHT);
        let left = LEFT.on(&whole, 5601, 1801);
        assert_eq!(
            i64::from(left.left) + i64::from(left.width),
            i64::from(RIGHT.on(&whole, 5601, 1801).left)
        );
    }
}
