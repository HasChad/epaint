use macroquad::prelude::*;

#[derive(Clone)]
pub struct BrushStroke {
    pub point: Vec2,
    pub color: Color,
    pub size: f32,
}

pub struct DrawState {
    pub lines: Vec<Vec<BrushStroke>>,
    pub redo_save: Vec<Vec<BrushStroke>>,
    pub current_line: Vec<BrushStroke>,
    pub brush_color: Color,
    pub brush_size: f32,
    pub bg_color: Color,
    pub can_draw: bool,
}

impl DrawState {
    pub fn new() -> Self {
        DrawState {
            lines: vec![],
            redo_save: vec![],
            current_line: vec![],
            brush_color: WHITE,
            brush_size: 3.0,
            bg_color: BLACK,
            can_draw: true,
        }
    }

    pub fn drawing(self: &mut Self, mouse_pos: Vec2) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.lines.push(vec![BrushStroke {
                point: Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                },
                color: self.brush_color,
                size: self.brush_size,
            }]);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if mouse_delta_position().x != 0.0 || mouse_delta_position().y != 0.0 {
                self.current_line.push(BrushStroke {
                    point: Vec2 {
                        x: mouse_pos.x,
                        y: mouse_pos.y,
                    },
                    color: self.brush_color,
                    size: self.brush_size,
                });
            }

            if self.lines.len() > 0 {
                let last_idx = self.lines.len() - 1;
                self.lines[last_idx] = self.current_line.clone();
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.current_line = vec![];
        }
    }

    pub fn undo(self: &mut Self) {
        if self.lines.len() > 0 {
            self.redo_save
                .push(self.lines[self.lines.len() - 1].clone());

            self.lines.pop();
        }
    }

    pub fn redo(self: &mut Self) {
        if self.redo_save.len() > 0 {
            self.lines
                .push(self.redo_save[self.redo_save.len() - 1].clone());

            self.redo_save.pop();
        }
    }

    pub fn clear_canvas(self: &mut Self) {
        self.lines.clear();
        self.redo_save.clear();
    }

    pub fn line_render(self: &Self) {
        for line in self.lines.iter() {
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
    }
}
