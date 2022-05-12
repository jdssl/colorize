fn main() {
    let themes = kitty::commands::kitty_theme_folder();
    println!("{:?}", themes);
}
