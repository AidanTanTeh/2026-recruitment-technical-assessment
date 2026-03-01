use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn process_data(Json(request): Json<DataRequest>) -> impl IntoResponse {
    // Calculate sums and return response
    let mut string_len: usize = 0;
    let mut int_sum: i64 = 0;

    for item in request.data {
        // If it's a string, add its length
        if let Some(s) = item.as_str() {
            string_len += s.len();
            continue;
        }
        // If it's an integer, add it
        if let Some(n) = item.as_i64() {
            int_sum += n;
            continue;
        }

        // ignore everything else
    }

    let response = DataResponse {
        string_len,
        int_sum,
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
pub struct DataRequest {
    // Add any fields here
    pub data: Vec<Value>,
}

#[derive(Serialize)]
pub struct DataResponse {
    // Add any fields here
    pub string_len: usize,
    pub int_sum: i64,
}
