use eframe::{
    egui::{CentralPanel, ComboBox},
    epi::App,
    run_native, NativeOptions,
};

use kitty;

struct Colorize {
    kitty_themes: Vec<String>,
}

impl Colorize {
    fn new() -> Colorize {
        let theme_list = kitty::commands::kitty_theme_folder();
        Colorize {
            kitty_themes: theme_list,
        }
    }
}

impl App for Colorize {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            let mut selected = 0;
            let mut index = 0;
            ComboBox::from_label("").width(150.0).show_ui(ui, |ui| {
                for theme in &self.kitty_themes {
                    index = index + 1;
                    ui.selectable_value(&mut selected, index, theme);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Colorize"
    }
}

fn main() {
    let app = Colorize::new();
    let win_option = NativeOptions::default();

    run_native(Box::new(app), win_option);
}
