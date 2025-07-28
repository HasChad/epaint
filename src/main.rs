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

        if draw_state.can_draw && mouse_position().1 > TOP_BAR_SIZE {
            let world_mpos = camera.screen_to_world(Vec2 {
                x: mouse_position().0,
                y: mouse_position().1,
            });

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
        draw_rectangle_lines(-250.0, -250.0, 500.0, 500.0, 5.0, YELLOW);

        // FIXME: need a new function to render smooth curved lines. probably splines
        draw_state.line_render();

        egui_macroquad::draw();

        next_frame().await
    }
}
