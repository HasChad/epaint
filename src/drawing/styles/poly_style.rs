use lyon::geom::{Angle, Vector};
use lyon::math::point;
use lyon::path::{Path, Polygon, Winding};
use macroquad::prelude::*;

use crate::drawing::{DrawState, Drawable, PolyStyle, lyon_ops::*};

impl Drawable for PolyStyle {
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
        };

        if is_mouse_button_down(MouseButton::Left) {
            state.current_line[1] = Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            };
        }

        if is_mouse_button_released(MouseButton::Left) {
            state.meshing();

            state.current_line.clear();
        }
    }

    fn draw_preview(&self, state: &DrawState) {
        if state.current_line.len() == 2 {
            let mut builder = Path::builder();

            let p1 = state.current_line[0];
            let p2 = state.current_line[1];

            let center = (p1 + p2) * 0.5;

            let sides = 6;
            let rot = 0.0;

            let radius = p2.distance(p1) * 0.5;
            let mut points = vec![];

            for i in 0..=sides {
                let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
                let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();

                let poly_p = point(center.x + radius * rx, center.y + radius * ry);

                points.push(poly_p);
            }

            builder.add_polygon(Polygon {
                points: &points,
                closed: true,
            });

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

        let p1 = state.current_line[0];
        let p2 = state.current_line[1];

        let center = (p1 + p2) * 0.5;

        builder.add_ellipse(
            point(center.x, center.y),
            Vector::new((p2.x - p1.x) / 2.0, (p2.y - p1.y) / 2.0),
            Angle::zero(),
            Winding::Positive,
        );

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
