use std::fs;
use std::path::{Component, Path};
use std::process::Command;

use serde_json::Value;
use tauri::path::BaseDirectory;
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

fn find_hash_by_resource_path(root: &Value, resource_path: &str) -> Option<String> {
    let music_obj = root.get("music")?.as_object()?;
    for group in music_obj.values() {
        let group_obj = group.as_object()?;
        if let Some(item) = group_obj.get(resource_path) {
            let hash = item.get("hash")?.as_str()?;
            return Some(hash.to_string());
        }
    }

    None
}

fn sha1_of_file(file_path: &Path) -> Result<String, String> {
    let output = Command::new("sha1sum")
        .arg(file_path)
        .output()
        .map_err(|e| format!("计算 SHA-1 失败 ({}): {e}", file_path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "计算 SHA-1 失败 ({}): {}",
            file_path.display(),
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let hash = stdout
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("解析 SHA-1 输出失败 ({})", file_path.display()))?;

    Ok(hash.to_lowercase())
}

/// 在 music.json 中，查询名为 resource_path 的 Object 的 hash 字段。
/// 例如，resource_path 为 "minecraft/sounds/music/game/danny.ogg" 时，
/// hash 为 "603fd41ebe3eac0795ebfa0b31195be1866d1e9c"。
/// 如果文件 '<app_data_dir>/assets/' + resource_path 已存在，并且hash校验通过，则跳过下载直接返回Ok。
/// 从 Mojang 的服务器异步下载这个文件，保存到 '<app_data_dir>/assets/' + resource_path，然后返回Ok。
/// 例如，在 hash 为 "7d2d4b60ec51462992b7429aa861d03048cd43c1" 时，URL 为：
/// https://resources.download.minecraft.net/7d/7d2d4b60ec51462992b7429aa861d03048cd43c1
#[tauri::command]
pub async fn request_music_async(
    app: tauri::AppHandle,
    resource_path: String,
) -> Result<(), String> {
    let relative_path = validate_resource_path(&resource_path)?;

    let music_json_path = app
        .path()
        .resolve("resources/music.json", BaseDirectory::Resource)
        .map_err(|e| format!("解析 music.json 资源路径失败: {e}"))?;
    let music_json_content = fs::read_to_string(&music_json_path)
        .map_err(|e| format!("读取 music.json 失败 ({}): {e}", music_json_path.display()))?;
    let parsed: Value = serde_json::from_str(&music_json_content)
        .map_err(|e| format!("解析 music.json 失败: {e}"))?;

    let hash = find_hash_by_resource_path(&parsed, &resource_path)
        .ok_or_else(|| format!("music.json 中未找到资源: {resource_path}"))?;

    if hash.len() < 2 {
        return Err(format!("hash 无效: {hash}"));
    }

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取 app_data_dir 失败: {e}"))?;
    let target_path = app_data_dir.join("assets").join(relative_path);

    if target_path.exists() {
        let current_hash = sha1_of_file(&target_path)?;
        if current_hash == hash.to_lowercase() {
            return Ok(());
        }
    }

    let prefix = &hash[..2];
    let url = format!("https://resources.download.minecraft.net/{prefix}/{hash}");
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("下载失败: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("下载失败，HTTP 状态码: {}", response.status()));
    }
    let data = response
        .bytes()
        .await
        .map_err(|e| format!("读取下载数据失败: {e}"))?;

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("创建目录失败 ({}): {e}", parent.display()))?;
    }
    fs::write(&target_path, &data)
        .map_err(|e| format!("写入文件失败 ({}): {e}", target_path.display()))?;

    Ok(())
}
