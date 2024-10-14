use crate::handlers::article_handler::UpdateArticleValidator;
use std::sync::{Arc, Mutex};
use axum::Json;
use crate::models::article::Article;

#[derive(Debug, Clone)]
pub struct ArticleService {
  pub articles: Arc<Mutex<Vec<Article>>>
}

impl ArticleService {
  pub fn new() -> ArticleService {
    ArticleService {
      articles: Arc::new(Mutex::new(Vec::new()))
    }
  }

  pub fn get(&self, value: &u32) -> Option<Article> {
    let articles = self.articles.lock().unwrap();
    articles.iter()
      .find(|element| &element.id == value)
      .cloned()
  }

  pub fn add(&self, article: Article) {
    let mut articles = self.articles.lock().unwrap();
    articles.push(article);
  }

  pub fn update(&self, id: u32, payload: UpdateArticleValidator) {
    let mut articles = self.articles.lock().unwrap();

    let index = articles.iter().position(|element| element.id == id).unwrap();

    articles.remove(index);

    let serialized = Json(&payload);
    let article = Article::new(
       payload.id,
       serialized.title.clone(),
       serialized.content.clone()
    );

    articles.push(article);
  }

  pub fn delete(&self, id: &u32) {
    let mut articles = self.articles.lock().unwrap();

    let index = articles.iter().position(|article| &article.id == id).unwrap();
    articles.remove(index);
  }
}

