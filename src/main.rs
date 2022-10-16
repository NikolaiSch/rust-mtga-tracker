pub mod log_parser;

fn main() {
    let mut logs = log_parser::Log::new("/Users/vii/Documents/rust-mtg-viewer/dev_files/Player.log");

    logs.init();
}
