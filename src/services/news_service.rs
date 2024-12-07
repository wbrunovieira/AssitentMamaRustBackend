use reqwest::Client;
use std::env;
use crate::models::news::{GNewsResponse, Article};

pub async fn get_top_headlines() -> Result<Vec<Article>, String> {
    let api_token = env::var("GNEWS_API").map_err(|_| "GNEWS_API token não encontrado".to_string())?;
    let url = format!("https://gnews.io/api/v4/top-headlines?lang=pt&country=br&token={}", api_token);

    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|err| format!("Erro ao consultar a API de notícias: {}", err))?;

    if !response.status().is_success() {
        return Err(format!("Falha ao obter notícias: HTTP {}", response.status()));
    }

    let data: GNewsResponse = response
        .json()
        .await
        .map_err(|err| format!("Falha ao parsear resposta de notícias: {}", err))?;

    Ok(data.articles)
}

pub fn format_news_articles(articles: &[Article]) -> String {
    if articles.is_empty() {
        return "Não há notícias disponíveis no momento.".to_string();
    }

    let mut formatted = String::new();
    for (i, article) in articles.iter().take(3).enumerate() {
        formatted.push_str(&format!("{}. {}\n", i + 1, article.title));
    }

    formatted
}
