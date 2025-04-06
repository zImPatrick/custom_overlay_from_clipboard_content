use std::{default, fs};
use std::thread::spawn;
use eframe::egui;
use cli_clipboard::get_contents;
use inputbot::KeybdKey::*;

struct ClipboardKeyValueDisplay {
    pairs: Vec<(String, String)>,
    key: String,
    value: String,
}

impl Default for ClipboardKeyValueDisplay {
    fn default() -> Self {
        Self {
            pairs: Vec::new(),
            key: String::new(),
            value: "Fortnite".to_string(),
        }
    }
}

impl eframe::App for ClipboardKeyValueDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.key = cli_clipboard::get_contents().unwrap();

        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            ui.label(self.value.clone());
        });
    }

    // fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    //     egui::Rgba::TRANSPARENT.to_array()
    // }
}

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("data.txt")
        .expect("Something went wrong data.txt");
    let mut lines = contents.lines();

    println!("First line:{}\n", lines.next().unwrap_or_default());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_decorations(false)
            .with_drag_and_drop(false)
            .with_always_on_top()
            .with_mouse_passthrough(true),
        ..Default::default()
    };

    let clipboard_object = ClipboardKeyValueDisplay {
        pairs: Vec::new(),
        key: String::new(),
        value: String::new(),
    };
    
    FKey.bind(move || {
        println!("FKey pressed");
    });

    std::thread::spawn(inputbot::handle_input_events);

    let _ = eframe::run_native(
        "Clipboard Key-Value Display",
        options,
        Box::new(|_cc| {
            Ok(Box::new(clipboard_object))
        }),
    );
}
