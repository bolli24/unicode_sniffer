use egui::Color32;
use log::error;

use crate::get_extended_unicode_name;

pub struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "Hello World!".to_owned(),
        }
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let output = ui
                .vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Input: ");
                        if ui.button("Load file").clicked() {
                            if let Some(file) =
                                rfd::FileDialog::new().set_directory("~").pick_file()
                            {
                                let result = std::fs::read(&file);
                                match result {
                                    Ok(content) => {
                                        self.text = String::from_utf8_lossy(&content).to_string();
                                    }
                                    Err(err) => {
                                        error!("Error reading file '{}': {}", file.display(), err)
                                    }
                                }
                            }
                        }
                    });
                    egui::TextEdit::multiline(&mut self.text).show(ui)
                })
                .inner;

            let cursor_range = output.cursor_range.map(|c| c.as_ccursor_range().sorted());

            ui.horizontal_wrapped(|ui| {
                for (i, char) in self.text.chars().enumerate() {
                    let color = cursor_range
                        .and_then(|range| {
                            if range[0].index <= i && range[1].index > i {
                                Some(Color32::ORANGE)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(ui.ctx().style().visuals.text_color());
                    let label = ui
                        .colored_label(color, format!("{:X}", char as u32))
                        .on_hover_text(
                            get_extended_unicode_name(char)
                                .map_or("NONAME".to_owned(), |name| name.to_string()),
                        );

                    if !char.is_ascii() {
                        label.highlight();
                    }
                }
            });

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
