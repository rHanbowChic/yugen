use axum::{
    body::Body,
    extract::{Path as AxumPath, State},
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    routing::get,
    Router,
};
use serde_json::Value;
use std::{
    fs,
    path::{Component, PathBuf},
    thread,
};

#[derive(Clone)]
struct AppState {
    assets_root: PathBuf,
}

// 这个服务器只负责传输ogg
pub fn start_server() {
    thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to create tokio runtime");

        runtime.block_on(async {
            let assets_root = resolve_assets_root();
            let app = Router::new()
                .route("/assets/*path", get(serve_assets))
                .with_state(AppState { assets_root });

            let listener = tokio::net::TcpListener::bind("127.0.0.1:10454")
                .await
                .expect("failed to bind http server");
            if let Err(err) = axum::serve(listener, app).await {
                eprintln!("asset server error: {err}");
            }
        });
    });
}

fn resolve_assets_root() -> PathBuf {
    if let Some(app_data_dir) = tauri_app_data_dir() {
        return app_data_dir.join("assets");
    }

    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home).join(".local/share/fyi.ect.yugen/assets")
}

fn tauri_app_data_dir() -> Option<PathBuf> {
    let config: Value = serde_json::from_str(include_str!("../tauri.conf.json")).ok()?;
    let identifier = config.get("identifier")?.as_str()?;
    let data_dir = platform_data_dir()?;
    Some(data_dir.join(identifier))
}

fn platform_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        return std::env::var_os("APPDATA").map(PathBuf::from);
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")?;
        return Some(
            PathBuf::from(home)
                .join("Library")
                .join("Application Support"),
        );
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Some(xdg_data_home) = std::env::var_os("XDG_DATA_HOME") {
            return Some(PathBuf::from(xdg_data_home));
        }
        let home = std::env::var_os("HOME")?;
        return Some(PathBuf::from(home).join(".local").join("share"));
    }
}

async fn serve_assets(
    State(state): State<AppState>,
    AxumPath(path): AxumPath<String>,
    headers: HeaderMap,
) -> Response<Body> {
    let mut real_path = state.assets_root.clone();
    for comp in PathBuf::from(path).components() {
        if let Component::Normal(seg) = comp {
            real_path.push(seg);
        } else {
            return simple_response(StatusCode::BAD_REQUEST, "invalid path");
        }
    }

    let bytes = match fs::read(&real_path) {
        Ok(v) => v,
        Err(_) => return simple_response(StatusCode::NOT_FOUND, "not found"),
    };

    let total_len = bytes.len() as u64;
    let range_header = headers
        .get(header::RANGE)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    if let Some(raw_range) = range_header {
        match parse_range(&raw_range, total_len) {
            Ok((start, end)) => {
                let body = Body::from(bytes[start as usize..=end as usize].to_vec());
                let mut resp = Response::new(body);
                *resp.status_mut() = StatusCode::PARTIAL_CONTENT;
                add_common_headers(resp.headers_mut(), total_len);
                resp.headers_mut().insert(
                    header::CONTENT_RANGE,
                    HeaderValue::from_str(&format!("bytes {start}-{end}/{total_len}"))
                        .unwrap_or_else(|_| HeaderValue::from_static("bytes */0")),
                );
                resp.headers_mut().insert(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&(end - start + 1).to_string())
                        .unwrap_or(HeaderValue::from_static("0")),
                );
                return resp;
            }
            Err(_) => {
                let mut resp = Response::new(Body::empty());
                *resp.status_mut() = StatusCode::RANGE_NOT_SATISFIABLE;
                add_common_headers(resp.headers_mut(), total_len);
                resp.headers_mut().insert(
                    header::CONTENT_RANGE,
                    HeaderValue::from_str(&format!("bytes */{total_len}"))
                        .unwrap_or(HeaderValue::from_static("bytes */0")),
                );
                return resp;
            }
        }
    }

    let mut resp = Response::new(Body::from(bytes));
    *resp.status_mut() = StatusCode::OK;
    add_common_headers(resp.headers_mut(), total_len);
    resp.headers_mut().insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&total_len.to_string()).unwrap_or(HeaderValue::from_static("0")),
    );
    resp
}

fn add_common_headers(headers: &mut HeaderMap, total_len: u64) {
    headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("audio/ogg"));
    if total_len == 0 {
        headers.insert(header::CONTENT_LENGTH, HeaderValue::from_static("0"));
    }
}

fn parse_range(raw: &str, total_len: u64) -> Result<(u64, u64), ()> {
    if total_len == 0 {
        return Err(());
    }
    if !raw.starts_with("bytes=") {
        return Err(());
    }
    let range = &raw[6..];
    if range.contains(',') {
        return Err(());
    }
    let (start_s, end_s) = range.split_once('-').ok_or(())?;

    if start_s.is_empty() {
        let suffix_len: u64 = end_s.parse().map_err(|_| ())?;
        if suffix_len == 0 {
            return Err(());
        }
        let start = total_len.saturating_sub(suffix_len);
        let end = total_len - 1;
        return Ok((start, end));
    }

    let start: u64 = start_s.parse().map_err(|_| ())?;
    let end: u64 = if end_s.is_empty() {
        total_len - 1
    } else {
        end_s.parse().map_err(|_| ())?
    };

    if start > end || start >= total_len {
        return Err(());
    }
    Ok((start, end.min(total_len - 1)))
}

fn simple_response(status: StatusCode, message: &'static str) -> Response<Body> {
    let mut resp = Response::new(Body::from(message));
    *resp.status_mut() = status;
    resp
}
