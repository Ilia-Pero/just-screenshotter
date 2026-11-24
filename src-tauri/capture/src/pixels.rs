//! A picture: pixels in memory and PNG.

use std::io::Cursor;

use crate::{Failure, Patch};

/// The length of `BITMAPINFOHEADER`.
const HEADER: u32 = 40;

/// Pixels of a picture: three bytes (red, green, blue) for each, rows from top to bottom.
#[derive(Debug, PartialEq, Eq)]
pub struct Pixels {
    pub width: u32,
    pub height: u32,
    pub rgb: Vec<u8>,
}

impl Pixels {
    /// The same pixels written as PNG. Compression is fast: the editor has to open at once.
    ///
    /// # Errors
    ///
    /// The number of bytes does not match the size of the picture.
    pub fn png(&self) -> Result<Vec<u8>, Failure> {
        let mut out = Vec::new();
        let mut encoder = png::Encoder::new(&mut out, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Fast);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.rgb)?;
        writer.finish()?;
        Ok(out)
    }

    /// The same pixels the way the Windows clipboard keeps a picture (`CF_DIB`):
    /// a header, then rows from bottom to top, each padded to four bytes.
    ///
    /// # Errors
    ///
    /// The picture is too large for this format.
    pub fn dib(&self) -> Result<Vec<u8>, Failure> {
        let line = self.width as usize * 3;
        let padding = vec![0; line.next_multiple_of(4) - line];
        let size = u32::try_from((line + padding.len()) * self.height as usize)?;
        let mut out = Vec::with_capacity(HEADER as usize + size as usize);
        out.extend(HEADER.to_le_bytes());
        out.extend(i32::try_from(self.width)?.to_le_bytes());
        out.extend(i32::try_from(self.height)?.to_le_bytes());
        out.extend(1_u16.to_le_bytes());
        out.extend(24_u16.to_le_bytes());
        out.extend(0_u32.to_le_bytes());
        out.extend(size.to_le_bytes());
        out.extend([0; 16]);
        for row in self.rgb.chunks(line).rev() {
            for &[red, green, blue] in row.as_chunks::<3>().0 {
                out.extend([blue, green, red]);
            }
            out.extend(&padding);
        }
        Ok(out)
    }

    /// A part of the picture. Whatever is beyond its edge is dropped.
    ///
    /// # Errors
    ///
    /// The whole part lies beyond the edge.
    pub fn cut(&self, patch: &Patch) -> Result<Pixels, Failure> {
        let left = u32::try_from(patch.left.max(0))?.min(self.width);
        let top = u32::try_from(patch.top.max(0))?.min(self.height);
        let width = patch.width.min(self.width - left);
        let height = patch.height.min(self.height - top);
        if width == 0 || height == 0 {
            return Err(format!(
                "{patch:?} lies beyond a picture of {} by {}",
                self.width, self.height
            )
            .into());
        }
        let line = self.width as usize * 3;
        let from = left as usize * 3;
        Ok(Pixels {
            width,
            height,
            rgb: self
                .rgb
                .chunks(line)
                .skip(top as usize)
                .take(height as usize)
                .flat_map(|row| &row[from..from + width as usize * 3])
                .copied()
                .collect(),
        })
    }
}

/// A picture in PNG.
pub struct Picture {
    png: Vec<u8>,
}

impl Picture {
    #[must_use]
    pub fn new(png: Vec<u8>) -> Self {
        Self { png }
    }

    /// Pixels of the picture, without transparency.
    ///
    /// # Errors
    ///
    /// This is not PNG, or the file is damaged.
    pub fn pixels(&self) -> Result<Pixels, Failure> {
        let mut decoder = png::Decoder::new(Cursor::new(&self.png));
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let size = reader
            .output_buffer_size()
            .ok_or("The picture is too large")?;
        let mut bytes = vec![0; size];
        let info = reader.next_frame(&mut bytes)?;
        bytes.truncate(info.buffer_size());
        Ok(Pixels {
            width: info.width,
            height: info.height,
            rgb: match info.color_type {
                png::ColorType::Rgb => bytes,
                png::ColorType::Rgba => bytes
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .flat_map(|&[red, green, blue, _]| [red, green, blue])
                    .collect(),
                png::ColorType::Grayscale => {
                    bytes.iter().flat_map(|&gray| [gray, gray, gray]).collect()
                }
                png::ColorType::GrayscaleAlpha => bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .flat_map(|&[gray, _]| [gray, gray, gray])
                    .collect(),
                png::ColorType::Indexed => return Err("The palette is not expanded".into()),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Picture, Pixels};
    use crate::Patch;

    /// A picture of 3 by 2: red is the number of the column, green is the number of the row.
    fn grid() -> Pixels {
        Pixels {
            width: 3,
            height: 2,
            rgb: vec![0, 0, 9, 1, 0, 9, 2, 0, 9, 0, 1, 9, 1, 1, 9, 2, 1, 9],
        }
    }

    fn patch(left: i32, top: i32, width: u32, height: u32) -> Patch {
        Patch {
            left,
            top,
            width,
            height,
        }
    }

    #[test]
    fn survives_png_round_trip() {
        assert_eq!(
            Picture::new(grid().png().unwrap()).pixels().unwrap(),
            grid()
        );
    }

    #[test]
    fn keeps_only_requested_pixels() {
        assert_eq!(
            grid().cut(&patch(1, 1, 2, 1)).unwrap().rgb,
            vec![1, 1, 9, 2, 1, 9]
        );
    }

    #[test]
    fn drops_part_beyond_the_edge() {
        assert_eq!(grid().cut(&patch(2, 0, 50, 50)).unwrap().width, 1);
    }

    #[test]
    fn refuses_patch_outside_the_picture() {
        assert!(grid().cut(&patch(3, 0, 5, 5)).is_err());
    }

    #[test]
    fn writes_clipboard_rows_bottom_up_in_blue_green_red() {
        assert_eq!(
            grid().dib().unwrap()[40..],
            [
                9, 1, 0, 9, 1, 1, 9, 1, 2, 0, 0,
                0, // the bottom row and three bytes of padding
                9, 0, 0, 9, 0, 1, 9, 0, 2, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn states_size_in_clipboard_header() {
        assert_eq!(grid().dib().unwrap()[4..12], [3, 0, 0, 0, 2, 0, 0, 0]);
    }

    #[test]
    fn refuses_bytes_that_are_not_png() {
        assert!(Picture::new(vec![1, 2, 3]).pixels().is_err());
    }
}
