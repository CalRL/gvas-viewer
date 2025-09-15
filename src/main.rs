mod save;
mod logger;

use std::fs::File;
use std::io::Error;
use std::path::PathBuf;
use eframe::Frame;
use egui::{Context, MenuBar, OpenUrl, Ui};
use egui::panel::Side;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use rfd::{FileDialog, MessageDialog};
use crate::save::convert;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
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
    json: Option<String>
}

#[derive(Default)]
pub struct AppState {
    label: String,
    value: f32,
    files: Files
}

impl AppState {
    fn show_menu(&mut self, ui: &mut Ui) {
        MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let dialog: FileDialog = FileDialog::new();
                    let path: Option<PathBuf> = dialog.pick_file();
                    if path.is_some() {
                        self.set_gvas_file(path.unwrap())
                    }

                }

                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }

                if ui.button("Join the Discord").clicked() {
                    let url: OpenUrl = OpenUrl {
                        url: "https://discord.cal.ceo".to_owned(),
                        new_tab: true
                    };
                    ui.ctx().open_url(url)
                }
            })
        });
    }
    /// Sets gvas_file from a given path
    /// if path is invalid, show err and do nothing
    fn set_gvas_file(&mut self, path: PathBuf) {
        let file_res: Result<File, Error> = File::open(path);
        let res: () = match file_res {
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
}

impl Files {
    pub fn from(&mut self, state: AppState) {
        let files: Files = state.files;

        self.gvas = files.gvas;
        self.json = files.json;
        self.gvas_file = files.gvas_file
    }

    fn is_gvas_file_loaded(&self) -> bool {
        return self.gvas_file.is_some()
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::new(Side::Left, "left-panel").show(ctx, |ui| {
            AppState::show_menu(self, ui);
            ui.heading("GVAS-VIEWER");
            ui.horizontal(|ui| {
                ui.label("write something:");
                ui.text_edit_singleline(&mut self.label);
            });
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });
        egui::SidePanel::new(Side::Right, "right-panel").show(ctx, |ui| {
            if let Some(file) = self.files.gvas_file.as_mut() {
                let gvas: GvasFile = GvasFile::read(
                    file,
                    GameVersion::Default
                ).unwrap();
                self.files.json = Some(convert::json::format_json(&gvas).unwrap());
                logger::info("converted to json...")
            }
        });
    }
}
