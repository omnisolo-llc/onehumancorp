# Research Report: Core Business Integrations (Meta & Mercado Pago)

## 1. Unified Inbox & Social Commerce: Meta Integration (Instagram / WhatsApp)
**Problem Statement:** Business owners like Maya (The Home Baker) rely heavily on Instagram DMs and WhatsApp for customer acquisition and order placement. Currently, OHC lacks a unified inbox connection to these platforms, forcing owners to switch contexts and losing AI capabilities.
**Research Report:** Meta provides Graph API for Instagram Direct Messages and WhatsApp Business Platform API. Integrating these into the OHC Unified Inbox will allow the "Customer Success" and "Operations" AI agents to interact directly with customers on their preferred platforms.
**Design Doc:**
- Connect Instagram/Facebook/WhatsApp Business accounts via Meta OAuth.
- Implement Webhooks to receive incoming messages into the OHC unified inbox.
- OHC Backend uses Graph API/WhatsApp API to send replies.
- AI agents can be configured to auto-draft or auto-reply.
**Implementation Prompt:** Implement Meta (Instagram and WhatsApp) integration in `src/server/integrations/meta`. Provide the necessary client and provider structures to handle incoming webhooks and sending messages.
**Priority:** P0
**Estimated Scope:** Large

## 2. LATAM Payment Processing: Mercado Pago Integration
**Problem Statement:** To effectively serve the LATAM market, OHC needs to support local payment methods like PIX, OXXO, and local installments, which Stripe does not fully cover in all regions.
**Research Report:** Mercado Pago is the dominant processor in LATAM. Integrating their API (Checkout Pro or direct API) will allow LATAM-based users to accept these local payments seamlessly.
**Design Doc:**
- Implement Mercado Pago integration alongside Stripe in `src/server/integrations/mercadopago`.
- Handle payment creation, retrieval, and IPN (Instant Payment Notification) webhooks.
**Implementation Prompt:** Add Mercado Pago integration. Create the client and provider to initiate payments and handle webhooks for payment status updates.
**Priority:** P2
**Estimated Scope:** Medium
