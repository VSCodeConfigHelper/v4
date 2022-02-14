// #![windows_subsystem = "windows"] // Hide console window

mod utils;
use utils::handlers::*;

fn main() {

  #[cfg(target_os = "windows")]
  utils::winapi::hide_console();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      vscode_verify,
      vscode_scan,
      compiler_setup_list,
      compiler_scan,
      compiler_verify,
      compiler_install,
      workspace_verify,
      options_scan,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
