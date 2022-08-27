#[allow(unused_imports)]
use mockall::{automock, predicate::*};
use std::process::Command;
use std::vec::Vec;

extern crate directories;
use directories::BaseDirs;

mod utils;

pub struct Kitty;

#[cfg_attr(test, automock)]
pub trait Commands {
    /// Execute a command to kitty theme change
    fn theme_change(&self, theme_name: String) -> Result<String, String>;

    /// Execute a command to return a theme list from folder
    fn list_themes_from_folder(&self, folder_name: String) -> Result<Vec<String>, String>;

    /// Get theme path from colorize file
    fn get_themes_path_from_colorize_file(&self) -> Result<String, String>;
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
            .arg(folder_name.trim())
            .output()
            .expect("failed to find kitty theme folder");

        let command_err = utils::convert_command_result_to_string(&command.stderr);
        let command_output = utils::convert_command_result_to_string(&command.stdout);
        let command_failed = !command_err.is_empty();

        if command_failed {
            Err(command_err)
        } else {
            let theme_split = command_output.split(".conf\n");
            let mut vec_split: Vec<&str> = theme_split.collect();

            vec_split.pop();

            let mut vec_theme_sanitized = vec![];
            for theme_name in vec_split {
                let mut theme_name_sanitized = sanitize_theme_name(theme_name);
                theme_name_sanitized = theme_name_sanitized.replace('_', " ");
                vec_theme_sanitized.push(theme_name_sanitized);
            }

            Ok(vec_theme_sanitized)
        }
    }

    fn get_themes_path_from_colorize_file(&self) -> Result<String, String> {
        if let Some(base_dirs) = BaseDirs::new() {
            let file = "colorize";
            let home = base_dirs.home_dir();
            let full_path = format!("{}/{}", home.to_string_lossy(), file);

            let command = Command::new("cat")
                .arg(full_path)
                .output()
                .expect("failed to find colorize file");

            let command_err = utils::convert_command_result_to_string(&command.stderr);
            let command_output = utils::convert_command_result_to_string(&command.stdout);
            let command_failed = !command_err.is_empty();

            if command_failed {
                Err(command_err)
            } else {
                Ok(command_output)
            }
        } else {
            Err("Oops something went wrong".to_string())
        }
    }
}

/// Return a theme name sanitized
fn sanitize_theme_name(s: &str) -> String {
    let theme_split = s.split('_');

    let mut theme_capitalized = vec![];

    for theme in theme_split {
        theme_capitalized.push(utils::capitalize(theme));
    }

    theme_capitalized.join(" ")
}

/// Change Kitty theme
pub fn kitty_theme_change(cmd: Box<dyn Commands>, theme_name: String) -> Result<String, String> {
    cmd.theme_change(theme_name)
}

/// Return a themes list
pub fn kitty_theme_folder(cmd: Box<dyn Commands>) -> Result<Vec<String>, String> {
    let theme_folder = cmd.get_themes_path_from_colorize_file();
    cmd.list_themes_from_folder(theme_folder.unwrap())
}

pub fn colorize_file(cmd: Box<dyn Commands>) -> Result<String, String> {
    cmd.get_themes_path_from_colorize_file()
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
        let mut mock_commands = Box::new(MockCommands::new());

        mock_commands
            .expect_theme_change()
            .once()
            .returning(|_x| Err(String::from("failed to change kitty theme")));

        let expected_message = kitty_theme_change(mock_commands, theme_name);

        assert_eq!(
            expected_message,
            Err(String::from("failed to change kitty theme"))
        );
    }

    #[test]
    fn it_change_theme() {
        let theme_name = String::from("Ayu");
        let mut mock_commands = Box::new(MockCommands::new());

        mock_commands
            .expect_theme_change()
            .once()
            .returning(|_x| Ok(String::from("Theme changed successfully")));

        let expected_message = kitty_theme_change(mock_commands, theme_name);

        assert_eq!(
            expected_message,
            Ok(String::from("Theme changed successfully"))
        );
    }

    #[test]
    fn it_throw_error_when_kitty_theme_folder_failed() {
        let mut mock_commands = Box::new(MockCommands::new());

        mock_commands
            .expect_get_themes_path_from_colorize_file()
            .once()
            .returning(|| Ok(String::from("any_path")));

        mock_commands
            .expect_list_themes_from_folder()
            .once()
            .returning(|_x| Err(String::from("failed to list themes from folder")));

        let expected_message = kitty_theme_folder(mock_commands);

        assert_eq!(
            expected_message,
            Err(String::from("failed to list themes from folder"))
        );
    }

    #[test]
    fn it_kitty_theme_folder() {
        let mut mock_commands = Box::new(MockCommands::new());

        mock_commands
            .expect_get_themes_path_from_colorize_file()
            .once()
            .returning(|| Ok(String::from("any_path")));

        mock_commands
            .expect_list_themes_from_folder()
            .with(eq(String::from("any_path")))
            .once()
            .returning(|_x| Ok(vec![String::from("Theme One"), String::from("Theme Two")]));

        let expected_message = kitty_theme_folder(mock_commands);

        assert_eq!(
            expected_message,
            Ok(vec![String::from("Theme One"), String::from("Theme Two")])
        );
    }
}
