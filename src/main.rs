use warp::{Filter, Rejection, Reply};
mod api;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Define the route with proper error type handling
    let news_route = warp::path!("news" / String)
        .and_then(news_handler);

    // Serve static files
    let static_files = warp::fs::dir("static");

    // Enable CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("GET")
        .allow_header("content-type");

    let routes = news_route.with(cors).or(static_files);

    println!("Server running at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Custom handler with explicit Result types
async fn news_handler(symbol: String) -> Result<impl Reply, Rejection> {
    match api::fetch_news(&symbol).await {
        Ok(news) => {
            let reply = warp::reply::json(&news);
            Ok(warp::reply::with_status(reply, warp::http::StatusCode::OK))
        }
        Err(_) => {
            let error_json = warp::reply::json(&serde_json::json!({
                "error": "Failed to fetch news"
            }));
            Ok(warp::reply::with_status(error_json, warp::http::StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

