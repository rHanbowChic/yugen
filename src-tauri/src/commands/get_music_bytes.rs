use std::fs;
use std::path::{Component, Path};
use tauri::Manager;

fn validate_resource_path(resource_path: &str) -> Result<&Path, String> {
    let path = Path::new(resource_path);
    if path.as_os_str().is_empty() {
        return Err(String::from("resource_path 不能为空"));
    }

    // 仅允许普通相对路径，避免目录穿越和绝对路径访问。
    if path
        .components()
        .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(String::from("resource_path 非法"));
    }

    Ok(path)
}

/// 从 resource_path 读取资源路径。
/// 将其拼接在 '<app_data_dir>/assets' 之后，得到真实文件路径。
/// 获取该文件的二进制数据并返回 Vec<u8> 。
#[tauri::command]
pub fn get_music_bytes(app: tauri::AppHandle, resource_path: &str) -> Result<Vec<u8>, String> {
    let relative_path = validate_resource_path(resource_path)?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取 app_data_dir 失败: {e}"))?;
    let full_path = app_data_dir.join("assets").join(relative_path);

    fs::read(&full_path).map_err(|e| format!("读取文件失败 ({}): {e}", full_path.display()))
}
