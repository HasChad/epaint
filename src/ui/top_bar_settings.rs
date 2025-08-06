use egui_macroquad::egui::{self, Slider};
use macroquad::time::get_fps;

use crate::{drawing::DrawState, ui::ui::TOP_BAR_SIZE};

pub struct TopBarSettings {
    visible: bool,
}

impl TopBarSettings {
    pub fn new() -> Self {
        Self { visible: true }
    }

    pub fn ui(&mut self, ctx: &egui::Context, state: &mut DrawState) {
        egui::TopBottomPanel::top("menu_bar")
            .exact_height(TOP_BAR_SIZE)
            .show_separator_line(true)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.label("Brush Size:");
                    ui.add(
                        Slider::new(&mut state.brush_size, 1.0..=30.0)
                            .trailing_fill(true)
                            .step_by(0.1),
                    );

                    ui.label("Color:");

                    let mut egui_color = egui::Color32::WHITE;

                    egui_color[0] = 255.min((state.brush_color.r * 255.0) as u8);
                    egui_color[1] = 255.min((state.brush_color.g * 255.0) as u8);
                    egui_color[2] = 255.min((state.brush_color.b * 255.0) as u8);
                    egui_color[3] = 255.min((state.brush_color.a * 255.0) as u8);

                    if ui.color_edit_button_srgba(&mut egui_color).changed() {
                        state.brush_color.a = egui_color.a() as f32 / 255.0;
                        state.brush_color.r = egui_color.r() as f32 / 255.0;
                        state.brush_color.g = egui_color.g() as f32 / 255.0;
                        state.brush_color.b = egui_color.b() as f32 / 255.0;
                    }

                    ui.label("BG Color:");

                    let mut egui_color = egui::Color32::WHITE;

                    egui_color[0] = 255.min((state.bg_color.r * 255.0) as u8);
                    egui_color[1] = 255.min((state.bg_color.g * 255.0) as u8);
                    egui_color[2] = 255.min((state.bg_color.b * 255.0) as u8);
                    egui_color[3] = 255.min((state.bg_color.a * 255.0) as u8);

                    if ui.color_edit_button_srgba(&mut egui_color).changed() {
                        state.bg_color.a = egui_color.a() as f32 / 255.0;
                        state.bg_color.r = egui_color.r() as f32 / 255.0;
                        state.bg_color.g = egui_color.g() as f32 / 255.0;
                        state.bg_color.b = egui_color.b() as f32 / 255.0;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("fps: {:.0}", get_fps()));
                        ui.add_space(20.0);
                    });
                });
            });
    }
}
