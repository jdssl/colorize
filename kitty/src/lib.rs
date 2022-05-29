use dotenv::dotenv;
use mockall::{automock, predicate::*};
use std::env;
use std::process::Command;
use std::vec::Vec;
mod utils;

pub struct Kitty;

#[cfg_attr(test, automock)]
pub trait Commands {
    /// Execute a command to kitty theme change
    fn theme_change(&self, theme_name: String) -> Result<String, String>;

    /// Execute a command to return a theme list from folder
    fn list_themes_from_folder(&self, folder_name: String) -> Result<Vec<String>, String>;
}

impl Commands for Kitty {
    fn theme_change(&self, theme_name: String) -> Result<String, String> {
        let message_success = String::from("Theme changed successfully");
        let status = Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg(theme_name)
            .status()
            .expect("failed to change kitty theme");

        if status.success() {
            Ok(message_success)
        } else {
            Err(format!("failed change theme: {}", status))
        }
    }

    fn list_themes_from_folder(&self, folder_name: String) -> Result<Vec<String>, String> {
        let command = Command::new("ls")
            .arg(folder_name)
            .output()
            .expect("failed to find kitty theme folder");

        let command_err = utils::convert_command_result_to_string(&command.stderr);
        let command_output = utils::convert_command_result_to_string(&command.stdout);
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
}

/// Return a theme name sanitized
fn sanitize_theme_name(s: &str) -> String {
    let theme_split = s.split("_");

    let mut theme_capitalized = vec![];

    for theme in theme_split {
        theme_capitalized.push(utils::capitalize(theme));
    }

    return theme_capitalized.join(" ");
}

/// Change Kitty theme
pub fn kitty_theme_change(cmd: Box<dyn Commands>, theme_name: String) -> Result<String, String> {
    cmd.theme_change(theme_name)
}

/// Return a themes list
pub fn kitty_theme_folder(cmd: Box<dyn Commands>) -> Result<Vec<String>, String> {
    dotenv().ok();
    let theme_folder = env::var("THEME_FOLDER_URL").expect("must be set");
    cmd.list_themes_from_folder(theme_folder)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn it_throw_error_when_theme_change_failed() {
        let theme_name = String::from("Ayu");
        let mut mock_kitty_theme_change = Box::new(MockCommands::new());

        mock_kitty_theme_change
            .expect_theme_change()
            .once()
            .returning(|_x| Err(String::from("failed to change kitty theme")));

        let expected_message = kitty_theme_change(mock_kitty_theme_change, theme_name);

        assert_eq!(
            expected_message,
            Err(String::from("failed to change kitty theme"))
        );
    }

    #[test]
    fn it_change_theme() {
        let theme_name = String::from("Ayu");
        let mut mock_kitty_theme_change = Box::new(MockCommands::new());

        mock_kitty_theme_change
            .expect_theme_change()
            .once()
            .returning(|_x| Ok(String::from("Theme changed successfully")));

        let expected_message = kitty_theme_change(mock_kitty_theme_change, theme_name);

        assert_eq!(
            expected_message,
            Ok(String::from("Theme changed successfully"))
        );
    }

    #[test]
    fn it_throw_error_when_kitty_theme_folder_failed() {
        let mut mock_list_themes_from_folder = Box::new(MockCommands::new());

        mock_list_themes_from_folder
            .expect_list_themes_from_folder()
            .once()
            .returning(|_x| Err(String::from("failed to list themes from folder")));

        let expected_message = kitty_theme_folder(mock_list_themes_from_folder);

        assert_eq!(
            expected_message,
            Err(String::from("failed to list themes from folder"))
        );
    }

    #[test]
    fn it_kitty_theme_folder() {
        let mut mock_list_themes_from_folder = Box::new(MockCommands::new());

        mock_list_themes_from_folder
            .expect_list_themes_from_folder()
            .once()
            .returning(|_x| Ok(vec![String::from("Theme One"), String::from("Theme Two")]));

        let expected_message = kitty_theme_folder(mock_list_themes_from_folder);

        assert_eq!(
            expected_message,
            Ok(vec![String::from("Theme One"), String::from("Theme Two")])
        );
    }
}
