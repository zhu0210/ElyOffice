// Release builds open no console on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ely_mark::{desk, remote};
use ely_platform::AppId;

fn main() {
    if let Some(code) = remote::command_line() {
        std::process::exit(code);
    }
    ely_ui::run(AppId::Mark, desk::open);
}
