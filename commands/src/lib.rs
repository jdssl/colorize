use std::process::Command;

pub struct Commands {}

impl Commands {
    pub fn execute() {
        let kitty_theme_change = Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg("Ayu")
            .output()
            .expect("failed to execute process");

        let kitty_theme_change_output = String::from_utf8_lossy(&kitty_theme_change.stdout);

        println!("list_dir status: {}", &kitty_theme_change.status);
        println!("list_dir outupt: {}", kitty_theme_change_output);
    }
}
