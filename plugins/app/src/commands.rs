use tauri::{AppHandle, Runtime};

#[tauri::command]
pub fn version<R: Runtime>(app: AppHandle<R>) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
pub fn name<R: Runtime>(app: AppHandle<R>) -> String {
    app.package_info().name.clone()
}

#[tauri::command]
pub fn tauri_version() -> &'static str {
    // TODO: return actual tauri version with `tauri::VERSION`
    env!("CARGO_PKG_VERSION")
}

#[tauri::command]
#[allow(unused_variables)]
pub fn show<R: Runtime>(app: AppHandle<R>) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    app.show()?;
    Ok(())
}

#[tauri::command]
#[allow(unused_variables)]
pub fn hide<R: Runtime>(app: AppHandle<R>) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    app.hide()?;
    Ok(())
}