// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod prelude;
mod commands;
mod collector;
mod generator;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
        commands::convert, 
        commands::analyze,
        commands::pipe_analyze,
        commands::pipe_volume,
    ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
