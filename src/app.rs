use egui::{Color32, ScrollArea};
use log::error;
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::MAX_FILE_SIZE;

pub struct MyApp {
    text: String,

    text_sender: Sender<String>,
    text_receiver: Receiver<String>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, text: Option<String>) -> Self {
        let channel = channel(1);

        Self {
            text: text.unwrap_or("\u{200B}Hel\u{200B}lo \u{200B}\u{200B}World 👍".to_owned()),
            text_sender: channel.0,
            text_receiver: channel.1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(text) = self.text_receiver.try_recv() {
            self.text = text;
        }

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
                            let text_sender = self.text_sender.clone();
                            crate::spawn_task!(async move {
                                let file = rfd::AsyncFileDialog::new()
                                    .set_directory("~")
                                    .pick_file()
                                    .await;
                                let Some(file) = file else {
                                    return;
                                };
                                let content = file.read().await;
                                if content.len() > MAX_FILE_SIZE {
                                    error!(
                                        "File '{}' is to big ({} bytes). Max is 1KiB.",
                                        file.file_name(),
                                        content.len()
                                    )
                                } else if let Err(err) = text_sender
                                    .send(String::from_utf8_lossy(&content).to_string())
                                    .await
                                {
                                    error!("Error sending text: {err}");
                                }
                            });
                        }
                    });
                    egui::TextEdit::multiline(&mut self.text).show(ui)
                })
                .inner;

            let cursor_range = output.cursor_range.map(|c| c.as_ccursor_range().sorted());

            ScrollArea::new([false, true]).show(ui, |ui| {
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
                                unicode_names2::name(char)
                                    .map_or("NONAME".to_owned(), |name| name.to_string()),
                            );

                        if !char.is_ascii() {
                            label.highlight();
                        }
                    }
                });
            });
        });
    }
}
