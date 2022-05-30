mod colorize;

use kitty::{kitty_theme_folder, Kitty};

fn main() {
    // GUI
    colorize::init();

    // Code
    let theme_list = kitty_theme_folder(Box::new(Kitty {}));
    println!("{:?}", theme_list);
}
