mod colorize;

fn main() {
    // GUI
    match colorize::init() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e)
        }
    }
}
