// use axum::response::IntoResponse;
// use axum::{Json, extract::Path, extract::State, http::StatusCode};
use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use sqlx::{PgPool, postgres::PgPoolOptions};
use sqlx::PgPool;
// use std::sync::Arc;
// use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct Part {
    pub id: Uuid,
    pub part_number: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct NewPart {
    pub part_number: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: Option<String>,
}

// #[axum::debug_handler]
pub async fn create_part(
    // State(parts): State<PartState>,
    // State(pool): State<Arc<PgPool>>,
    State(pool): State<PgPool>,
    Json(new_part): Json<NewPart>,
) -> Result<Json<Part>, (StatusCode, String)> {
    // let part = Part {
    //     id: generate_uuid(),
    //     part_number: new_part.part_number,
    //     name: new_part.name,
    //     description: new_part.description,
    //     kind: new_part.kind,
    // };

    // let mut parts_lock = parts.lock().await;
    // parts_lock.push(part.clone());
    let part = sqlx::query_as!(
        Part,
        r#"INSERT INTO parts (id, part_number, name, description, kind)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, part_number, name, description, kind, created_at, updated_at"#,
        Uuid::new_v4(),
        new_part.part_number,
        new_part.name,
        new_part.description,
        new_part.kind
    )
    // .fetch_one(pool.as_ref())
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB INSERT失敗: {}", e),
        )
    })?;

    Ok(Json(part))
}

// #[axum::debug_handler]
pub async fn get_parts(
    // State(parts): State<PartState>
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Part>>, (StatusCode, String)> {
    // let parts_lock = parts.lock().await;
    // Json(parts_lock.clone())
    let parts = sqlx::query_as!(
        Part,
        r#"SELECT id, part_number, name, description, kind, created_at, updated_at
        FROM parts
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB SELECT失敗: {}", e),
        )
    })?;

    Ok(Json(parts))
}

// pub type PartState = Arc<Mutex<Vec<Part>>>;

// fn generate_uuid() -> String {
//     Uuid::new_v4().to_string()
// }

// pub async fn get_part(
//     State(parts): State<PartState>,
//     Path(id): Path<String>,
// ) -> Result<Json<Part>, StatusCode> {
//     let parts_lock = parts.lock().await;
//     if let Some(part) = parts_lock.iter().find(|p| p.id == id) {
//         Ok(Json(part.clone()))
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }

// pub async fn update_part(
//     State(parts): State<PartState>,
//     Path(id): Path<String>,
//     Json(updated_part): Json<NewPart>,
// ) -> Result<Json<Part>, StatusCode> {
//     let mut parts_lock = parts.lock().await;
//     if let Some(part) = parts_lock.iter_mut().find(|p| p.id == id) {
//         part.part_number = updated_part.part_number;
//         part.name = updated_part.name;
//         part.description = updated_part.description;
//         part.kind = updated_part.kind;

//         Ok(Json(part.clone()))
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }

// pub async fn delete_part(
//     State(parts): State<PartState>,
//     Path(id): Path<String>,
// ) -> Result<StatusCode, StatusCode> {
//     let mut parts_lock = parts.lock().await;
//     if let Some(pos) = parts_lock.iter().position(|p| p.id == id) {
//         parts_lock.remove(pos);
//         Ok(StatusCode::NO_CONTENT)
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }
