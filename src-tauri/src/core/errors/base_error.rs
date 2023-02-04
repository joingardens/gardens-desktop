use tauri::http::status::StatusCode;

pub struct BaseError {
    pub status_code: StatusCode,
    pub description: String,
}