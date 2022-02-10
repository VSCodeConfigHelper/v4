use serde::Deserialize;

#[derive(Deserialize)]
pub struct VscodeVerifyArg {
  path: String
}

#[tauri::command]
pub fn vscode_verify(path: VscodeVerifyArg) -> Result<(), &'static str> {
  Ok(())
}
