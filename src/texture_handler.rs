use std::collections::HashMap;
use std::path::{Path, PathBuf};

use eframe::egui::{self, TextureHandle};
use image::GenericImageView;

#[derive(Default)]
pub(crate) struct TextureHandler {
    texture_map: HashMap<PathBuf, TextureHandle>,
}

impl TextureHandler {
    const MAX_SIZE: u32 = 400;

    pub(crate) fn get(&mut self, path: &Path) -> Option<&TextureHandle> {
        self.texture_map.get(path)
    }

    pub(crate) fn get_image_from_path(
        &mut self,
        path: &Path,
        ctx: &egui::Context,
    ) -> Option<&TextureHandle> {
        use std::collections::hash_map::Entry;

        match self.texture_map.entry(path.to_path_buf()) {
            Entry::Occupied(entry) => Some(entry.into_mut()),
            Entry::Vacant(entry) => {
                let texture = Self::load_texture(path, ctx)?;
                Some(entry.insert(texture))
            }
        }
    }

    fn load_texture(path: &Path, ctx: &egui::Context) -> Option<TextureHandle> {
        let image_bytes = std::fs::read(path).ok()?;

        let image = image::load_from_memory(&image_bytes).ok()?;

        let (width, height) = image.dimensions();
        let (new_width, new_height) = if width > height {
            let ratio = Self::MAX_SIZE as f32 / width as f32;
            (Self::MAX_SIZE, (height as f32 * ratio) as u32)
        } else {
            let ratio = Self::MAX_SIZE as f32 / height as f32;
            ((width as f32 * ratio) as u32, Self::MAX_SIZE)
        };

        let resized =
            image.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

        let size = [resized.width() as usize, resized.height() as usize];

        let image_buffer = resized.to_rgba8();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, image_buffer.as_raw());

        Some(ctx.load_texture(
            path.file_name()?.to_str()?,
            color_image,
            egui::TextureOptions::LINEAR,
        ))
    }
}
