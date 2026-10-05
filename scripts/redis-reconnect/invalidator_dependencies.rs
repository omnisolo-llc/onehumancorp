// Inert dependencies for the exact invalidator loop. No events are published
// in the reconnect contract; any accidental SQL or HTTP attempt fails loudly.
mod sqlx {
    #[derive(Clone)]
    pub struct PgPool;
    pub struct Query<DB, T>(std::marker::PhantomData<(DB, T)>);
    pub fn query_scalar<DB, T>(_sql: &str) -> Query<DB, T> {
        let _: std::marker::PhantomData<DB> = std::marker::PhantomData;
        Query(std::marker::PhantomData)
    }
    impl<T> Query<(), T> {
        pub fn bind(self, _value: uuid::Uuid) -> Self {
            self
        }
        pub async fn fetch_one(self, _pool: &PgPool) -> Result<T, ()> {
            panic!("no SQL is allowed in the subscription transport fixture")
        }
    }
}
mod reqwest {
    #[derive(Clone)]
    pub struct Client;
    pub struct Request;
    impl Client {
        pub fn new() -> Self {
            Self
        }
        pub fn post(&self, _url: &str) -> Request {
            panic!("no HTTP is allowed in the subscription transport fixture")
        }
    }
    impl Request {
        pub fn body(self, _body: String) -> Self {
            self
        }
        pub async fn send(self) -> Result<(), std::io::Error> {
            panic!("no HTTP is allowed in the subscription transport fixture")
        }
    }
}
mod builder {
    pub mod edge {
        pub fn get_edge_cache() -> crate::cache::HybridCache<String> {
            static CACHE: std::sync::OnceLock<crate::cache::HybridCache<String>> =
                std::sync::OnceLock::new();
            CACHE
                .get_or_init(|| crate::cache::HybridCache::new(None))
                .clone()
        }
        pub async fn regenerate_product_cache(
            _pool: crate::sqlx::PgPool,
            _tenant: uuid::Uuid,
            _product: uuid::Uuid,
            _key: String,
            _cache: crate::cache::HybridCache<String>,
        ) -> Result<(), ()> {
            panic!("no product regeneration is allowed in the subscription transport fixture")
        }
    }
}
mod utils {
    pub mod edge_caching_middleware {
        pub fn get_cdn_cache() -> crate::cache::HybridCache<String> {
            static CACHE: std::sync::OnceLock<crate::cache::HybridCache<String>> =
                std::sync::OnceLock::new();
            CACHE
                .get_or_init(|| crate::cache::HybridCache::new(None))
                .clone()
        }
    }
}
mod invalidator {
    use crate::{reqwest, sqlx};
    include!(concat!(env!("OUT_DIR"), "/cache_invalidator.rs"));
}
