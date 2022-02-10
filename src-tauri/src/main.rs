mod utils;
use utils::handlers::vscode_verify;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![vscode_verify])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
