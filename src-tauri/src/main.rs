// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, LogicalSize};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();
      let final_size = window
          .current_monitor().ok().flatten()
          .map(|monitor| {
              let size = monitor.size();
              LogicalSize::new(size.width as f64 * 0.8, size.height as f64 * 0.8)
          })
          .unwrap_or_else(|| LogicalSize::new(1280.0, 720.0));
      window.set_size(final_size)?;
      window.center()?;

      Ok(())
    }) 
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
