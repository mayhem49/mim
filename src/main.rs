#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]

extern crate simplelog;

use std::fs::File;
use simplelog::{WriteLogger,LevelFilter,Config};


mod editor;
use editor::Editor;

fn main() {
    WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("my_rust_binary.log").unwrap(),
    );
    Editor::new().unwrap().run();
}
