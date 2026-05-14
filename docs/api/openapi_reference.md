# API Reference (For Advanced Users)

*Note: This section is for developers who want to connect custom software to OneHumanCorp. You do not need to read this to use the standard OHC app!*

## Overview

The OneHumanCorp API is organized around REST. Our API has predictable resource-oriented URLs, accepts form-encoded request bodies, returns JSON-encoded responses, and uses standard HTTP response codes, authentication, and verbs.

## The Interactive Swagger UI

We provide an interactive OpenAPI (Swagger) interface where you can browse all available endpoints, view schema definitions, and execute test requests directly against your Sandbox environment.

**Accessing the Sandbox API Explorer:**
1. Navigate to the **Developer Tools** section in your OHC dashboard.
2. Generate a Sandbox API Key.
3. Click the **Launch API Explorer** button to open the Swagger UI.

## Authentication

The OHC API uses Bearer tokens to authenticate requests. You must include the token in the `Authorization` header of your HTTP requests.

```http
Authorization: Bearer ohc_prod_live_abc123def456
```

Never share your secret API keys. Keep them guarded and out of public repositories.

## Rate Limits

To prevent abuse and ensure stability, the API is rate-limited:
- **Sandbox Environment:** 100 requests per minute per IP.
- **Production Environment:** 1000 requests per minute per API key.

If you exceed the rate limit, the API will return a `429 Too Many Requests` response.

## Webhooks

Instead of constantly polling the API to see if a customer bought something, you can use Webhooks. OHC will send an HTTP `POST` payload to your configured URL whenever a specified event occurs.

**Supported Events:**
- `order.created`
- `order.refunded`
- `product.updated`
- `agent.conversation.started`

Webhooks are signed using an HMAC SHA-256 hash. You should verify this signature in your backend to ensure the request genuinely originated from OneHumanCorp.
