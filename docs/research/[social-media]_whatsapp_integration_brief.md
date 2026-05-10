# WhatsApp Business API Integration

## Problem Statement
For a non-technical small business owner (like a local bakery or boutique shop), managing customer inquiries across multiple platforms is overwhelming. Customers increasingly prefer messaging over calling or emailing. The owner misses orders or inquiries because they are too busy running the business to constantly check their personal WhatsApp or Instagram. They need a unified inbox where all customer messages arrive in one place, allowing them (or their staff) to respond efficiently without giving out their personal number.

## Research Report
The WhatsApp Business API is the industry standard for connecting WhatsApp to third-party unified inboxes.
- **Benefits for Users:** It centralizes communication, meaning the owner doesn't need to juggle multiple devices. It supports rich media (images for orders) and automated quick replies.
- **Ease of Use:** From the owner's perspective, once integrated into OHC, it feels just like an inbox. They don't need to know what an API is.
- **Reputation:** Meta is the provider. Reliability is high, though account approval processes can sometimes be strict.
- **Pricing:** WhatsApp uses conversation-based pricing. The first 1,000 service conversations each month are typically free. After that, utility/service conversations cost roughly $0.01 to $0.08 per conversation, depending on the user's country.
- **Environment Compatibility:** Works seamlessly in both Cloud (multi-tenant) and Standalone (local) execution modes, as it relies on webhook delivery to the OHC backend.

## Design Doc
```mermaid
graph TD
    User(Small Business Owner) -->|Views Unified Inbox| OHC_UI[OHC App Interface]
    Customer(Customer) -->|Sends Message| WA[WhatsApp]
    WA -->|Webhook via API| OHC_Backend[OHC Backend]
    OHC_Backend -->|Saves Message & Triggers Event| DB[(SIPDB / Postgres)]
    OHC_Backend -->|Pushes Real-time Update| OHC_UI

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class OHC_UI,OHC_Backend,DB premium;
```

When a customer sends a message to the business's WhatsApp number, WhatsApp sends a webhook to the OHC backend. The message is stored in the database and surfaced in the OHC unified inbox UI. When the owner replies from the OHC UI, the backend sends the message out via the WhatsApp API.

## Implementation Prompt
Integrate WhatsApp Business API into the OHC unified inbox.
- **User Outcome:** Business owners should see a "Connect WhatsApp" button in their settings. Once authenticated, any incoming messages to their WhatsApp Business number should appear in their OHC inbox. They must be able to reply directly from OHC.
- **Acceptance Criteria:**
  - Secure webhook endpoint to receive incoming messages.
  - Outbound messaging capability.
  - Support for text and basic media (images).
  - Clean UI integration with the existing unified inbox using standard OHC Glassmorphism design tokens.

## Priority
P0

## Estimated Scope
Medium
