use eframe::egui;
use crate::agent_manager::Agent;

#[derive(Default)]
pub struct SpaceConsole {}

impl SpaceConsole {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for SpaceConsole {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let mut agent = Agent::new();
        // let _ = agent.login_agent().await;
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("Hello World!");
        // });
   }
} 