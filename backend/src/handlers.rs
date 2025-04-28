use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Part {
    id: String,
    part_number: String,
    name: String,
    description: String,
    kind: String,
}

pub async fn get_parts(State(parts): State<PartState>) -> Json<Vec<Part>> {
    let parts_lock = parts.lock().await;
    Json(parts_lock.clone())
}

pub type PartState = Arc<Mutex<Vec<Part>>>;

#[derive(Deserialize)]
pub struct NewPart {
    part_number: String,
    name: String,
    description: String,
    kind: String,
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub async fn create_part(
    State(parts): State<PartState>,
    Json(new_part): Json<NewPart>,
) -> Json<Part> {
    let part = Part {
        id: generate_uuid(),
        part_number: new_part.part_number,
        name: new_part.name,
        description: new_part.description,
        kind: new_part.kind,
    };

    let mut parts_lock = parts.lock().await;
    parts_lock.push(part.clone());

    Json(part)
}

pub async fn get_part(
    State(parts): State<PartState>,
    Path(id): Path<String>,
) -> Result<Json<Part>, StatusCode> {
    let parts_lock = parts.lock().await;
    if let Some(part) = parts_lock.iter().find(|p| p.id == id) {
        Ok(Json(part.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn update_part(
    State(parts): State<PartState>,
    Path(id): Path<String>,
    Json(updated_part): Json<NewPart>,
) -> Result<Json<Part>, StatusCode> {
    let mut parts_lock = parts.lock().await;
    if let Some(part) = parts_lock.iter_mut().find(|p| p.id == id) {
        part.part_number = updated_part.part_number;
        part.name = updated_part.name;
        part.description = updated_part.description;
        part.kind = updated_part.kind;

        Ok(Json(part.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
