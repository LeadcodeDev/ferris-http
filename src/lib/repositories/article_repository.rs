use axum::Json;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::models::article::Article;
use crate::validators::article_validator::{CreateArticleValidator, UpdateArticleValidator};

pub trait ArticleRepositoryContract: Debug {
  fn get_list(&self) -> Vec<Article>;
  fn get_one(&self, id: u32) -> Option<Article>;
  fn create(&self, payload: CreateArticleValidator) -> Article;
  fn update(&self, id: u32, payload: UpdateArticleValidator) -> Article;
  fn delete(&self, id: u32);
}

#[derive(Clone, Debug)]
pub struct ArticleRepository {
  articles: Arc<Mutex<Vec<Article>>>,
}

impl ArticleRepositoryContract for ArticleRepository {
  fn get_list(&self) -> Vec<Article> {
    let articles = self.articles.lock().unwrap();
    articles.clone()
  }

  fn get_one(&self, id: u32) -> Option<Article> {
    let articles = self.articles.lock().unwrap();
    articles.iter()
      .find(|element| element.id == id)
      .cloned()
  }

  fn create(&self, payload: CreateArticleValidator) -> Article {
    let mut articles = self.articles.lock().unwrap();
    let serialized = Json(&payload);

    let article = Article::new(
      serialized.id,
      serialized.title.clone(),
      serialized.content.clone()
    );

    articles.push(article.clone());

    article
  }

  fn update(&self, id: u32, payload: UpdateArticleValidator) -> Article {
    let mut articles = self.articles.lock().unwrap();
    let index = articles.iter().position(|element| element.id == id).unwrap();

    articles.remove(index);

    let serialized = Json(&payload);
    let article = Article::new(
      serialized.id,
      serialized.title.clone(),
      serialized.content.clone()
    );

    articles.push(article.clone());
    article
  }

  fn delete(&self, id: u32) {
    let mut articles = self.articles.lock().unwrap();

    let index = articles.iter().position(|article| article.id == id).unwrap();
    articles.remove(index);
  }
}

impl ArticleRepository {
  pub fn new() -> ArticleRepository {
    ArticleRepository {
      articles: Arc::new(Mutex::new(Vec::new())),
    }
  }
}
