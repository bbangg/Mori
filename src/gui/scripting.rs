use std::sync::{Arc, RwLock};
use std::thread;
use eframe::egui::{self, Ui};
use egui::include_image;
use crate::manager::bot_manager::BotManager;
use crate::utils;

#[derive(Default)]
pub struct Scripting {
    pub selected_bot: String,
    pub code: String,
}

impl Scripting {
    pub fn render(&mut self, ui: &mut Ui, manager: &Arc<RwLock<BotManager>>) {
        self.selected_bot = utils::config::get_selected_bot();
        ui.vertical(|ui| {
            if ui.add_sized([30.0, 30.0], egui::Button::image(
                include_image!("../../assets/play.svg"),
            )).clicked() {
                let bot = {
                    let manager = manager.read().unwrap();

                    match manager.get_bot(&self.selected_bot) {
                        Some(bot) => Some(bot.clone()),
                        None => None,
                    }
                };
                if let Some(bot) = bot {
                    let bot_clone = bot.clone();
                    let code = self.code.clone();
                    thread::spawn(move || {
                        let lua = bot_clone.lua.lock().unwrap();
                        lua.load(&code).exec().unwrap();
                    });
                }
            }
            if ui.add_sized([30.0, 30.0], egui::Button::image(
                include_image!("../../assets/square.svg"),
            )).clicked() {}
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
            let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    &theme,
                    string,
                    "lua",
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };
            ui.add_sized([ui.available_width(), ui.available_height()],
                         egui::TextEdit::multiline(&mut self.code)
                             .code_editor()
                             .desired_rows(10)
                             .desired_width(f32::INFINITY)
                             .layouter(&mut layouter));
        });
    }
}
