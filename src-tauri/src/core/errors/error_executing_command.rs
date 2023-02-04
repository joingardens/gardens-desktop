use tauri::http::status::StatusCode;

use super::base_error::{BaseError};

impl BaseError {
    pub fn error_executing_command(command: &str) -> Self {
        return Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            description: format!("Error executing {} command", command),
        }
    }
}