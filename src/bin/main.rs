use std::sync::Arc;
use axum::{serve, Router};
use axum::routing::{delete, get, post, put};
use tokio::net::TcpListener;
use lib::data::repositories::article_repository::ArticleRepository;
use lib::domain::handlers::article_handler::{article_create, article_delete, article_index, article_show, article_update};
use lib::domain::handlers::home_handler::handle_home;
use lib::domain::services::article_service::ArticleService;
use lib::infrastructure::app_state::AppState;

#[tokio::main]
async fn main() {
  println!("Hello, world!");

  let article_repository = Box::new(ArticleRepository::new());
  let article_service = Arc::new(ArticleService::new(article_repository));
  let app_state = Arc::new(AppState::new(article_service));

  let app = Router::new()
    .route("/", get(handle_home))
    .route("/articles", get(article_index))
    .route("/articles", post(article_create))
    .route("/articles/:id", get(article_show))
    .route("/articles/:id", put(article_update))
    .route("/articles/:id", delete(article_delete))
    .with_state(app_state);

  let listener = TcpListener::bind("0.0.0.0:3333")
    .await;

  let listener = match listener {
    Ok(listener) => listener,
    Err(error) => panic!("{}", error)
  };

  serve(listener, app).await.unwrap()
}
