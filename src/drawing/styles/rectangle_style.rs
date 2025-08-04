use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::drawing::{DrawState, Drawable, RectStyle, lyon_ops::*};

impl Drawable for RectStyle {
    fn drawing(&self, mouse_pos: Vec2, state: &mut DrawState) {
        if is_mouse_button_pressed(MouseButton::Left) {
            state.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });

            state.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });

            state.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });

            state.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });
        };

        if is_mouse_button_down(MouseButton::Left) {
            state.current_line[1] = Vec2 {
                x: state.current_line[0].x,
                y: mouse_pos.y,
            };

            state.current_line[2] = Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            };
            state.current_line[3] = Vec2 {
                x: mouse_pos.x,
                y: state.current_line[0].y,
            };
        }

        if is_mouse_button_released(MouseButton::Left) {
            state.meshing();

            state.current_line.clear();
        }
    }

    fn draw_preview(&self, state: &DrawState) {
        if state.current_line.len() == 4 {
            let mut builder = Path::builder();

            builder.begin(point(state.current_line[0].x, state.current_line[0].y));
            builder.line_to(point(state.current_line[1].x, state.current_line[1].y));
            builder.line_to(point(state.current_line[2].x, state.current_line[2].y));
            builder.line_to(point(state.current_line[3].x, state.current_line[3].y));
            builder.end(true);

            let path = builder.build();

            let lops = LyonOpsFill::new(&path, state.brush_color);

            let mesh = Mesh {
                vertices: lops.vertices,
                indices: lops.geometry.indices,
                texture: None,
            };

            draw_mesh(&mesh);
        }
    }

    fn mesh(&self, state: &mut DrawState) {
        state.lines.push(vec![]);

        let mut builder = Path::builder();

        builder.begin(point(state.current_line[0].x, state.current_line[0].y));
        builder.line_to(point(state.current_line[1].x, state.current_line[1].y));
        builder.line_to(point(state.current_line[2].x, state.current_line[2].y));
        builder.line_to(point(state.current_line[3].x, state.current_line[3].y));
        builder.end(true);

        let path = builder.build();

        let lops = LyonOpsFill::new(&path, state.brush_color);

        let mesh = Mesh {
            vertices: lops.vertices,
            indices: lops.geometry.indices,
            texture: None,
        };

        let last = state.lines.len() - 1;
        state.lines[last].push(mesh);
    }
}
