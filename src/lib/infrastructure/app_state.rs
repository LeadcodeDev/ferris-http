use std::sync::Arc;
use crate::domain::services::article_service::ArticleService;

#[derive(Debug, Clone)]
pub struct AppState {
  pub article_service: Arc<Box<ArticleService>>
}

impl AppState {
  pub fn new(article_service: Arc<Box<ArticleService>>) -> AppState {
    AppState {
      article_service: Arc::clone(&article_service)
    }
  }
}
