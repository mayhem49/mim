#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]

mod editor;
use editor::Editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut editor = Editor::default();
    //do better
    if let Some(arg) = args.get(1) {
        if std::path::Path::new(arg).exists() {
            editor.run(Some(arg.clone()));
        } else {
            editor.run(None);
        }
    } else {
        editor.run(None);
    }
}
