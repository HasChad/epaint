// #![windows_subsystem = "windows"]
use egui_macroquad::egui::{self, Slider};
use macroquad::prelude::*;

mod app_settings;
mod drawing;

use app_settings::*;
use drawing::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = ZOOM_DEFAULT;

    let mut draw_state = DrawState::new();

    loop {
        camera_fixer(&mut camera, &mut zoomer);

        let world_mpos = camera.screen_to_world(Vec2 {
            x: mouse_position().0,
            y: mouse_position().1,
        });

        if is_mouse_button_pressed(MouseButton::Left) {
            draw_state.lines.push(vec![BrushStroke {
                point: Vec2 {
                    x: world_mpos.x,
                    y: world_mpos.y,
                },
                color: draw_state.brush_color,
                size: draw_state.brush_size,
            }]);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if mouse_delta_position().x != 0.0 || mouse_delta_position().y != 0.0 {
                draw_state.current_line.push(BrushStroke {
                    point: Vec2 {
                        x: world_mpos.x,
                        y: world_mpos.y,
                    },
                    color: draw_state.brush_color,
                    size: draw_state.brush_size,
                });
            }

            if draw_state.lines.len() > 0 {
                let last_idx = draw_state.lines.len() - 1;
                draw_state.lines[last_idx] = draw_state.current_line.clone();
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            draw_state.current_line = vec![];
        }

        if is_key_pressed(KeyCode::A) {
            draw_state.lines.pop();
        }

        // ! draw
        clear_background(BLACK);
        set_camera(&camera);

        // MARK: UI
        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("menu_bar")
                .exact_height(25.0)
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

                        if ui.button("Undo").clicked() {
                            draw_state.lines.pop();
                        }

                        if ui.button("Clear").clicked() {
                            draw_state.lines.clear();
                        }

                        ui.add_space(50.0);

                        ui.label(&format!("fps: {}", get_fps()))
                    });
                });
        });

        // MARK: DRAW
        draw_rectangle_lines(-250.0, -250.0, 500.0, 500.0, 5.0, YELLOW);

        // FIXME: need a new function to render smooth curved lines. probably splines
        draw_state.line_render();

        egui_macroquad::draw();

        next_frame().await
    }
}
