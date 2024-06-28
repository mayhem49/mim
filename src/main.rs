#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]

extern crate simplelog;

use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

mod editor;
use editor::Editor;

fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("my_rust_binary.log").unwrap(),
    )
    .unwrap();
    Editor::new().unwrap().run();
}
