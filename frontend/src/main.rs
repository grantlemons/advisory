use eframe::{egui, run_native};

struct FrontendApp;

impl Default for FrontendApp {
    fn default() -> Self {
        Self {}
    }
}

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
        egui::TopBottomPanel::top("my_top_panel")
            .default_height(60.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Advisory Creator");
            });
        egui::SidePanel::left("my_left_panel")
            .default_width(300.0)
            .width_range(200.0..=350.0)
            .resizable(false)
            .show(ctx, |ui| ui.label("Configuration"));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Advisories");
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
