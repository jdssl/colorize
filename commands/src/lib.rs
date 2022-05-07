use std::process::Command;

pub struct Commands {}

impl Commands {
    pub fn execute() {
        let list_dir = Command::new("kitty")
            .arg("+kitten")
            .arg("themes")
            .arg("--reload-in=all")
            .arg("Ayu")
            .output()
            .expect("failed to execute process");

        let list_dir_output = String::from_utf8_lossy(&list_dir.stdout);

        println!("list_dir status: {}", &list_dir.status);
        println!("list_dir outupt: {}", list_dir_output);
    }
}
