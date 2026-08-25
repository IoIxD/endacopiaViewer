use std::io::BufWriter;

use base64::{Engine, engine::general_purpose};
use image::{EncodableLayout, ImageEncoder, Rgb, RgbImage, codecs::png::PngEncoder};

use crate::backend::Backend;

impl Backend {
    pub fn handle_ttf(&mut self, str: String) {
        let ags = self.ags.as_ref().unwrap();
        let bytes = ags.get_file(str.clone());
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();

        let img_width = 800;
        let img_height = 600;
        let mut x_cursor: i32 = 10;
        let y_baseline: i32 = 32; // adjust based on font_size
        let mut image = RgbImage::new(img_width, img_height);

        let text = "The quick brown fox jumps over the lazy dog";
        let mut font_size = 32.0;
        let mut y = 0;

        while font_size > 0.0 {
            for ch in text.chars() {
                let (metrics, bitmap) = font.rasterize(ch, font_size);

                // bitmap is a Vec<u8> of coverage values (0-255), one per pixel,
                // row-major, size metrics.width * metrics.height
                for row in 0..metrics.height {
                    for col in 0..metrics.width {
                        let coverage = bitmap[row * metrics.width + col];

                        if coverage == 0 {
                            continue; // fully transparent, skip
                        }

                        // Position on the image
                        let px = x_cursor + col as i32 + metrics.xmin;
                        let py =
                            y + (y_baseline - metrics.ymin - (metrics.height as i32 - row as i32));

                        if px >= 0 && py >= 0 && (px as u32) < img_width && (py as u32) < img_height
                        {
                            // Blend black text onto white background using coverage as alpha
                            let alpha = coverage as f32 / 255.0;
                            let blended = Rgb([
                                255 - (255.0 * (1.0 - alpha) + 0.0 * alpha) as u8, // R
                                255 - (255.0 * (1.0 - alpha) + 0.0 * alpha) as u8, // G
                                255 - (255.0 * (1.0 - alpha) + 0.0 * alpha) as u8, // B
                            ]);
                            image.put_pixel(px as u32, py as u32, blended);
                        }
                    }
                }

                x_cursor += metrics.advance_width.round() as i32;
            }
            y += font_size as i32 + 16;
            font_size -= 2.0;
            x_cursor = 10;
        }

        let mut buf = vec![];
        let pngbuf = BufWriter::new(&mut buf);
        let encoder = PngEncoder::new(pngbuf);
        encoder
            .write_image(
                image.as_bytes(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgb8,
            )
            .unwrap();
        let b64 = general_purpose::STANDARD.encode(buf);

        self.image_data_url = format!("data:image/png;base64,{b64}");
        self.image_display();
    }
}
