mod file_manager;
mod memory_leak_test;
mod picture_list;
mod picutre_handler;
mod texture_handler;

use eframe::egui;

use crate::{
    file_manager::get_picture_list_for, picutre_handler::PictureHandler,
    texture_handler::TextureHandler,
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
    path_field: String,
    current_scene: Scene,
    texture_manager: TextureHandler,
    picture_handler: PictureHandler,
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
        self.picture_handler
            .init(get_picture_list_for(self.path_field.as_str()));
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
            self.update_image(ui);
            ui.horizontal_centered(|ui| {
                ui.label("pictures left to purge: ");
                ui.label(self.picture_handler.images_left());
                if ui.button("DELETE").clicked() {
                    self.picture_handler.delete();
                }
                if ui.button("SAVE").clicked() {
                    self.picture_handler.save();
                }
                if ui.button("REVERT").clicked() {
                    self.picture_handler.revert_last_action();
                }
            });
        });
    }

    fn update_image(&mut self, ui: &mut egui::Ui) {
        if let Some(texture) = self.texture_manager.get(self.picture_handler.get_next()) {
            ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(600.0, 600.0)));
        } else {
            ui.vertical_centered(|ui| {
                ui.spinner();
                ui.label("Loading images...");
            });
            return;
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
