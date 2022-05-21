#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod calculator;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![calculator::select_operation])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
