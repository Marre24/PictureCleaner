use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use eframe::egui::{self, ColorImage, TextureHandle};
use image::GenericImageView;

use crate::picture_list::PictureList;

#[derive(Default)]
pub(crate) struct TextureHandler {
    texture_map: HashMap<PathBuf, TextureHandle>,
    pending: Arc<Mutex<Vec<(PathBuf, String, ColorImage)>>>,
}

impl TextureHandler {
    const MAX_SIZE: u32 = 200;
    const NUM_THREADS: usize = 2;

    pub(crate) fn get(&mut self, path: &Path, ctx: &egui::Context) -> Option<&TextureHandle> {
        let mut pending = self.pending.lock().unwrap();
        for (path, name, color_image) in pending.drain(..) {
            let img = ctx.load_texture(&name, color_image, egui::TextureOptions::LINEAR);
            self.texture_map.insert(path, img);
        }
        drop(pending);

        self.texture_map.get(path)
    }

    pub(crate) fn loaded_images(&self) -> String {
        (self.pending.lock().unwrap().len() + self.texture_map.len()).to_string()
    }

    pub(crate) fn init(&mut self, unchecked_pics: PictureList) {
        let pending = Arc::clone(&self.pending);
        let paths: Vec<PathBuf> = unchecked_pics.path_iterator().cloned().collect();

        let chunk_size = (paths.len() + Self::NUM_THREADS - 1) / Self::NUM_THREADS;

        pending.lock().unwrap().reserve(paths.len());

        for chunk in paths.chunks(chunk_size) {
            let pending = Arc::clone(&pending);
            let chunk = chunk.to_vec();

            thread::spawn(move || {
                for path in chunk {
                    Self::load_texture(&path, &pending);
                }
            });
        }
    }

    fn load_texture(path: &Path, pending: &Arc<Mutex<Vec<(PathBuf, String, ColorImage)>>>) {
        use image::DynamicImage;

        let image_bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Could not get image bytes for {:?}: {}", path, e);
                return;
            }
        };

        let image = match image::load_from_memory(&image_bytes) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Could not load image {:?}: {}", path, e);
                return;
            }
        };

        let (width, height) = image.dimensions();
        let (new_width, new_height) = if width > height {
            let ratio = Self::MAX_SIZE as f32 / width as f32;
            (Self::MAX_SIZE, (height as f32 * ratio) as u32)
        } else {
            let ratio = Self::MAX_SIZE as f32 / height as f32;
            ((width as f32 * ratio) as u32, Self::MAX_SIZE)
        };

        let resized = image.resize(new_width, new_height, image::imageops::FilterType::Triangle);

        let size = [resized.width() as usize, resized.height() as usize];

        let image_buffer = match resized {
            DynamicImage::ImageRgba8(buf) => buf,
            _ => resized.to_rgba8(),
        };

        let color_image = ColorImage::from_rgba_unmultiplied(size, image_buffer.as_raw());

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => {
                eprintln!("Could not find file name for {:?}", path);
                return;
            }
        };

        pending
            .lock()
            .unwrap()
            .push((path.to_path_buf(), file_name, color_image));
    }
}
