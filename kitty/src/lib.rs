pub mod commands {

    use dotenv::dotenv;
    use std::env;
    use std::process::Command;
    use std::vec::Vec;

    /// Convert output command to string
    fn convert_to_string(command: &std::vec::Vec<u8>) -> String {
        return String::from_utf8_lossy(command).to_string();
    }

    /// Change Kitty theme
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

    /// Return a string with the first letter uppercase
    fn capitalize(s: &str) -> String {
        s[0..1].to_uppercase() + &s[1..]
    }

    /// Return a theme name sanitized
    fn sanitize_theme_name(s: &str) -> String {
        let theme_split = s.split("_");

        let mut theme_capitalized = vec![];

        for theme in theme_split {
            theme_capitalized.push(capitalize(theme));
        }

        return theme_capitalized.join(" ");
    }

    /// Return a themes list
    fn kitty_theme_folder() -> Vec<String> {
        dotenv().ok();
        let theme_folder = env::var("THEME_FOLDER_URL").expect("must be set");
        let command = Command::new("ls")
            .arg(theme_folder)
            .output()
            .expect("failed to find kitty theme folder");

        let command_err = convert_to_string(&command.stderr);
        let command_output = convert_to_string(&command.stdout);
        let command_failed = !command_err.is_empty();

        if command_failed {
            panic!("{}", command_err);
        } else {
            let theme_split = command_output.split(".conf\n");
            let mut vec_split: Vec<&str> = theme_split.collect();

            vec_split.pop();

            let mut vec_theme_sanitized = vec![];
            for theme_name in vec_split {
                let mut theme_name_sanitized = sanitize_theme_name(theme_name);
                theme_name_sanitized = theme_name_sanitized.replace("_", " ");
                vec_theme_sanitized.push(theme_name_sanitized);
            }

            return vec_theme_sanitized;
        }
    }

    pub fn execute() {
        // let theme = "Gruvbox Dark";
        // let theme = "Dracula";
        // kitty_theme_change(theme);
        let themes: Vec<String> = kitty_theme_folder();
        println!("{:?}", themes);
    }
}
