mod utils;
use utils::handlers::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![vscode_verify, vscode_scan, compiler_setup_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
