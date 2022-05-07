pub mod kitty {

    use std::process::Command;

    fn kitty_theme_change() {
        let command = Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg("Gruvbox Dark")
            .output()
            .expect("failed to execute command");

        // let command_output = String::from_utf8_lossy(&command.stdout);
        let command_err = String::from_utf8_lossy(&command.stderr);

        if !command_err.is_empty() {
            panic!("Error: {}", command_err);
        } else {
            println!("Kitty theme changed successfully");
        }

        // println!("kitty status: {}", &command.status);
        // println!("kitty outupt: {}", command_output);
        // println!("kitty err: {}", command_err);
    }

    pub fn execute() {
        kitty_theme_change();
    }
}
