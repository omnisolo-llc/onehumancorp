# CDN Integration Pattern

The OneHumanCorp (OHC) platform utilizes an edge caching mechanism for high-performance storefront delivery. To facilitate this:
- The system employs an `edge_caching_middleware` to simulate CDN functionality locally (for standalone or docker-compose environments).
- The middleware supports caching based on `Cache-Tag` or `Surrogate-Key` HTTP headers.
- When dynamic resources (like product pricing, inventory levels, or SEO metadata) are updated, the backend services invalidate the affected content by emitting specific cache tags using the `invalidate_by_tag` method or publishing to the `cache_invalidation_events` Pub/Sub topic via Redis.

This enables the application to automatically scale its storefront delivery while keeping the edge caches consistent with the central PostgreSQL datastore.
