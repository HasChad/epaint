use crate::{
    drawing::DrawState,
    ui::{draw_settings::DrawSettings, style_settings::StyleSettings},
};

pub const TOP_BAR_SIZE: f32 = 25.0;

pub struct UI {
    draw_settings: DrawSettings,
    style_settings: StyleSettings,
}

impl UI {
    pub fn new() -> Self {
        let draw_settings = DrawSettings::new();
        let style_settings = StyleSettings::new();

        Self {
            draw_settings,
            style_settings,
        }
    }

    pub fn render_ui(self: &mut Self, draw_state: &mut DrawState) {
        egui_macroquad::ui(|ctx| {
            draw_state.can_draw = !ctx.wants_pointer_input();

            self.style_settings.ui(ctx, draw_state);
            self.draw_settings.ui(ctx, draw_state);
        })
    }
}
