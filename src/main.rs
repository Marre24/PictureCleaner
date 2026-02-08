mod file_manager;
mod memory_leak_test;
mod picture_list;

use std::{collections::LinkedList, num::NonZero, path::PathBuf};

use eframe::egui;

use crate::{file_manager::get_picture_list_for, picture_list::PictureList};

#[derive(Default)]
enum Scene {
    #[default]
    PathChecker,
    Deleter,
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    unchecked_pics: PictureList,
    saved_pics: PictureList,
    deleted_pics: PictureList,
    history: LinkedList<bool>,
    path_field: String,
    current_scene: Scene,
    texture: Option<egui::TextureHandle>,
    changed: bool,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::new_internal()
    }

    #[cfg(test)]
    pub fn new_for_testing() -> Self {
        Self::new_internal()
    }

    fn new_internal() -> Self {
        MyEguiApp {
            unchecked_pics: PictureList::default(),
            saved_pics: PictureList::default(),
            deleted_pics: PictureList::default(),
            history: LinkedList::default(),
            path_field: String::default(),
            current_scene: Scene::default(),
            texture: None,
            changed: true,
        }
    }

    fn init(&mut self) {
        self.unchecked_pics = get_picture_list_for(self.path_field.as_str());
    }

    fn path_searcher_scene(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Enter the path where you want search for pictures to clean!");
                ui.horizontal_centered(|ui| {
                    ui.label("Path: ");
                    ui.text_edit_singleline(&mut self.path_field);
                    if ui.button("Search for Images").clicked() {
                        self.init();
                        self.current_scene = Scene::Deleter;
                    }
                });
            });
        });
    }

    fn get_image_from_path(&mut self, ctx: &egui::Context) {
        if !self.changed {
            return;
        }
        self.changed = false;

        let path = self.unchecked_pics.peek();

        let image_bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Could not read image file: {}", e);
                return;
            }
        };

        let image = match image::load_from_memory(&image_bytes) {
            Ok(img) => img,
            Err(e) => {
                println!("Could not decode image: {}", e);
                return;
            }
        };

        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        let texture = ctx.load_texture("my-image", color_image, Default::default());
        self.texture = Some(texture);
    }

    fn image_deleter_scene(&mut self, ctx: &egui::Context) {
        self.get_image_from_path(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PURGE THESE PICTURES");
            if let Some(texture) = &self.texture {
                ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(600.0, 600.0)));
            } else {
                ui.label("No image to display");
            }
            ui.horizontal_centered(|ui| {
                ui.label("pictures left to purge: ");
                ui.label(self.unchecked_pics.size().to_string());
                if ui.button("DELETE").clicked() {
                    self.changed = true;
                    self.deleted_pics.transfer_from(&mut self.unchecked_pics);
                    self.history.push_front(false);
                    self.get_image_from_path(ctx);
                }
                if ui.button("SAVE").clicked() {
                    self.changed = true;
                    self.saved_pics.transfer_from(&mut self.unchecked_pics);
                    self.history.push_front(true);
                    self.get_image_from_path(ctx);
                }
                if ui.button("REVERT").clicked() {
                    self.changed = true;
                    if self.history.is_empty() {
                        println!("Empty history, cannot revert");
                        return;
                    }
                    if self.history.pop_front().unwrap() {
                        self.unchecked_pics.transfer_from(&mut self.saved_pics);
                    } else {
                        self.unchecked_pics.transfer_from(&mut self.deleted_pics);
                    }
                }
            });
        });
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.current_scene {
            Scene::PathChecker => self.path_searcher_scene(ctx),
            Scene::Deleter => self.image_deleter_scene(ctx),
        }
    }
}
