use egui_macroquad::egui::{
    self, Color32, FontId, RichText, Slider, TextFormat, WidgetText, text::LayoutJob,
};
use macroquad::time::get_fps;

use crate::drawing::DrawState;

pub const TOP_BAR_SIZE: f32 = 25.0;

pub fn render_ui(draw_state: &mut DrawState) {
    egui_macroquad::ui(|egui_ctx| {
        draw_state.can_draw = !egui_ctx.wants_pointer_input();

        egui::TopBottomPanel::top("menu_bar")
            .exact_height(TOP_BAR_SIZE)
            .show_separator_line(true)
            .show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.label("Brush Size:");
                    ui.add(
                        Slider::new(&mut draw_state.brush_size, 1.0..=10.0)
                            .trailing_fill(true)
                            .step_by(0.1),
                    );

                    ui.label("Color:");

                    let mut egui_color = egui::Color32::WHITE;

                    egui_color[0] = 255.min((draw_state.brush_color.r * 255.0) as u8);
                    egui_color[1] = 255.min((draw_state.brush_color.g * 255.0) as u8);
                    egui_color[2] = 255.min((draw_state.brush_color.b * 255.0) as u8);
                    egui_color[3] = 255.min((draw_state.brush_color.a * 255.0) as u8);

                    if ui.color_edit_button_srgba(&mut egui_color).changed() {
                        draw_state.brush_color.a = egui_color.a() as f32 / 255.0;
                        draw_state.brush_color.r = egui_color.r() as f32 / 255.0;
                        draw_state.brush_color.g = egui_color.g() as f32 / 255.0;
                        draw_state.brush_color.b = egui_color.b() as f32 / 255.0;
                    }

                    ui.label("BG Color:");

                    let mut egui_color = egui::Color32::WHITE;

                    egui_color[0] = 255.min((draw_state.bg_color.r * 255.0) as u8);
                    egui_color[1] = 255.min((draw_state.bg_color.g * 255.0) as u8);
                    egui_color[2] = 255.min((draw_state.bg_color.b * 255.0) as u8);
                    egui_color[3] = 255.min((draw_state.bg_color.a * 255.0) as u8);

                    if ui.color_edit_button_srgba(&mut egui_color).changed() {
                        draw_state.bg_color.a = egui_color.a() as f32 / 255.0;
                        draw_state.bg_color.r = egui_color.r() as f32 / 255.0;
                        draw_state.bg_color.g = egui_color.g() as f32 / 255.0;
                        draw_state.bg_color.b = egui_color.b() as f32 / 255.0;
                    }

                    ui.menu_button("Options", |ui| {
                        let mut job = LayoutJob::default();

                        job.append(
                            "Undo",
                            0.0,
                            TextFormat {
                                color: Color32::WHITE,
                                ..Default::default()
                            },
                        );

                        job.append(
                            "Ctrl+Z",
                            30.0,
                            TextFormat {
                                color: Color32::GRAY,
                                ..Default::default()
                            },
                        );

                        let test_label: WidgetText = WidgetText::from(job);
                        if ui.button(test_label).clicked() {
                            draw_state.undo();
                        }

                        let mut job = LayoutJob::default();

                        job.append(
                            "Redo",
                            0.0,
                            TextFormat {
                                color: Color32::WHITE,
                                ..Default::default()
                            },
                        );

                        job.append(
                            "Ctrl+X",
                            30.0,
                            TextFormat {
                                color: Color32::GRAY,
                                ..Default::default()
                            },
                        );

                        let test_label: WidgetText = WidgetText::from(job);
                        if ui.button(test_label).clicked() {
                            draw_state.redo();
                        }

                        let mut job = LayoutJob::default();

                        job.append(
                            "Clear",
                            0.0,
                            TextFormat {
                                color: Color32::WHITE,
                                ..Default::default()
                            },
                        );

                        job.append(
                            "C",
                            30.0,
                            TextFormat {
                                color: Color32::GRAY,
                                ..Default::default()
                            },
                        );

                        let test_label: WidgetText = WidgetText::from(job);
                        if ui.button(test_label).clicked() {
                            draw_state.clear_canvas();
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("fps: {:.0}", get_fps()));
                        ui.add_space(20.0);
                    });
                });
            });
    });
}

fn menu_button(ui: &mut egui::Ui, label: &str, shortcut: &str) -> LayoutJob {
    let mut job = menu_button(ui, "Undo", "Ctrl+Z");

    job.append(
        label,
        10.0,
        TextFormat {
            font_id: FontId::proportional(16.0),
            color: Color32::WHITE,
            ..Default::default()
        },
    );

    job.append(
        shortcut,
        10.0,
        TextFormat {
            font_id: FontId::proportional(16.0),
            color: Color32::WHITE,
            ..Default::default()
        },
    );

    job
}
