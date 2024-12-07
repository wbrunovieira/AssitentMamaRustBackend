use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GNewsResponse {
    #[serde(rename = "totalArticles")]
    pub total_articles: u32,
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
  
}
