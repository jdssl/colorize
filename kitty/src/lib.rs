pub mod commands {

    use dotenv::dotenv;
    use std::env;
    use std::process::Command;
    use std::vec::Vec;

    /// Convert output command to string
    fn convert_command_result_to_string(command: &std::vec::Vec<u8>) -> String {
        return String::from_utf8_lossy(command).to_string();
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

    /// Execute a command to kitty theme change
    fn command_kitty_theme_change(theme_name: &'static str) -> std::process::Output {
        return Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg(theme_name)
            .output()
            .expect("failed to change kitty theme");
    }

    /// Execute a command to return a theme list from folder
    fn command_list_theme_folder(theme_folder: String) -> std::process::Output {
        return Command::new("ls")
            .arg(theme_folder)
            .output()
            .expect("failed to find kitty theme folder");
    }

    /// Change Kitty theme
    pub fn kitty_theme_change(theme_name: &'static str) -> Result<bool, String> {
        let command = command_kitty_theme_change(theme_name);
        let command_err = convert_command_result_to_string(&command.stderr);
        let command_failed = !command_err.is_empty();

        if command_failed {
            Err(format!("{}", command_err))
        } else {
            Ok(true)
        }
    }

    /// Return a themes list
    pub fn kitty_theme_folder() -> Result<Vec<String>, String> {
        dotenv().ok();
        let theme_folder = env::var("THEME_FOLDER_URL").expect("must be set");
        let command = command_list_theme_folder(theme_folder);
        let command_err = convert_command_result_to_string(&command.stderr);
        let command_output = convert_command_result_to_string(&command.stdout);
        let command_failed = !command_err.is_empty();

        if command_failed {
            Err(format!("{}", command_err))
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

            Ok(vec_theme_sanitized)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn append_string(buffer: &mut Vec<u8>, data: &str) {
            for value in data.bytes() {
                buffer.push(value);
            }
        }

        #[test]
        fn it_convert_command_output_to_string() {
            let mut buffer = Vec::new();
            let word = "hello";
            append_string(&mut buffer, word);
            assert_eq!("hello", convert_command_result_to_string(&buffer));
        }

        #[test]
        fn it_capitalize_first_letter() {
            assert_eq!("Ayu", capitalize("ayu"));
        }

        #[test]
        fn it_capitalize_first_letter_with_the_compost_name() {
            assert_eq!("Gruvbox_dark", capitalize("gruvbox_dark"));
        }

        #[test]
        fn it_sanitize_theme_name() {
            let theme_name = "ayu";
            assert_eq!("Ayu", sanitize_theme_name(theme_name));
        }

        #[test]
        fn it_sanitize_theme_name_with_the_compost_name() {
            let theme_name = "gruvbox_dark";
            assert_eq!("Gruvbox Dark", sanitize_theme_name(theme_name));
        }

        // TODO: How mock/stub/spy Command::new result?
    }
}
