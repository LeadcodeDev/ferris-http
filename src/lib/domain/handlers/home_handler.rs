use axum::extract::Path;

pub async fn handle_home(Path(article_id): Path<u32>) {
  println!("Home request with article {}", article_id);
}
