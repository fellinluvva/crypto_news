# Cryptocurrency News Aggregator in Rust

A simple yet powerful web service that aggregates the latest cryptocurrency news using the Rust programming language. Users can search by a cryptocurrency name or symbol and view the most recent news from multiple sources.

---

## Objective

Build a web-based service in **Rust** that:
- Accepts cryptocurrency names or symbols (e.g., `btc`, `eth`)
- Fetches recent news from multiple public APIs
- Displays the results in a clean and structured web interface

---

## Tech Stack

| Layer       | Technology Used                                                  |
|------------|------------------------------------------------------------------|
| Backend     | [Rust](https://www.rust-lang.org/) with `warp`, `reqwest`, `tokio`, `serde`, `serde_json` |
| Frontend    | Basic HTML (served using `warp`)                                |
| APIs Used   | [CoinGecko](https://www.coingecko.com/en/api), [NewsData.io](https://newsdata.io/) |
| (Optional)  | Caching with Redis / Storage with PostgreSQL (Not Implemented Yet) |

---

## ✨ Features

### ✅ Core Features

- **Search by Symbol**  
  Input a symbol (e.g., `btc`) to retrieve relevant news.

- **Multiple API Integration**  
  News and data are fetched from:
  - [CoinGecko](https://www.coingecko.com/en/api) for metadata.
  - [NewsData.io](https://newsdata.io/) for real-time articles.

- 📰 **Structured News Display**  
  News articles include:
  - Title
  - Source
  - Date
  - Summary
  - Link to the full article

- **Error Handling**  
  Gracefully handles API errors, invalid inputs, and unavailable data.
