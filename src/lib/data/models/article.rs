use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Article {
  pub id: u32,
  pub title: String,
  pub content: String
}

impl Article {
  pub fn new(id: u32, title: String, content: String) -> Article {
    Article { id, title, content }
  }
}
