#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
mod save;

use eframe::{Frame};
use egui::{Context, MenuBar, OpenUrl, RichText, Ui};
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use rfd::{FileDialog, MessageDialog};
use std::fs;
use std::fs::File;
use std::io::{Error, Seek};
use std::path::PathBuf;
use crate::save::json::format_json;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    fs::create_dir_all("logs").unwrap();

    eframe::run_native(
        "gvas-viewer",
        options,
        Box::new(|_cc| Ok(Box::new(AppState::default())))
    )
}

#[derive(Default)]
pub struct Files {
    gvas_file: Option<File>,
    gvas: Option<GvasFile>,
    json: Option<String>,
    pretty_json: Option<String>
}

#[derive(Default)]
pub struct AppState {
    files: Files,
    selected: Option<String>,
    edit_buffer: Option<String>
}

impl AppState {
    fn show_menu(&mut self, ui: &mut Ui) {
        MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let dialog: FileDialog = FileDialog::new();
                    let path: Option<PathBuf> = dialog.pick_file();
                    if path.is_some() {
                        self.set_gvas_file(path.unwrap());
                        self.load_file();
                    }

                }
                if ui.button("Export").clicked() {
                    let dialog: FileDialog = FileDialog::new();
                    let path: Option<PathBuf> = dialog.save_file();
                    if path.is_some() {
                        if let Some(gvas_file) = &self.files.gvas {
                            let file = File::create(path.unwrap());
                            if let Ok(mut f) = file {
                                gvas_file.write(&mut f).expect("failed to write");
                            }
                        }
                    }
                }

                ui.separator();

                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }


            });

            ui.menu_button("Help", |ui| {
                if ui.button("Join the Discord").clicked() {
                    let url: OpenUrl = OpenUrl {
                        url: "https://discord.cal.ceo".to_owned(),
                        new_tab: true
                    };
                    ui.ctx().open_url(url)
                }
            });
        });
    }
    /// Sets gvas_file from a given path
    /// if path is invalid, show err and do nothing
    fn set_gvas_file(&mut self, path: PathBuf) {
        let file_res: Result<File, Error> = File::open(path);
        match file_res {
            Ok(file) => {
                if AppState::is_gvas_file(&file) {
                    self.files.gvas_file = Some(file)
                }
            }
            Err(e) => {
               MessageDialog::new()
                    .set_title("Error")
                    .set_description(e.to_string())
                    .show();
            }
        };
    }


    fn is_gvas_file(mut file: &File) -> bool {
        let gvas_file: Result<GvasFile, gvas::error::Error> = GvasFile::read(&mut file, GameVersion::Default);
        match gvas_file {
            Ok(_) => {
                true
            }
            Err(e) => {
                MessageDialog::new()
                    .set_title("Error")
                    .set_description(format!("{}\nIs this an Unreal Engine Save File?", e.to_string()))
                    .show();
                false
            }
        }
    }
    fn load_file(&mut self) {
        if let Some(file) = self.files.gvas_file.as_mut() {
            file.seek(std::io::SeekFrom::Start(0)).ok();
            if let Ok(save) = GvasFile::read(file, GameVersion::Default) {
                self.files.json = Some(format_json(&save).unwrap());
                self.files.pretty_json = serde_json::to_string_pretty(&self.files.json).ok();
                self.files.gvas = Some(save);
                logger::info("converted to json...");
            }
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("left-panel")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                AppState::show_menu(self, ui);

                    if let Some(file) = &self.files.gvas {
                        let properties = &file.properties;
                        egui::ScrollArea::vertical().id_salt("left").show(ui, |ui| {
                            for (key, value) in properties {
                                let text = RichText::new(key.as_str()).heading();
                                if ui.button(text).clicked() {
                                    self.selected = Some(key.to_owned());
                                    self.edit_buffer = Some(serde_json::to_string_pretty(value).unwrap());
                                }

                                ui.add_space(10.0);
                            }
                        });
                    } else {
                        logger::info("Failed to create labels");
                    }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(ui.available_width());
            if let (Some(key), Some(buf)) = (&self.selected, &mut self.edit_buffer) {
                ui.label(format!("Editing: {:?}", key));

                if ui.button("Apply changes").clicked() {
                    match serde_json::from_str(buf) {
                        Ok(new_value) => {
                            if let Some(gvas) = &mut self.files.gvas {
                                gvas.properties.insert(key.clone(), new_value);
                            }
                            logger::info("Applied property changes");
                        }
                        Err(e) => {
                            logger::info(&format!("Invalid JSON: {}", e));
                        }
                    }
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        egui::vec2(ui.available_width(), 0.0),
                        egui::TextEdit::multiline(buf)
                            .code_editor()
                    );
                });


            } else {
                ui.label("Select a property from the left panel");
            }
        });
    }
}