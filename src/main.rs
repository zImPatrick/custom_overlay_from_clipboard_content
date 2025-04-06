use std::fs;
use eframe::egui;

struct ClipboardKeyValueDisplay {
    key: String,
    value: String,
}

impl Default for ClipboardKeyValueDisplay {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: "Fortnite".to_string(),
        }
    }
}

impl eframe::App for ClipboardKeyValueDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            ui.label(self.value.clone());
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

fn main() -> eframe::Result<()> {
    println!("Hello, world!");

    let contents = fs::read_to_string("data.txt")
        .expect("Something went wrong data.txt");
    let mut lines = contents.lines();

    println!("First line:{}\n", lines.next().unwrap_or_default());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_decorations(false)
            .with_drag_and_drop(false),
        ..Default::default()
    };

    eframe::run_native(
        "Clipboard Key-Value Display",
        options,
        Box::new(|_cc| {
            Ok(Box::<ClipboardKeyValueDisplay>::default())
        }),
    )
}
