use eframe::{
    egui::{Button, CentralPanel, Grid, ScrollArea},
    App,
};

use kitty::{kitty_theme_change, kitty_theme_folder, Kitty};

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
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.);
            let mut iter = 0;

            ScrollArea::vertical().show(ui, |ui| {
                Grid::new("some_unique_id").show(ui, |ui| {
                    for theme in &self.kitty_themes {
                        iter += 1;

                        let button = ui.add(Button::new(theme));

                        if iter % 4 == 0 {
                            ui.end_row();
                        }

                        if button.clicked() {
                            let _message =
                                kitty_theme_change(Box::new(Kitty {}), String::from(theme));
                        }
                    }
                });
            });
        });
    }
}

pub fn init() -> eframe::Result {
    let name = "colorize - v0.2.0";
    let app = Box::new(Colorize::new());
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(name, native_options, Box::new(|_cc| Ok(app)))
}
