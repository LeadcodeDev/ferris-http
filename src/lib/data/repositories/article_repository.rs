use crate::data::models::article::Article;
use crate::data::validators::article_validator::{CreateArticleValidator, UpdateArticleValidator};
use axum::Json;
use std::fmt::Debug;
use std::sync::MutexGuard;

pub trait ArticleRepositoryContract: Debug {
  fn get_list(&self);
  fn get_one(&self, articles: MutexGuard<Vec<Article>>, id: u32) -> Option<Article>;
  fn create(&self, payload: CreateArticleValidator) -> Article;
  fn update(&self, id: u32, payload: UpdateArticleValidator) -> Article;
  fn delete(&self, id: u32);
}

#[derive(Clone, Debug)]
pub struct ArticleRepository;

impl ArticleRepositoryContract for ArticleRepository {
  fn get_list(&self) {}

  fn get_one(&self, articles: MutexGuard<Vec<Article>>, id: u32) -> Option<Article> {
    articles.iter()
      .find(|element| element.id == id)
      .cloned()
  }

  fn create(&self, payload: CreateArticleValidator) -> Article {
    let serialized = Json(&payload);
    Article::new(
      serialized.id,
      serialized.title.clone(),
      serialized.content.clone()
    )
  }

  fn update(&self, _id: u32, payload: UpdateArticleValidator) -> Article {
    let serialized = Json(&payload);
    Article::new(
      serialized.id,
      serialized.title.clone(),
      serialized.content.clone()
    )
  }

  fn delete(&self, _id: u32) {}
}

impl ArticleRepository {
  pub fn new() -> ArticleRepository {
    ArticleRepository {}
  }
}
