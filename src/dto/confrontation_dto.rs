use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateConfrontationRequest {
    pub line_one_name: String,
    pub line_two_name: String,
    pub date_of_confrontation: String,
    pub point_of_line_one: i16,
    pub point_of_line_two: i16,
}

#[derive(Serialize)]
pub struct ConfrontationResponse {
    pub line_one_name: String,
    pub line_two_name: String,
    pub date_of_confrontation: String,
    pub point_of_line_one: i16,
    pub point_of_line_two: i16,
}
