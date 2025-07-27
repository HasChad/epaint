use macroquad::prelude::*;

#[derive(Clone)]
pub struct BrushStroke {
    pub point: Vec2,
    pub color: Color,
    pub size: f32,
}

pub struct DrawState {
    pub lines: Vec<Vec<BrushStroke>>,
    pub current_line: Vec<BrushStroke>,
    pub brush_color: Color,
    pub brush_size: f32,
    pub bg_color: Color,
}

impl DrawState {
    pub fn new() -> Self {
        DrawState {
            lines: vec![],
            current_line: vec![],
            brush_color: WHITE,
            brush_size: 3.0,
            bg_color: BLACK,
        }
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
