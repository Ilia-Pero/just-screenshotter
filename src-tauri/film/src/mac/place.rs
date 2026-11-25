//! Where on the desktop the film is made. Points of the desktop of macOS are logical.

use std::path::Path;

use objc2::rc::Retained;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::{CGDirectDisplayID, CGError, CGGetDisplaysWithPoint};
use objc2_foundation::{NSString, NSURL};

use crate::{Area, Failure, Order};

/// The display where the monitor is.
pub fn display(order: &Order) -> Result<CGDirectDisplayID, Failure> {
    let scale = order.scale.max(1.0);
    let inside = CGPoint {
        x: f64::from(order.monitor.left) / scale + 1.0,
        y: f64::from(order.monitor.top) / scale + 1.0,
    };
    let mut display = 0;
    let mut count = 0;
    // SAFETY: there is a place for one display and for the count, and both outlive the call.
    let found = unsafe { CGGetDisplaysWithPoint(inside, 1, &raw mut display, &raw mut count) };
    if found == CGError::Success && count > 0 {
        Ok(display)
    } else {
        Err("There is no display".into())
    }
}

/// The place of the file.
pub fn url(file: &Path) -> Result<Retained<NSURL>, Failure> {
    let path = file.to_str().ok_or("The name of the file is not Unicode")?;
    Ok(NSURL::fileURLWithPath(&NSString::from_str(path)))
}

/// The area in logical points, counting from the upper left corner of the monitor.
pub fn from_top(area: &Area, scale: f64) -> CGRect {
    let scale = scale.max(1.0);
    CGRect {
        origin: CGPoint {
            x: f64::from(area.left) / scale,
            y: f64::from(area.top) / scale,
        },
        size: CGSize {
            width: f64::from(area.width) / scale,
            height: f64::from(area.height) / scale,
        },
    }
}

/// The area in logical points, counting from the lower left corner of the monitor.
pub fn from_bottom(monitor: &Area, area: &Area, scale: f64) -> CGRect {
    let seen = from_top(area, scale);
    let height = f64::from(monitor.height) / scale.max(1.0);
    CGRect {
        origin: CGPoint {
            x: seen.origin.x,
            y: height - seen.origin.y - seen.size.height,
        },
        size: seen.size,
    }
}

#[cfg(test)]
mod tests {
    use super::{from_bottom, from_top};
    use crate::Area;

    const MONITOR: Area = Area {
        left: 0,
        top: 0,
        width: 2880,
        height: 1800,
    };
    const AREA: Area = Area {
        left: 200,
        top: 100,
        width: 800,
        height: 600,
    };

    #[test]
    fn counts_from_the_bottom() {
        assert!((from_bottom(&MONITOR, &AREA, 2.0).origin.y - 550.0).abs() < 0.01);
    }

    #[test]
    fn counts_from_the_top() {
        assert!((from_top(&AREA, 2.0).origin.y - 50.0).abs() < 0.01);
    }

    #[test]
    fn counts_in_logical_points() {
        assert!((from_bottom(&MONITOR, &AREA, 2.0).size.width - 400.0).abs() < 0.01);
    }
}
