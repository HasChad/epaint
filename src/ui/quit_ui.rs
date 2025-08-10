use egui_macroquad::egui::{self, Align2};

pub struct QuitUI {
    pub visible: bool,
    pub quit_app: bool,
}

impl QuitUI {
    pub fn new() -> Self {
        Self {
            visible: false,
            quit_app: false,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Quit?")
            .open(&mut self.visible.clone())
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Yes").clicked() {
                        self.quit_app = true
                    }

                    if ui.button("No").clicked() {
                        self.visible = false
                    }
                })
            });
    }
}
