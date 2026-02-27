use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub status_code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(success: bool, status_code: StatusCode, message: String, data: Option<T>) -> Self {
        Self {
            success,
            status_code: status_code.as_u16(),
            message,
            data,
        }
    }
}