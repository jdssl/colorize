pub mod kitty {

    use std::process::Command;

    fn kitty_theme_change(theme: &'static str) {
        let command = Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg(theme)
            .output()
            .expect("failed to execute command");

        let command_err = String::from_utf8_lossy(&command.stderr);

        if !command_err.is_empty() {
            panic!("{}", command_err);
        } else {
            println!("Kitty theme changed successfully");
        }
    }

    pub fn execute() {
        let theme = "Gruvbox Dark";
        kitty_theme_change(theme);
    }
}
