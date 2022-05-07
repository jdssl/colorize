pub mod kitty {

    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    // use std::fs;

    fn kitty_theme_change(theme: &'static str) {
        let command = Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg(theme)
            .output()
            .expect("failed to change kitty theme");

        let command_err = String::from_utf8_lossy(&command.stderr);

        if !command_err.is_empty() {
            panic!("{}", command_err);
        } else {
            println!("kitty theme changed successfully");
        }
    }

    fn kitty_theme_folder() {
        const THEME_FOLDER: &'static str = "../../.config/kitty/themes/";
        let command = Command::new("ls")
            .arg(THEME_FOLDER)
            .output()
            .expect("failed to find kitty theme folder");

        let command_err = String::from_utf8_lossy(&command.stderr);
        let command_output = String::from_utf8_lossy(&command.stdout);

        println!("{}", command_output);
        if !command_err.is_empty() {
            panic!("{}", command_err);
        } else {
        }
    }

    pub fn execute() {
        // let theme = "Gruvbox Dark";
        // let theme = "Dracula";
        // kitty_theme_change(theme);
        kitty_theme_folder();
    }
}
