use std::{fs, sync::{atomic::{self, AtomicBool}, Arc, Mutex}};
use eframe::egui::{self, Context};
use inputbot::KeybdKey::*;
use str_distance::*;

struct ClipboardKeyValueDisplay {
    pairs: Vec<(String, String)>,
    key: String,
    value: String,
    shown: Arc<AtomicBool>
}

impl Default for ClipboardKeyValueDisplay {
    fn default() -> Self {
        Self {
            pairs: Vec::new(),
            key: String::new(),
            value: "Fortnite".to_string(),
            shown: Arc::new(AtomicBool::new(false))
        }
    }
}

impl eframe::App for ClipboardKeyValueDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.key = cli_clipboard::get_contents().unwrap_or(self.key.clone());
        
        if !self.shown.load(atomic::Ordering::Relaxed) {
            self.value = "".to_string();
        } else {
            // this is hella expensive and crashed twice while testing on a large file
            let mut min_levenshtein_distance = f64::MAX;
            let mut min_levenshtein_value = String::new();
            for (k, v) in &self.pairs {
                let distance = str_distance_normalized(&self.key, k, Levenshtein::default());
                if distance < min_levenshtein_distance {
                    min_levenshtein_distance = distance;
                    min_levenshtein_value = v.clone();
                    if distance == 0.0 {
                        break;
                    }
                }
            }
            self.value = min_levenshtein_value;
        }


        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                ui.label(self.value.clone());
            })
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("data.txt")
        .expect("Something went wrong data.txt");
    let lines = contents.lines();
    let mut pairs: Vec<(String, String)> = Vec::new();

    for line in lines {
        let penis = line.replace("\\n", "\n");
        let mut parts = penis.split(';');
        let key = parts.next().unwrap();
        let mut value = parts.fold(String::new(), |a, b| a + b + ";");
        value.pop();
        pairs.push((key.to_string(), value.to_string()));
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_decorations(false)
            .with_drag_and_drop(false)
            .with_always_on_top()
            .with_mouse_passthrough(true)
            .with_maximized(true),
        ..Default::default()
    };

    let shown = Arc::new(AtomicBool::new(false));
    
    let clipboard_object = ClipboardKeyValueDisplay {
        pairs,
        key: String::new(),
        value: String::new(),
        shown: shown.clone(),
    };

    // this has to be illegal in at least 32 countries
    let ctx_mutex: Arc<Mutex<Option<Context>>> = Arc::new(Mutex::new(None));
    let ctx_mut_clone = ctx_mutex.clone();
    FKey.bind(move || {
        let current_state = shown.load(atomic::Ordering::Relaxed);
        shown.store(!current_state, atomic::Ordering::Relaxed);

        // rerender
        let ctx_option = ctx_mutex.lock().unwrap();
        match *ctx_option {
            Some(ref ctx) => ctx.request_repaint(),
            None => return
        };
    });

    std::thread::spawn(inputbot::handle_input_events);

    let _ = eframe::run_native(
        "Clipboard Key-Value Display",
        options,
        Box::new(|_cc| {
            let mut ctx = ctx_mut_clone.lock().unwrap();

            // this cant be safe
            let _ = std::mem::replace(&mut *ctx, Some(_cc.egui_ctx.clone()));
            Ok(Box::new(clipboard_object))
        }),
    );
}
