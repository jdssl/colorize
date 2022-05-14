fn main() {
    let themes = kitty::commands::kitty_theme_folder();
    // kitty::commands::kitty_theme_change("Gruvbox Dark");
    println!("{:?}", themes);
}
