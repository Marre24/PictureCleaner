use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui::{self, TextureHandle};

#[derive(Default)]
pub(crate) struct TextureHandler {
    texture_map: HashMap<PathBuf, TextureHandle>,
}

impl TextureHandler {
    pub(crate) fn get_image_from_path(
        &mut self,
        path: &PathBuf,
        ctx: &egui::Context,
    ) -> Option<&TextureHandle> {
        if self.texture_map.contains_key(path) {
            return self.texture_map.get(path);
        }

        let image_bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Could not read image file: {}", e);
                return None;
            }
        };

        let image = match image::load_from_memory(&image_bytes) {
            Ok(img) => img,
            Err(e) => {
                println!("Could not decode image: {}", e);
                return None;
            }
        };

        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        self.texture_map.insert(
            path.to_path_buf(),
            ctx.load_texture("my-image", color_image, Default::default()),
        );

        self.texture_map.get(path)
    }
}
