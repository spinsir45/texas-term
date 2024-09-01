use eframe::Storage;

pub struct TexasTerm {
    transparency: f32,
    text_size: f32,
}

impl TexasTerm {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.

        // Load values
        let text_size: Option<f32> = eframe::get_value(_cc.storage.unwrap(), "text_size");
        let transparency: Option<f32> = eframe::get_value(_cc.storage.unwrap(), "transparency");

        // Update values
        let mut app: TexasTerm = Self::default();
        if let Some(num) = text_size {app.text_size = num;}
        if let Some(num) = transparency {app.transparency = num;}
        app
    }
}

impl Default for TexasTerm {
    fn default() -> Self {
        TexasTerm {
            transparency: 1.0,
            text_size: 1.0,
        }
    }
}

impl eframe::App for TexasTerm {

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::from_black_alpha(self.transparency).to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Change font size
        ctx.set_pixels_per_point(self.text_size);

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            ui.label("Hello, world!");
            let button = ui.button("push me");
            if button.clicked() {
                self.transparency = 0.0;
            }
        });

        egui::TopBottomPanel::bottom("tab_area").show(ctx, |ui| {
            let settings_btn = ui.button("settings");
            if settings_btn.clicked() {
                println!("opening settings");
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        eframe::set_value(_storage, "text_size", &self.text_size);
        eframe::set_value(_storage, "transparency", &self.transparency);
    }
}
