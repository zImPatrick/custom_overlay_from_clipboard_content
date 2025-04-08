#![windows_subsystem = "windows"]

use std::{fs, sync::{atomic::{self, AtomicBool}, Arc, Mutex}, time::Duration};
use arboard::Clipboard;
use eframe::egui::{self, Color32, Context, Label, Widget as _};
use inputbot::KeybdKey::*;
use str_distance::*;

struct ClipboardKeyValueDisplay {
    pairs: Vec<(String, String)>,
    key: String,
    value: String,
    shown: Arc<AtomicBool>,
    last_updated_key: String,
    clipboard: Clipboard
}

impl Default for ClipboardKeyValueDisplay {
    fn default() -> Self {
        Self {
            pairs: Vec::new(),
            key: String::new(),
            value: "Fortnite".to_string(),
            shown: Arc::new(AtomicBool::new(false)),
            last_updated_key: String::new(),
            clipboard: Clipboard::new().unwrap()
        }
    }
}

#[cfg(target_os = "linux")]
fn get_selected_or_clipboard_text(clipboard: &mut Clipboard) -> Result<String, arboard::Error> {
    use arboard::GetExtLinux;

    return clipboard.get().clipboard(arboard::LinuxClipboardKind::Primary).text();
}

#[cfg(not(target_os = "linux"))]
fn get_selected_or_clipboard_text(clipboard: &mut Clipboard) -> Result<String, arboard::Error> {
    return clipboard.get_text()
}

const RERENDER_DURATION: Duration = Duration::from_secs(1);
impl eframe::App for ClipboardKeyValueDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // workaround https://github.com/emilk/egui/issues/2537
        ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
        if !ctx.has_requested_repaint() {
            ctx.request_repaint_after(RERENDER_DURATION);
        }

        self.key = get_selected_or_clipboard_text(&mut self.clipboard).unwrap_or_else(|_| self.key.clone());
        
        if !self.shown.load(atomic::Ordering::Relaxed) {
            self.value = String::new();
            self.last_updated_key = String::new();
        } else if self.last_updated_key != self.key {
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
            self.last_updated_key = self.key.clone();
        }

        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                Label::new(egui::RichText::new(self.value.clone()).heading().color(Color32::from_white_alpha(5))).wrap().ui(ui)
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
            .with_maximized(true),
        ..Default::default()
    };

    let shown = Arc::new(AtomicBool::new(false));
    
    let clipboard_object = ClipboardKeyValueDisplay {
        pairs,
        key: String::new(),
        value: String::new(),
        shown: shown.clone(),
        last_updated_key: String::new(),
        clipboard: Clipboard::new().unwrap()
    };

    // I'm so sorry for this
    let ctx_mutex = Arc::new(Mutex::new(None::<Context>));
    let cloned_ctx_mutex = ctx_mutex.clone();
    
    FKey.bind(move || {
        let current_state = shown.load(atomic::Ordering::Relaxed);
        shown.store(!current_state, atomic::Ordering::Relaxed);

        let guard = match ctx_mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return
        };

        match *guard {
            Some(ref ctx) => ctx.request_repaint(),
            None => ()
        }
    });

    std::thread::spawn(inputbot::handle_input_events);

    eframe::run_native(
        "Clipboard Key-Value Display",
        options,
        Box::new(|_cc| {
            cloned_ctx_mutex.lock().unwrap().get_or_insert(_cc.egui_ctx.clone());
            Ok(Box::new(clipboard_object))
        }),
    ).unwrap();
}
