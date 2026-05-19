# Scout Tool Integration Research Final Report

This report summarizes the research for integrating various tools into the OHC platform to address critical business owner pain points.

## Tools Evaluated

1.  **ManyChat** (Social Media Integration): Unified Inbox
2.  **Cal.com** (Calendar & Scheduling): Booking Sync
3.  **Resend** (Email Marketing): Automated Campaigns
4.  **Mercado Pago** (Payment Processing): Localized Payments
5.  **Shippo** (Shipping & Logistics): Label Generation
6.  **Twilio** (SMS & Notifications): Global Messaging
7.  **Daily.co** (Video Conferencing): Auto-Meeting Links

---

### [Social Media Integration] Unified Inbox with ManyChat
**Priority: P0 | Estimated Scope: Large | Capability: Cloud-first**
Provides a unified view for Instagram, Facebook, and WhatsApp DMs, with an AI agent to handle basic questions. Abstracted behind OHC UI using ManyChat's API via OAuth.

### [Calendar & Scheduling] Booking Sync with Cal.com
**Priority: P0 | Estimated Scope: Medium | Capability: Cloud/Standalone**
Open-source API-first scheduling allowing customers to book time without double-booking over personal events. Cal.com sub-accounts are provisioned per tenant.

### [Email Marketing] Automated Campaigns with Resend
**Priority: P1 | Estimated Scope: Medium | Capability: Cloud**
Developer-focused, fast email platform for sending beautifully designed announcements. The Marketing agent drafts content and Resend delivers it, hiding complexity from the business owner.

### [Payment Processing] Localized Payments with Mercado Pago
**Priority: P1 | Estimated Scope: Medium | Capability: Cloud/Standalone**
Alternative gateway for LATAM customers offering familiar methods like Pix and OXXO, improving checkout conversion rates in those regions.

### [Shipping & Logistics] Label Generation with Shippo
**Priority: P2 | Estimated Scope: Large | Capability: Cloud-first**
Simplifies label purchasing across multiple carriers directly within the OHC order management UI, calculating rates instantly and generating tracking links.

### [SMS & Notifications] Global Messaging with Twilio
**Priority: P2 | Estimated Scope: Small | Capability: Cloud**
Ensures robust mobile notification delivery via SMS for critical events (like order updates), improving reliability over push notifications.

### [Video Conferencing] Auto-Meeting Links with Daily.co
**Priority: P3 | Estimated Scope: Medium | Capability: Cloud**
Automatically generates and embeds video call rooms for virtual bookings. Tutors and students meet natively inside the OHC ecosystem without needing Zoom.

---
Detailed markdown research briefs including problem statements, comparative tables, persona pain points, actionable recommendations, Mermaid.js architecture charts, wireframes, and implementation prompts are stored in `docs/research/`.

## Mission Blockers & Missing Components
- **Missing Go Backend:** The implementation prompts request Go backend services, but the repository is entirely a Rust-based backend (Axum).
- **Missing Flutter Frontend:** The implementation prompts request Flutter UI components, but there is no Flutter application source code in the repository.
- **Missing Database:** PostgreSQL is not available to update the `agent_missions` table for a formal mission handover.

As per the protocol for "Scout Task Output" tasks involving missing databases or lacking concrete coding objectives, we are documenting these missing components and blockers here rather than creating dummy migrations, Go files, or placeholder Flutter code.
