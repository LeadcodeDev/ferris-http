use crate::data::models::article::Article;
use crate::data::repositories::article_repository::ArticleRepositoryContract;
use crate::data::validators::article_validator::{CreateArticleValidator, UpdateArticleValidator};

#[derive(Debug)]
pub struct ArticleService {
  pub article_repository: Box<dyn ArticleRepositoryContract + Send + Sync>
}

impl ArticleService {
  pub fn new(article_repository: Box<dyn ArticleRepositoryContract + Send + Sync>) -> Box<ArticleService> {
    Box::new(ArticleService {
      article_repository
    })
  }

  pub fn index(&self) -> Vec<Article> {
    self.article_repository.get_list()
  }

  pub fn get(&self, id: u32) -> Option<Article> {
    self.article_repository.get_one(id)
  }

  pub fn create(&self, payload: CreateArticleValidator) -> Article {
    self.article_repository.create(payload)
  }

  pub fn update(&self, id: u32, payload: UpdateArticleValidator) -> Article {
    self.article_repository.update(id, payload)
  }

  pub fn delete(&self, id: u32) {
    self.article_repository.delete(id)
  }
}

