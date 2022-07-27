#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_clipboard_text,
            set_clipboard_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use dao_core::{Clipboard, ClipboardData, ClipboardProvider};

#[tauri::command]
fn get_clipboard_text() -> String {
    let clip = Clipboard::new().expect("Could not open clip :/");

    match clip.get().expect("could not get clip :/") {
        ClipboardData::Text(text) => text,
        ClipboardData::Image(_) => "<there is an image>".into(),
    }
}

#[tauri::command]
fn set_clipboard_text(text: String) {
    let clip = Clipboard::new().expect("Could not open clip :/");

    clip.set(ClipboardData::Text(text))
        .expect("could not set clip :/");
}
