use crate::models::article::Article;
use crate::repositories::article_repository::ArticleRepositoryContract;
use crate::validators::article_validator::{CreateArticleValidator, UpdateArticleValidator};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ArticleService {
  articles: Arc<Mutex<Vec<Article>>>,
  pub article_repository: Box<dyn ArticleRepositoryContract + Send + Sync>
}

impl ArticleService {
  pub fn new(article_repository: Box<dyn ArticleRepositoryContract + Send + Sync>) -> Box<ArticleService> {
    Box::new(ArticleService {
      articles: Arc::new(Mutex::new(Vec::new())),
      article_repository
    })
  }

  pub fn index(&self) -> Vec<Article> {
    let articles = self.articles.lock().unwrap();
    articles.clone()
  }

  pub fn get(&self, id: u32) -> Option<Article> {
    let articles = self.articles.lock().unwrap();
    self.article_repository.get_one(articles, id)
  }

  pub fn create(&self, payload: CreateArticleValidator) -> Article {
    let mut articles = self.articles.lock().unwrap();
    let article = self.article_repository.create(payload);

    articles.push(article.clone());

    article
  }

  pub fn update(&self, id: u32, payload: UpdateArticleValidator) -> Article {
    let mut articles = self.articles.lock().unwrap();
    let index = articles.iter().position(|element| element.id == id).unwrap();

    articles.remove(index);

    let article = self.article_repository.update(id, payload);
    articles.push(article.clone());

    article
  }

  pub fn delete(&self, id: &u32) {
    let mut articles = self.articles.lock().unwrap();

    let index = articles.iter().position(|article| &article.id == id).unwrap();
    articles.remove(index);
  }
}

