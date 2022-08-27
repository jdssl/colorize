/// Convert output command to string
pub fn convert_command_result_to_string(command: &[u8]) -> String {
    return String::from_utf8_lossy(command).to_string();
}

/// Return a string with the first letter uppercase
pub fn capitalize(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
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
}
