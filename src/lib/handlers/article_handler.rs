use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct CreateArticleValidator {
  pub label: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct UpdateArticleValidator {
  pub label: Option<String>,
}

pub async fn article_index(State(state): State<Arc<AppState>>) -> Json<Value> {
  let articles = &state.article_service.articles;
  Json(json!(**articles))
}

pub async fn article_show(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Json<Value> {
  let article = &state.article_service.get(&id);

  match article {
    Some(_) => Json(json!(article)),
    None => Json(json!({}))
  }
}

pub async fn article_create(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateArticleValidator>,
) -> Json<Value> {
  println!("New Article received");
  state.article_service.add(payload.label);

  Json(json!({ "message": "Created" }))
}

pub async fn article_update(
  State(state): State<Arc<AppState>>,
  Path(id): Path<String>,
  Json(payload): Json<UpdateArticleValidator>,
) -> Json<Value> {
  state.article_service.update(&id, payload);

  Json(json!({}))
}

pub async fn article_delete(
  State(state): State<Arc<AppState>>,
  Path(id): Path<String>,
) -> Json<Value> {
  state.article_service.delete(&id);

  Json(json!({}))
}
