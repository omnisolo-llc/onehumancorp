# API Reference (Advanced Users)

*Warning: This section is for developers. If you do not know what JSON or a REST API is, you do not need this page! You can run your entire business using our normal app.*

## Overview
The One Human Corp API allows you to programmatically access your store's data. You can use this to build custom checkout flows, integrate with specialized inventory software, or pull raw data into your own spreadsheets.

## Authentication
All API requests require a Bearer token.
1. Go to **Settings > Advanced > API Keys**.
2. Click **Generate New Key**.
3. Include this key in the header of your requests: `Authorization: Bearer ohc_live_xxxxxxx`.

*Keep your key secret. Do not put it in front-end Javascript code.*

## Base URL
All API requests should be made to:
`https://api.onehumancorp.com/v1/`

## Core Endpoints

### 1. List Products
`GET /products`
Returns a list of all products in your store. Uses cursor-based pagination.

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "prod_12345",
      "name": "Winter Beanie",
      "price_cents": 1500,
      "inventory_quantity": 42
    }
  ],
  "next_cursor": "cx_98765"
}
```

### 2. Get a Single Order
`GET /orders/{order_id}`
Returns details about a specific customer order.

### 3. Update Inventory
`PATCH /products/{product_id}/inventory`
Programmatically add or subtract from your stock count.

**Request Body:**
```json
{
  "adjustment": -5,
  "reason": "sold_in_person"
}
```

## Rate Limits
To protect the platform, API requests are limited to 100 requests per minute per store. If you exceed this, you will receive a `429 Too Many Requests` response.

For complete, interactive documentation using Swagger UI, please visit [developer.onehumancorp.com](https://developer.onehumancorp.com).

### 4. Create a Product
`POST /products`
Add a new product to your catalog programmatically.

**Request Body:**
```json
{
  "name": "Summer Hat",
  "description": "A nice hat for the beach.",
  "price_cents": 2500,
  "inventory_quantity": 100,
  "category_id": "cat_summer"
}
```

**Response (201 Created):**
```json
{
  "id": "prod_67890",
  "name": "Summer Hat",
  "status": "active"
}
```

### 5. Webhooks
Instead of constantly asking the API if something changed (polling), you can set up Webhooks. We will send an HTTP POST request to your server whenever an important event happens.

**Supported Events:**
- `order.created`: Fires when a customer completes checkout.
- `inventory.depleted`: Fires when a product reaches 0 stock.
- `payout.completed`: Fires when money is successfully transferred to your bank.

**Registering a Webhook:**
`POST /webhooks`
```json
{
  "url": "https://your-server.com/webhooks/ohc",
  "events": ["order.created"]
}
```

**Webhook Security:**
Every webhook payload includes a `X-OHC-Signature` header. You must verify this signature using your API secret to ensure the request actually came from us and not a malicious third party.

## SDKs and Libraries
While you can make raw HTTP requests, we provide official libraries to make integration easier:
- **Node.js:** `npm install @onehumancorp/node`
- **Python:** `pip install onehumancorp`
- **Ruby:** `gem install onehumancorp`

For code examples using these libraries, please consult the full documentation site.

## API Versioning
We use URI versioning for our API. The current version is `v1`.
- When we make breaking changes, we will release a `v2` endpoint and leave `v1` active for at least 12 months.
- We strongly recommend subscribing to our developer newsletter to receive deprecation notices.

## Support
If you need help building your integration, please refer to the advanced documentation or contact our developer relations team at `dev-support@onehumancorp.com`.
