<<<<<<< SEARCH
        .nest("/api/sync", api::sync_handler::router(db.pool.clone(), store.clone()))
        .nest("/api/agents", api::agents::hire::router(hub.clone()))
=======
        .nest("/api/sync", api::sync_handler::router(db.pool.clone(), std::sync::Arc::new(crate::auth::Store::new())))
        .nest("/api/agents", api::agents::hire::router(hub.clone()))
>>>>>>> REPLACE
