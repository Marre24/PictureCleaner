use std::collections::HashMap;
use std::path::{Path, PathBuf};

use eframe::egui::{self, TextureHandle};
use eframe::epaint::tessellator::path;
use image::GenericImageView;

use crate::picture_list::PictureList;

#[derive(Default)]
pub(crate) struct TextureHandler {
    texture_map: HashMap<PathBuf, TextureHandle>,
}

impl TextureHandler {
    const MAX_SIZE: u32 = 400;

    pub(crate) fn get(&mut self, path: &Path) -> Option<&TextureHandle> {
        self.texture_map.get(path)
    }

    pub(crate) fn init(&mut self, ctx: &egui::Context, unchecked_pics: PictureList) {
        for path in unchecked_pics.path_iterator() {
            self.load_texture(path, ctx);
        }
    }

    fn load_texture(&mut self, path: &Path, ctx: &egui::Context) {
        let maybe_image_bytes = std::fs::read(path);
        if let Err(e) = maybe_image_bytes {
            print!("Could not get image bytes: {}", e);
            return;
        }
        let image_bytes = maybe_image_bytes.unwrap();

        let maybe_image = image::load_from_memory(&image_bytes);
        if let Err(e) = maybe_image {
            print!("Could not load image: {}", e);
            return;
        }
        let image = maybe_image.unwrap();

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

        let maybe_file_name = path.file_name();
        if let None = maybe_file_name {
            print!("Could not find file name");
            return;
        }
        let file_name = maybe_file_name.unwrap().to_str().unwrap();

        let img = ctx.load_texture(file_name, color_image, egui::TextureOptions::LINEAR);
        self.texture_map.insert(path.to_path_buf(), img);
    }
}
