use eframe::{
    egui::{Button, CentralPanel, Grid, ScrollArea, Vec2},
    epi::App,
    run_native, NativeOptions,
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
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.);
            let mut iter = 0;

            ScrollArea::auto_sized().show(ui, |ui| {
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

    fn name(&self) -> &str {
        "colorize - v0.1.0"
    }
}

pub fn init() {
    let app = Colorize::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 70.));

    run_native(Box::new(app), win_option);
}
