use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub link: String,
}


pub async fn fetch_news(crypto: &str) -> Result<Vec<Article>, reqwest::Error> {
    let client = Client::new();
    let mut articles = Vec::new();

    // ─── CoinGecko (General Info Only) ───
    let gecko_key = env::var("COINGECKO_API_KEY").unwrap_or_default();
    let gecko_url = format!("https://pro-api.coingecko.com/api/v3/coins/{}", crypto);
    println!("🔍 CoinGecko URL: {}", gecko_url);

    let gecko_resp = client
        .get(&gecko_url)
        .header("x-cg-pro-api-key", gecko_key)
        .send()
        .await?;

    let gecko_json: Value = gecko_resp.json().await?;
    if let Some(description) = gecko_json["description"]["en"].as_str() {
        articles.push(Article {
            title: format!("About {}", crypto.to_uppercase()),
            description: description.to_string(),
            link: gecko_json["links"]["homepage"][0].as_str().unwrap_or_default().to_string(),
        });
    }

    // ─── NewsData (Regular News Endpoint) ───
    let newsdata_key = env::var("NEWSDATA_API_KEY").unwrap_or_default();
    let newsdata_url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en",
        newsdata_key, crypto
    );
    println!("🔍 NewsData URL: {}", newsdata_url);

    let newsdata_resp = client.get(&newsdata_url).send().await?;
    let newsdata_json: Value = newsdata_resp.json().await?;
    println!("📰 NewsData Response: {:#?}", newsdata_json);

    if let Some(results) = newsdata_json["results"].as_array() {
        for item in results {
            let title = item["title"].as_str().unwrap_or_default().to_string();
            let description = item["description"].as_str().unwrap_or("").to_string();
            let link = item["link"].as_str().unwrap_or("").to_string();
            articles.push(Article { title, description, link });
        }
    }

    Ok(articles)
}
