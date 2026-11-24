//! Pictures the way video encoders take them.

/// How many bytes the picture of the given size takes.
pub fn length(width: usize, height: usize) -> usize {
    width * height * 3 / 2
}

/// Turns the picture of the screen (blue, green, red, spare; a row takes `pitch` bytes) into NV12:
/// the brightness of every pixel, then one pair of colours for every square of four. The colours
/// follow BT.709 with the range of video, 16 to 235. Both sides are even.
pub fn convert(screen: &[u8], pitch: usize, width: usize, out: &mut [u8]) {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        // SAFETY: the processor has just said that it knows these instructions.
        #[allow(unsafe_code)]
        unsafe {
            swift(screen, pitch, width, out);
        }
        return;
    }
    plain(screen, pitch, width, out);
}

/// The same for processors that take eight numbers at once. It is more than twice as fast.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn swift(screen: &[u8], pitch: usize, width: usize, out: &mut [u8]) {
    plain(screen, pitch, width, out);
}

/// The conversion itself. It is compiled into both functions above.
#[inline(always)]
#[allow(clippy::inline_always)]
fn plain(screen: &[u8], pitch: usize, width: usize, out: &mut [u8]) {
    let height = out.len() * 2 / 3 / width.max(1);
    let (light, color) = out.split_at_mut(width * height);
    for ((pair, lights), colors) in screen
        .chunks(pitch * 2)
        .zip(light.chunks_exact_mut(width * 2))
        .zip(color.chunks_exact_mut(width))
    {
        let (upper, lower) = pair.split_at(pitch);
        let (first, second) = lights.split_at_mut(width);
        shine(&upper[..width * 4], first);
        shine(&lower[..width * 4], second);
        paint(&upper[..width * 4], &lower[..width * 4], colors);
    }
}

/// The brightness of a row. A pixel is taken as one number: so compilers turn it into
/// instructions for several pixels at once.
#[inline(always)]
#[allow(clippy::inline_always, clippy::cast_possible_truncation)]
fn shine(row: &[u8], out: &mut [u8]) {
    for (pixel, light) in row.as_chunks::<4>().0.iter().zip(out) {
        let pixel = u32::from_le_bytes(*pixel);
        let sum = (pixel & 0xff) * 16 + ((pixel >> 8) & 0xff) * 157 + ((pixel >> 16) & 0xff) * 47;
        *light = (((sum + 128) >> 8) + 16) as u8;
    }
}

/// The colours of two rows. A pair of pixels is taken as one number.
#[inline(always)]
#[allow(
    clippy::inline_always,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn paint(upper: &[u8], lower: &[u8], out: &mut [u8]) {
    for ((above, below), pair) in upper
        .as_chunks::<8>()
        .0
        .iter()
        .zip(lower.as_chunks::<8>().0)
        .zip(out.as_chunks_mut::<2>().0)
    {
        let above = u64::from_le_bytes(*above);
        let below = u64::from_le_bytes(*below);
        let mean = |at: u32| {
            let part = |pixels: u64, at: u32| ((pixels >> at) & 0xff) as i32;
            let sum =
                part(above, at) + part(above, at + 32) + part(below, at) + part(below, at + 32);
            (sum + 2) >> 2
        };
        let (blue, green, red) = (mean(0), mean(8), mean(16));
        pair[0] = (((112 * blue - 87 * green - 25 * red + 128) >> 8) + 128) as u8;
        pair[1] = (((112 * red - 102 * green - 10 * blue + 128) >> 8) + 128) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::{convert, length};

    /// A picture of 2 by 2 pixels of one colour in NV12.
    fn filled(blue: u8, green: u8, red: u8) -> Vec<u8> {
        let mut out = vec![0; length(2, 2)];
        convert(&[blue, green, red, 0].repeat(4), 8, 2, &mut out);
        out
    }

    #[test]
    fn takes_one_byte_and_a_half_for_a_pixel() {
        assert_eq!(length(4, 2), 12);
    }

    #[test]
    fn black_has_the_lowest_brightness_of_video() {
        assert_eq!(filled(0, 0, 0)[..4], [16; 4]);
    }

    #[test]
    fn white_has_the_highest_brightness_of_video() {
        assert_eq!(filled(255, 255, 255)[..4], [235; 4]);
    }

    #[test]
    fn grey_has_no_colour() {
        assert_eq!(filled(128, 128, 128)[4..], [128; 2]);
    }

    #[test]
    fn red_is_seen_in_the_second_colour() {
        assert_eq!(filled(0, 0, 255)[5], 240);
    }

    #[test]
    fn blue_is_seen_in_the_first_colour() {
        assert_eq!(filled(255, 0, 0)[4], 240);
    }

    #[test]
    fn skips_the_end_of_a_row_that_belongs_to_nothing() {
        let mut out = vec![0; length(2, 2)];
        let row = [[255; 8].as_slice(), &[0; 8]].concat();
        convert(&[row.clone(), row].concat(), 16, 2, &mut out);
        assert_eq!(out[..4], [235; 4]);
    }

    #[test]
    fn reads_the_last_row_without_its_end() {
        let mut out = vec![0; length(2, 2)];
        let row = [[255; 8].as_slice(), &[0; 8]].concat();
        convert(&[row.as_slice(), &[255; 8]].concat(), 16, 2, &mut out);
        assert_eq!(out[2..4], [235; 2]);
    }
}
