mod frontend;

use frontend::app::TexasTerm;

fn main() {
    // let native_options = eframe::NativeOptions::default();
    let viewport_builder = egui::viewport::ViewportBuilder::default().with_transparent(true);
    let native_options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };
    let texas_term = eframe::run_native("TexasTerm", native_options, Box::new(|cc| Ok(Box::new(TexasTerm::new(cc)))));
    match texas_term {
        Ok(_) => println!("we good"),
        Err(_) => println!("not good"),
    }
}
