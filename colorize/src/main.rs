use eframe::{
    egui::{CentralPanel, ComboBox},
    epi::App,
    run_native, NativeOptions,
};

use kitty::{kitty_theme_folder, Kitty};

struct Colorize {
    kitty_themes: Vec<String>,
}

impl Colorize {
    fn new() -> Colorize {
        let theme_list = kitty_theme_folder(Box::new(Kitty {}));
        Colorize {
            kitty_themes: theme_list.unwrap(),
        }
    }
}

impl App for Colorize {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            let mut selected = 0;
            ComboBox::from_label("Select theme")
                .width(150.0)
                .show_index(ui, &mut selected, self.kitty_themes.len(), |i| {
                    self.kitty_themes[i].to_owned()
                });
        });
    }

    fn name(&self) -> &str {
        "Colorize"
    }
}

fn main() {
    // GUI
    let app = Colorize::new();
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);

    // Code
    let theme_list = kitty_theme_folder(Box::new(Kitty {}));
    println!("{:?}", theme_list);
}
