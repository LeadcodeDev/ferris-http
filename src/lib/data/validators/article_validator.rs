use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct CreateArticleValidator {
  pub id: u32,
  pub title: String,
  pub content: String
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct UpdateArticleValidator {
  pub id: u32,
  pub title: String,
  pub content: String
}
