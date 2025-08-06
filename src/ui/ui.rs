use crate::{
    drawing::{DrawState, DrawStyle},
    ui::{
        poly_settings::PolySettings, style_settings::StyleSettings,
        top_bar_settings::TopBarSettings,
    },
};

pub const TOP_BAR_SIZE: f32 = 25.0;

pub struct UI {
    top_bar_settings: TopBarSettings,
    poly_settings: PolySettings,
    style_settings: StyleSettings,
}

impl UI {
    pub fn new() -> Self {
        let top_bar_settings = TopBarSettings::new();
        let poly_settings = PolySettings::new();
        let style_settings = StyleSettings::new();

        Self {
            top_bar_settings,
            poly_settings,
            style_settings,
        }
    }

    pub fn render_ui(self: &mut Self, draw_state: &mut DrawState) {
        egui_macroquad::ui(|ctx| {
            draw_state.can_draw = !ctx.wants_pointer_input();

            self.top_bar_settings.ui(ctx, draw_state);
            self.style_settings.ui(ctx, draw_state);

            if draw_state.style == DrawStyle::Poly || draw_state.style == DrawStyle::PolyO {
                self.poly_settings.ui(ctx);
            }
        })
    }
}
