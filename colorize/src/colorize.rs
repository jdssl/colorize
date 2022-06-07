use eframe::{
    egui::{Button, CentralPanel, Grid, ScrollArea, Vec2},
    run_native, App, NativeOptions,
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
                        iter = iter + 1;

                        let button = ui.add(Button::new(theme));

                        if iter % 5 == 0 {
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

pub fn init() {
    let name = "colorize - v0.2.0";
    let app = Box::new(Colorize::new());
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 70.));

    run_native(name, win_option, Box::new(|_cc| app));
}
