// #![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod app_settings;
mod drawing;
mod ui;

use app_settings::*;
use drawing::*;
use ui::*;

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

        if draw_state.can_draw && mouse_position().1 > TOP_BAR_SIZE {
            draw_state.drawing(world_mpos);
        }

        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
            draw_state.undo();
        }

        if is_key_pressed(KeyCode::X) && is_key_down(KeyCode::LeftControl) {
            draw_state.redo();
        }

        if is_key_pressed(KeyCode::C) {
            draw_state.clear_canvas();
        }

        // ! draw
        clear_background(draw_state.bg_color);
        set_camera(&camera);

        // MARK: UI
        render_ui(&mut draw_state);

        // MARK: DRAW
        draw_state.line_render();
        draw_state.current_line_render();

        draw_circle(
            world_mpos.x,
            world_mpos.y,
            draw_state.brush_size / 2.0,
            draw_state.brush_color,
        );

        // MARK: DRAW UI
        egui_macroquad::draw();

        next_frame().await
    }
}
