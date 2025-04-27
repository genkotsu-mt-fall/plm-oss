use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Part {
    id: String,
    part_number: String,
    name: String,
    description: String,
    kind: String,
}

pub async fn get_parts() -> Json<Vec<Part>> {
    let parts = vec![Part {
        id: "00000000-0000-0000-0000-000000000001".to_string(),
        part_number: "ABC-123".to_string(),
        name: "ねじ".to_string(),
        description: "ステンレス製のねじ".to_string(),
        kind: "部品".to_string(),
    }];

    Json(parts)
}
