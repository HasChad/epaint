// #![windows_subsystem = "windows"]
use egui_macroquad::egui::{self, Slider};
use macroquad::prelude::*;

mod app_settings;

use app_settings::*;

#[derive(Clone)]
struct Brush {
    point: Vec2,
    color: Color,
    size: f32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = ZOOM_DEFAULT;

    let mut lines: Vec<Vec<Brush>> = vec![];
    let mut current_line: Vec<Brush> = vec![];

    let mut egui_color = egui::Color32::WHITE;
    let mut brush_color = WHITE;
    let mut brush_size = 3.0;

    loop {
        camera_fixer(&mut camera, &mut zoomer);

        let world_mpos = camera.screen_to_world(Vec2 {
            x: mouse_position().0,
            y: mouse_position().1,
        });

        if is_mouse_button_pressed(MouseButton::Left) {
            lines.push(vec![Brush {
                point: Vec2 {
                    x: world_mpos.x,
                    y: world_mpos.y,
                },
                color: brush_color,
                size: brush_size,
            }]);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if mouse_delta_position().x != 0.0 || mouse_delta_position().y != 0.0 {
                current_line.push(Brush {
                    point: Vec2 {
                        x: world_mpos.x,
                        y: world_mpos.y,
                    },
                    color: brush_color,
                    size: brush_size,
                });
            }

            if lines.len() > 0 {
                let last_idx = lines.len() - 1;
                lines[last_idx] = current_line.clone();
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            current_line = vec![];
        }

        if is_key_pressed(KeyCode::A) {
            lines.pop();
        }

        // ! draw
        clear_background(BLACK);
        set_camera(&camera);

        // MARK: UI
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                    ui.add(Slider::new(&mut brush_size, 1.0..=10.0).text("Brush Size"));

                    if ui.color_edit_button_srgba(&mut egui_color).changed() {
                        brush_color.a = egui_color.a() as f32 / 255.0;
                        brush_color.r = egui_color.r() as f32 / 255.0;
                        brush_color.g = egui_color.g() as f32 / 255.0;
                        brush_color.b = egui_color.b() as f32 / 255.0;
                    }

                    if ui.button("Undo").clicked() {
                        lines.pop();
                    }
                });
        });

        // MARK: DRAW
        draw_rectangle_lines(-250.0, -250.0, 500.0, 500.0, 2.0, YELLOW);

        // need a new function to render smooth curved lines. probably splines

        for line in lines.iter() {
            draw_multi_line(line);
        }

        egui_macroquad::draw();

        next_frame().await
    }
}

fn draw_multi_line(line: &Vec<Brush>) {
    let mut prev_point: &Vec2 = &Vec2::ZERO;
    for (i, stroke) in line.iter().enumerate() {
        if i == 0 {
            draw_circle(
                stroke.point.x,
                stroke.point.y,
                stroke.size / 2.0,
                stroke.color,
            );
            prev_point = &stroke.point;
            continue;
        }

        draw_circle(
            stroke.point.x,
            stroke.point.y,
            stroke.size / 2.0,
            stroke.color,
        );
        draw_line(
            prev_point.x,
            prev_point.y,
            stroke.point.x,
            stroke.point.y,
            stroke.size,
            stroke.color,
        );
        prev_point = &stroke.point;
    }
}
