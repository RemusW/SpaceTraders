mod agent_manager;

use eframe::egui;
use tokio::runtime::Runtime;
use agent_manager::Agent;

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
        })
    });


    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: false,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(400.0, 100.0)),
        initial_window_size: Some(egui::vec2(400.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("My egui App", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
    Ok(());
}

#[derive(Default)]
struct MyEguiApp {
    name: String,
    age: u32,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            name : "bro".to_owned(),
            age : 12,
        }
    }

    fn login(ctx: egui::Context) {
        let mut agent = agent_manager::Agent::new();

        ctx.request_repaint();
    }   
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
       });
   }
} 