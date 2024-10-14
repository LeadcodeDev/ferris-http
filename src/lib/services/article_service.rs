use crate::handlers::article_handler::UpdateArticleValidator;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ArticleService {
  pub articles: Arc<Mutex<Vec<String>>>
}

impl ArticleService {
  pub fn new() -> ArticleService {
    ArticleService {
      articles: Arc::new(Mutex::new(Vec::new()))
    }
  }

  pub fn get(&self, value: &String) -> Option<String> {
    let mut articles = self.articles.lock().unwrap();
    articles.iter()
      .find(|element| element.clone() == value)
      .cloned()
  }

  pub fn add(&self, value: String) {
    let mut articles = self.articles.lock().unwrap();
    articles.push(value);
  }

  pub fn update(&self, id: &String, payload: UpdateArticleValidator) {
    let mut articles = self.articles.lock().unwrap();

    let index = articles.iter().position(|element| element == id).unwrap();
    println!("{}", index);

    articles.remove(index);
    articles.push(payload.label.unwrap());
  }

  pub fn delete(&self, id: &String) {
    let mut articles = self.articles.lock().unwrap();

    let index = articles.iter().position(|element| element == id).unwrap();
    articles.remove(index);
  }
}

