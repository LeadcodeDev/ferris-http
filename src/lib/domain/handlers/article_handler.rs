use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::data::validators::article_validator::{CreateArticleValidator, UpdateArticleValidator};
use crate::infrastructure::app_state::AppState;

pub async fn article_index(State(state): State<Arc<AppState>>) -> Json<Value> {
  let articles = &state.article_service.index();
  Json(json!(articles))
}

pub async fn article_show(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> Json<Value> {
  let article = &state.article_service.get(id);

  match article {
    Some(_) => Json(json!(article)),
    None => Json(json!({}))
  }
}

pub async fn article_create(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateArticleValidator>,
) -> Json<Value> {
  state.article_service.create(payload);

  Json(json!({ "message": "Created" }))
}

pub async fn article_update(
  State(state): State<Arc<AppState>>,
  Path(id): Path<u32>,
  Json(payload): Json<UpdateArticleValidator>,
) -> Json<Value> {
  state.article_service.update(id, payload);

  Json(json!({}))
}

pub async fn article_delete(
  State(state): State<Arc<AppState>>,
  Path(id): Path<u32>,
) -> Json<Value> {
  state.article_service.delete(id);

  Json(json!({}))
}
