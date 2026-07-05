# Universal Edge-Cached Dynamic Storefront & SEO Architecture

## Overview
OneHumanCorp (OHC) platform relies on a high-performance serving layer that delivers instant page loads and strong SEO metrics, leveraging edge caching architectures such as Cloudflare or Varnish. This document outlines the CDN integration pattern and how cache invalidations are automatically triggered when tenants update their products, storefronts, or settings.

## Edge Caching Mechanism
The OHC backend is designed to interact natively with edge CDNs. The dynamic storefront endpoints (e.g. `/api/v1/storefront/{tenant_id}/{product_id}` or the builder routes) pre-render HTML and SEO metadata server-side and instruct the edge CDN to cache the result.

### Cache Headers
Responses destined for the edge cache are tagged with the following headers:
- `Cache-Control: public, s-maxage=60, stale-while-revalidate=86400`
  - Instructs the CDN to cache the response for 60 seconds and serve stale content for up to a day while revalidating in the background.
- `ETag`
  - A hash of the content to allow the CDN to check for changes and reduce payload size on 304 Not Modified.
- `Surrogate-Key` (or `Cache-Tag`)
  - A space- or comma-separated list of tags associated with the response. Typical tags include:
    - `tenant-id:{tenant_id}`: Marks the response as belonging to a specific tenant.
    - `entity:product:{product_id}`: Marks the response as depending on a specific product's data.

## Cache Invalidation
When inventory updates, products are modified, or tenant settings change, the central control plane (the `ohc-core` app servers) triggers a targeted cache purge.

1. **Database Mutation:** The core API endpoints (such as `offline_sync`, `pos`, `growth`, and `billing_webhook`) execute `UPDATE products` or `UPDATE tenants` commands in PostgreSQL.
2. **Pub/Sub Notification:** Immediately after the transaction, the API publishes a JSON payload to the Redis Pub/Sub topic `cache_invalidation_events`. The payload looks like this:
   ```json
   {
       "event": "inventory.updated",
       "tags": [
           "tenant-id:1234",
           "entity:product:5678"
       ]
   }
   ```
3. **Invalidator Service:** The internal `Cache Invalidator Service` (in `ohc-core`) listens to the `cache_invalidation_events` topic. It purges the tags from the internal `CDN_CACHE` and/or dispatches HTTP `PURGE` requests to the external edge CDN (like Cloudflare or Varnish) to instantly invalidate the corresponding `Surrogate-Key`.

## Cloudflare Integration
For Cloudflare, the `Surrogate-Key` translates to Cloudflare's `Cache-Tag`.
You can configure a Cloudflare Worker or Page Rule to respect `s-maxage` and `Cache-Tag`. When an event occurs, the internal system can hit the Cloudflare API to purge by tag:
```bash
curl -X POST "https://api.cloudflare.com/client/v4/zones/{zone_id}/purge_cache" \
     -H "Authorization: Bearer {cloudflare_token}" \
     -H "Content-Type: application/json" \
     --data '{"tags":["entity:product:5678"]}'
```

## Varnish Integration
For Varnish, you configure the VCL to store `Surrogate-Key` values, and process incoming HTTP `BAN` or `PURGE` requests matching those keys using `obj.http.Surrogate-Key`.

By adopting this edge-caching model, OHC storefronts enjoy sub-100ms TTFB globally and ensure search engines index the complete, fully rendered SEO content.
