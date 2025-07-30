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
            brush_size: 3.0,
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

            self.lines.push(Line {
                points: self.current_line.clone(),
                color: self.brush_color,
                size: self.brush_size,
            });
        };

        if is_mouse_button_down(MouseButton::Left) {
            if mouse_delta_position().x != 0.0 || mouse_delta_position().y != 0.0 {
                self.current_line.push(Vec2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                });
            }

            let count = self.lines.len() - 1;
            self.lines[count].points = self.current_line.clone();
        }

        if is_mouse_button_released(MouseButton::Left) {
            if self.current_line.len() == 1 {
                self.lines.pop();
            }
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
            let mut builder = Path::builder();
            let mut raw_points = vec![];

            for stroke in line.points.iter() {
                raw_points.push(point(stroke.x, stroke.y));
            }

            if raw_points.len() > 2 {
                for (i, point) in raw_points.iter().enumerate() {
                    if i != raw_points.len() - 1 {
                        if i == 0 {
                            builder.begin(*point);
                        } else {
                            builder.line_to(*point);
                        }
                    } else {
                        builder.end(false);
                    }
                }
            }

            let path = builder.build();

            let lops = LyonOps::new(&path, line.color, line.size);

            // verts = 1500, indies = verts * 3 - 6
            info!("verts = {}", lops.vertices.len());
            info!("indie = {}", lops.geometry.indices.len());

            if lops.vertices.len() > 1500 {
                let verts: Vec<Vertex> = lops.vertices[0..1500].iter().cloned().collect();
                let indis: Vec<u16> = lops.geometry.indices[0..(1500 * 3 - 6)]
                    .iter()
                    .cloned()
                    .collect();

                let mesh = Mesh {
                    vertices: verts,
                    indices: indis,
                    texture: None,
                };

                draw_mesh(&mesh);
            } else {
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
