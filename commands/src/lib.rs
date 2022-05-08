pub mod kitty {

    use dotenv::dotenv;
    use std::env;
    use std::process::Command;

    fn convert_to_string(command: &std::vec::Vec<u8>) -> String {
        return String::from_utf8_lossy(command).to_string();
    }

    fn kitty_theme_change(theme: &'static str) {
        let command = Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg(theme)
            .output()
            .expect("failed to change kitty theme");

        let command_err = convert_to_string(&command.stderr);

        if !command_err.is_empty() {
            panic!("{}", command_err);
        } else {
            println!("kitty theme changed successfully");
        }
    }

    fn kitty_theme_folder() {
        dotenv().ok();
        let theme_folder = env::var("THEME_FOLDER_URL").expect("must be set");
        let command = Command::new("ls")
            .arg(theme_folder)
            .output()
            .expect("failed to find kitty theme folder");

        let command_err = convert_to_string(&command.stderr);
        let command_output = convert_to_string(&command.stdout);

        if !command_err.is_empty() {
            panic!("{}", command_err);
        } else {
            let theme_split = command_output.split(".conf\n");
            let vec: Vec<&str> = theme_split.collect();
            println!("{:?}", vec);

            // To each theme name uppercase first letter and empty string
            todo!();
        }
    }

    pub fn execute() {
        // let theme = "Gruvbox Dark";
        // let theme = "Dracula";
        // kitty_theme_change(theme);
        kitty_theme_folder();
    }
}
