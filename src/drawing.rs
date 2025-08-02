use lyon::math::point;
use lyon::path::{LineCap, LineJoin, Path};
use lyon::tessellation::{
    BuffersBuilder, StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers,
};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Line {
    pub points: Vec<Vec2>,
    pub color: Color,
    pub size: f32,
}

pub struct DrawState {
    pub lines: Vec<Line>,
    pub redo_save: Vec<Line>,
    pub current_line: Vec<Vec2>,
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
            brush_size: 5.0,
            bg_color: BLACK,
            can_draw: true,
        }
    }

    pub fn drawing(self: &mut Self, mouse_pos: Vec2) {
        if is_mouse_button_pressed(MouseButton::Left) {
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
            if self.current_line.len() == 1 {
                self.lines.pop();
            }
            self.line_smoothing();

            self.lines.push(Line {
                points: self.current_line.clone(),
                color: self.brush_color,
                size: self.brush_size,
            });

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

    fn line_smoothing(self: &mut Self) {
        let raw_points: Vec<Vec2> = self.current_line.clone();
        let filtered = remove_nearby_points(&raw_points, 2.0);
        let filtered2 = remove_colinear_points(&filtered, 0.05); // ~3 degrees
        let final_points = smooth_points(&filtered2, 0.05, 3);

        info!("final = {}", final_points.len());

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

            if raw_points.len() > 2 {
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
            }

            let path = builder.build();

            let lops = LyonOps::new(&path, self.brush_color, self.brush_size);

            let mesh = Mesh {
                vertices: lops.vertices,
                indices: lops.geometry.indices,
                texture: None,
            };

            draw_mesh(&mesh);
        }
    }

    pub fn line_render(self: &Self) {
        for line in self.lines.iter() {
            let mut prev_last: Option<&Vec2> = None;

            for line_chunk in line.points.chunks(350) {
                let mut builder = Path::builder();
                let mut raw_points = vec![];

                if let Some(prev) = prev_last {
                    raw_points.push(point(prev.x, prev.y));
                }

                for stroke in line_chunk.iter() {
                    raw_points.push(point(stroke.x, stroke.y));
                }

                prev_last = line_chunk.last();

                if raw_points.len() > 2 {
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
                }

                let path = builder.build();

                let lops = LyonOps::new(&path, line.color, line.size);

                let mesh = Mesh {
                    vertices: lops.vertices,
                    indices: lops.geometry.indices,
                    texture: None,
                };

                draw_mesh(&mesh);
            }
        }
    }
}

struct LyonOps {
    geometry: VertexBuffers<[f32; 2], u16>,
    vertices: Vec<Vertex>,
}

impl LyonOps {
    fn new(path: &Path, color: Color, width: f32) -> Self {
        // Tessellate into triangles
        let mut geometry: VertexBuffers<[f32; 2], u16> = VertexBuffers::new();
        let mut tessellator = StrokeTessellator::new();

        tessellator
            .tessellate_path(
                path,
                &StrokeOptions::default()
                    .with_line_width(width)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
                &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
                    vertex.position().to_array()
                }),
            )
            .unwrap();

        // Convert into Macroquad Mesh
        let vertices: Vec<Vertex> = geometry
            .vertices
            .iter()
            .map(|[x, y]| Vertex {
                position: Vec3::new(*x, *y, 0.0),
                uv: Vec2::ZERO,
                color: color.into(),
                normal: Vec4::ZERO,
            })
            .collect();

        LyonOps { geometry, vertices }
    }
}

fn remove_nearby_points(points: &Vec<Vec2>, min_distance: f32) -> Vec<Vec2> {
    let mut cleaned = Vec::new();

    for i in 0..points.len() {
        let p: Vec2 = points[i];

        if cleaned
            .last()
            .map_or(true, |last: &Vec2| last.distance(p) >= min_distance)
        {
            cleaned.push(p);
        }
    }

    cleaned
}

fn is_colinear(a: Vec2, b: Vec2, c: Vec2, tolerance: f32) -> bool {
    let ab = b - a;
    let bc = c - b;
    let angle = ab.angle_between(bc).abs();
    angle < tolerance
}

fn remove_colinear_points(points: &Vec<Vec2>, angle_tolerance: f32) -> Vec<Vec2> {
    if points.len() < 3 {
        return points.clone();
    }

    let mut cleaned = vec![points[0]];
    for i in 1..points.len() - 1 {
        let prev = cleaned.last().unwrap();
        let curr = points[i];
        let next = points[i + 1];

        if !is_colinear(*prev, curr, next, angle_tolerance) {
            cleaned.push(curr);
        }
    }
    cleaned.push(*points.last().unwrap());
    cleaned
}

fn smooth_points(points: &[Vec2], strength: f32, iterations: usize) -> Vec<Vec2> {
    let mut result = points.to_vec();

    for _ in 0..iterations {
        let mut new_points = result.clone();
        for i in 1..result.len() - 1 {
            let prev = result[i - 1];
            let curr = result[i];
            let next = result[i + 1];

            // Average neighbors and move current point slightly toward the average
            let target = (prev + next) * 0.5;
            new_points[i] = curr.lerp(target, strength);
        }
        result = new_points;
    }

    result
}
