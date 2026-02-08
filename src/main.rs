mod file_manager;
mod memory_leak_test;
mod picture_list;
mod texture_handler;

use std::collections::LinkedList;

use eframe::egui;

use crate::{
    file_manager::get_picture_list_for, picture_list::PictureList, texture_handler::TextureHandler,
};

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
    texture_manager: TextureHandler,
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
        Self::default()
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

    fn image_deleter_scene(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PURGE THESE PICTURES");
            self.update_image(ctx, ui);
            ui.horizontal_centered(|ui| {
                ui.label("pictures left to purge: ");
                ui.label(self.unchecked_pics.size().to_string());
                if ui.button("DELETE").clicked() {
                    self.deleted_pics.transfer_from(&mut self.unchecked_pics);
                    self.history.push_front(false);
                    self.update_image(ctx, ui);
                }
                if ui.button("SAVE").clicked() {
                    self.saved_pics.transfer_from(&mut self.unchecked_pics);
                    self.history.push_front(true);
                    self.update_image(ctx, ui);
                }
                if ui.button("REVERT").clicked() {
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

    fn update_image(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if let Some(texture) = self
            .texture_manager
            .get_image_from_path(self.unchecked_pics.peek(), ctx)
        {
            ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(600.0, 600.0)));
        } else {
            ui.label("No image to display");
        }
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
