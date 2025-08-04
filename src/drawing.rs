use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::line_smoothing::*;
use crate::lyon_ops::*;

#[derive(Clone, Copy)]

pub enum DrawStyle {
    Brush,
    SBrush,
    Line,
    Arrow,
    Rect,
    RectO,
    Circle,
    CircleO,
}

pub struct DrawState {
    pub style: DrawStyle,
    pub lines: Vec<Vec<Mesh>>,
    pub redo_save: Vec<Vec<Mesh>>,
    pub current_line: Vec<Vec2>,
    pub brush_color: Color,
    pub brush_size: f32,
    pub bg_color: Color,
    pub can_draw: bool,
}

impl DrawState {
    pub fn new() -> Self {
        DrawState {
            style: DrawStyle::Brush,
            lines: vec![],
            redo_save: vec![],
            current_line: vec![],
            brush_color: WHITE,
            brush_size: 5.0,
            bg_color: BLACK,
            can_draw: true,
        }
    }

    pub fn drawing(self: &mut Self, mouse_pos: Vec2) {
        if is_mouse_button_pressed(MouseButton::Left) {
            // self.current_line.push(Vec2::new(-100.0, 0.0));
            // self.current_line.push(Vec2::new(100.0, 0.0));

            self.current_line.push(Vec2 {
                x: mouse_pos.x,
                y: mouse_pos.y,
            });
        };

        if is_mouse_button_down(MouseButton::Left) {
            if mouse_delta_position().x != 0.0 || mouse_delta_position().y != 0.0 {
                self.current_line.push(Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                });
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if self.current_line.len() > 1 {
                if self.current_line.len() > 2 {
                    self.line_smoothing();
                }
                self.meshing();
            }

            self.current_line.clear();
        }
    }

    pub fn inputs(self: &mut Self) {
        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
            self.undo();
        }

        if is_key_pressed(KeyCode::X) && is_key_down(KeyCode::LeftControl) {
            self.redo();
        }

        if is_key_pressed(KeyCode::C) {
            self.clear_canvas();
        }
    }

    fn undo(self: &mut Self) {
        if let Some(line) = self.lines.pop() {
            self.redo_save.push(line);
        }
    }

    fn redo(self: &mut Self) {
        if let Some(line) = self.redo_save.pop() {
            self.lines.push(line);
        }
    }

    fn clear_canvas(self: &mut Self) {
        self.lines.clear();
        self.redo_save.clear();
    }

    fn line_smoothing(self: &mut Self) {
        let raw_points: Vec<Vec2> = self.current_line.clone();

        let filtered = remove_nearby_points(&raw_points, 2.0);
        let filtered2 = remove_colinear_points(&filtered, 0.05); // ~3 degrees
        let filtered3 = smooth_points(&filtered2, 0.2, 3);
        let final_points = remove_colinear_points(&filtered3, 0.05); // ~3 degrees

        self.current_line = final_points;
    }

    pub fn current_line_render(self: &Self) {
        let mut prev_last: Option<&Vec2> = None;

        for line_chunk in self.current_line.chunks(350) {
            let mut builder = Path::builder();
            let mut raw_points = vec![];

            if let Some(prev) = prev_last {
                raw_points.push(point(prev.x, prev.y));
            }

            for stroke in line_chunk.iter() {
                raw_points.push(point(stroke.x, stroke.y));
            }

            prev_last = line_chunk.last();

            for (i, point) in raw_points.iter().enumerate() {
                if i == 0 {
                    builder.begin(*point);
                } else {
                    builder.line_to(*point);
                }

                if i == raw_points.len() - 1 {
                    builder.end(false);
                }
            }

            let path = builder.build();

            let lops = LyonOpsLine::new(&path, self.brush_color, self.brush_size);

            let mesh = Mesh {
                vertices: lops.vertices,
                indices: lops.geometry.indices,
                texture: None,
            };

            draw_mesh(&mesh);
        }
    }

    fn meshing(self: &mut Self) {
        let mut prev_last: Option<&Vec2> = None;
        self.lines.push(vec![]);

        for line_chunk in self.current_line.chunks(350) {
            let mut builder = Path::builder();
            let mut raw_points = vec![];

            if let Some(prev) = prev_last {
                raw_points.push(point(prev.x, prev.y));
            }

            for stroke in line_chunk.iter() {
                raw_points.push(point(stroke.x, stroke.y));
            }

            prev_last = line_chunk.last();

            for (i, point) in raw_points.iter().enumerate() {
                if i == 0 {
                    builder.begin(*point);
                    continue;
                }

                builder.line_to(*point);

                if i == raw_points.len() - 1 {
                    builder.end(false);
                }
            }

            let path = builder.build();

            let lops = LyonOpsLine::new(&path, self.brush_color, self.brush_size);

            let mesh = Mesh {
                vertices: lops.vertices,
                indices: lops.geometry.indices,
                texture: None,
            };

            let last = self.lines.len() - 1;
            self.lines[last].push(mesh);
        }
    }

    pub fn line_render(self: &Self) {
        for lines in &self.lines {
            for mesh in lines {
                draw_mesh(&mesh);
            }
        }
    }
}
