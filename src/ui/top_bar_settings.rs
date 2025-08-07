use egui_macroquad::egui::{self, Slider};
use macroquad::time::get_fps;

use crate::{drawing::DrawState, ui::ui::TOP_BAR_SIZE};

pub struct TopBarSettings;

impl TopBarSettings {
    pub fn new() -> Self {
        Self
    }

    pub fn ui(&mut self, ctx: &egui::Context, state: &mut DrawState) {
        egui::TopBottomPanel::top("menu_bar")
            .exact_height(TOP_BAR_SIZE)
            .show_separator_line(true)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {});
            });
    }
}
