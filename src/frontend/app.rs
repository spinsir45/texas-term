

pub struct TexasTerm {
    transparency: f32,
}

impl TexasTerm {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let app: TexasTerm = Self::default();
        app
    }
}

impl Default for TexasTerm {
    fn default() -> Self {
        TexasTerm {
            transparency: 1.0,
        }
    }
}

impl eframe::App for TexasTerm {

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::from_black_alpha(self.transparency).to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            ui.label("Hello, world!");
            let button = ui.button("push me");
            if button.clicked() {
                self.transparency = 0.0;
            }
        });
    }
}
