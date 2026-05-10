# OHC Tool Integration Research Report (Q2 2026)

## Overview
This report details the evaluation of five key external tool integrations to expand One Human Corp (OHC)'s capabilities for non-technical small business owners. Each integration has been evaluated based on its ability to solve real-world pain points, user experience, pricing, and compatibility with OHC's Hybrid Architecture (Cloud and Standalone).

All evaluations strictly adhere to the **Visual Excellence Mandate** and are designed from the perspective of a single human CEO managing their business.

---

## Evaluated Tools

### 1. WhatsApp Business API (Social Media Integration)
**Pain Point:** Owners miss orders because they juggle personal WhatsApp, Instagram, and SMS.
- **Solution:** A unified inbox within OHC for WhatsApp messages.
- **Key Advantages:** High user engagement, centralizes communication.
- **Pricing:** ~$0.01 - $0.08 per service conversation after the first 1,000 free.
- **Environment:** Compatible with Cloud and Standalone (Webhook based).
- **Priority:** P0

### 2. Google Calendar Sync (Calendar & Scheduling)
**Pain Point:** Double-booking and the manual overhead of scheduling client appointments.
- **Solution:** Automated two-way sync between OHC bookings and Google Calendar.
- **Key Advantages:** Highly trusted, totally automated post-setup.
- **Pricing:** API access is generally free for typical SMB volumes.
- **Environment:** Compatible with Cloud and Standalone.
- **Priority:** P1

### 3. Mercado Pago (Payment Processing)
**Pain Point:** Stripe is often unavailable or too expensive in LATAM markets. Customers want local options like PIX.
- **Solution:** Checkout integration with Mercado Pago.
- **Key Advantages:** Dominant LATAM presence, supports local currencies.
- **Pricing:** 3% - 5% per transaction depending on settlement speed.
- **Environment:** Compatible with Cloud and Standalone.
- **Priority:** P2

### 4. Twilio (SMS & Notifications)
**Pain Point:** Email notifications go unread by customers, leading to appointment no-shows.
- **Solution:** Automated SMS reminders for bookings and updates.
- **Key Advantages:** Near-instant delivery, high open rates.
- **Pricing:** ~$0.0079 per SMS (US).
- **Environment:** Compatible with Cloud and Standalone.
- **Priority:** P1

### 5. Zoom (Video Conferencing)
**Pain Point:** Manually creating and emailing video links for online services is error-prone and tedious.
- **Solution:** Auto-generate Zoom links upon service booking.
- **Key Advantages:** Industry standard, eliminates manual data entry.
- **Pricing:** Requires a Zoom Pro account (~$15.99/mo).
- **Environment:** Compatible with Cloud and Standalone.
- **Priority:** P2

---

## Architectural Synthesis (Hybrid RAG & Event Flow)

The integration of these external providers utilizes OHC's backend as a unified secure mediator.

```mermaid
graph TD
    Client(End Customer) -->|Interacts| External[External Provider: WhatsApp/Zoom/MP]
    External -->|Webhooks & APIs| API[OHC Backend API]
    API -->|Synchronizes State| SyncEngine{Sync Engine}
    SyncEngine -->|Stores Data| CloudDB[(PostgreSQL - Cloud)]
    SyncEngine -.->|OHC-SIP Sync| LocalDB[(SQLite - Standalone)]
    API -->|Real-time Updates| OHC_UI[OHC App Interface]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class API,SyncEngine,CloudDB,LocalDB,OHC_UI premium;
```

## Next Steps
Detailed issue briefs for each integration have been generated in the `docs/research/` directory. Implementers should pull these briefs, review the required outcomes, and begin technical design mapping for API endpoints and database schemas.
