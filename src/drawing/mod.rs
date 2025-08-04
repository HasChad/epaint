use macroquad::prelude::*;

pub mod line_smoothing;
pub mod lyon_ops;
pub mod styles;

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

pub struct BrushStyle;
pub struct LineStyle;
pub struct ArrowStyle;
pub struct RectStyle;
pub struct RectOStyle;

pub trait Drawable {
    fn drawing(&self, mouse_pos: Vec2, state: &mut DrawState);
    fn draw_preview(&self, state: &DrawState);
    fn mesh(&self, state: &mut DrawState);
}

impl DrawStyle {
    pub fn as_drawable(&self) -> Box<dyn Drawable> {
        match self {
            DrawStyle::Brush => Box::new(BrushStyle),
            DrawStyle::SBrush => Box::new(BrushStyle),
            DrawStyle::Line => Box::new(LineStyle),
            DrawStyle::Arrow => Box::new(ArrowStyle),
            DrawStyle::RectO => Box::new(RectOStyle),
            DrawStyle::Rect => Box::new(RectStyle),
            // ...
            _ => unimplemented!(),
        }
    }
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

    pub fn drawing(self: &mut Self, mouse_pos: Vec2) {
        self.style.as_drawable().drawing(mouse_pos, self);
    }

    pub fn current_line_render(self: &Self) {
        self.style.as_drawable().draw_preview(self);
    }

    fn meshing(self: &mut Self) {
        self.style.as_drawable().mesh(self);
    }

    pub fn line_render(self: &Self) {
        for lines in &self.lines {
            for mesh in lines {
                draw_mesh(&mesh);
            }
        }
    }
}
