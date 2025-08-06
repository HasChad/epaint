use egui_macroquad::egui::{self, Color32, Slider};

pub struct PolySettings {
    visible: bool,
    sides: u32,
    rotation: f32,
}

impl PolySettings {
    pub fn new() -> Self {
        Self {
            visible: true,
            sides: 6,
            rotation: 0.0,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Polygon Settings")
            .open(&mut self.visible)
            .resizable(false)
            .collapsible(false)
            .frame(egui::Frame::window(&egui::Style::default()))
            .show(ctx, |ui| {
                egui::Grid::new("poly_grid")
                    .num_columns(1)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.add(
                            Slider::new(&mut self.sides, 1..=20)
                                .trailing_fill(true)
                                .step_by(0.1)
                                .text("Edge Count")
                                .text_color(Color32::WHITE),
                        );
                        ui.end_row();

                        ui.add(
                            Slider::new(&mut self.rotation, 0.0..=360.0)
                                .trailing_fill(true)
                                .step_by(0.1)
                                .text("Rotation")
                                .text_color(Color32::WHITE),
                        );
                    })
            });
    }
}
