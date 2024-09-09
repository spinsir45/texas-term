use super::app::TexasTerm;

impl TexasTerm {
    pub fn settings_window(&mut self, ctx: &egui::Context) {
        if self.open_settings_window {
            egui::Window::new("Settings")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
            .collapsible(false)
            .constrain(false)
            .open(&mut self.open_settings_window)
            .movable(false)
            .show(ctx, |ui| {
                // Window Transparency
                ui.label("Window Transparency:");
                let transparency_slider = egui::Slider::new(&mut self.transparency, 0.0..=1.0);
                ui.add(transparency_slider);
                ui.add_space(10.0);

                // Text Size
                ui.label("Text Size");
                let text_size_slider = egui::Slider::new(&mut self.temp_text_size, 0.1..=3.0);
                let text_size_slider = ui.add(text_size_slider);
                if text_size_slider.drag_stopped() {
                    self.text_size = self.temp_text_size;
                }
            });
        }
    }
}
