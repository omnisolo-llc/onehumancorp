pub mod funding_api;

pub fn get_redis_client() -> Option<redis::Client> {
    if let Ok(url) = std::env::var("REDIS_URL") {
        redis::Client::open(url).ok()
    } else {
        redis::Client::open("redis://127.0.0.1/").ok()
    }
}
