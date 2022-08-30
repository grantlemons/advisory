use eframe::{egui, run_native};

#[derive(Default)]
struct FrontendApp;

impl FrontendApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for FrontendApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Advisory Creator");
        });
    }
}

/// Called when running natively
fn main() {
    run_native(
        "Advisory Creator",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(FrontendApp::new(cc))),
    )
}
