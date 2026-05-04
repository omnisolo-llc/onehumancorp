# OHC Integration Scout Report: Core SMB Tools [Q2]

## Executive Summary
This report evaluates seven critical tool integrations designed to empower non-technical small business owners using OneHumanCorp (OHC). Our research focuses on solving real-world friction points across communication, scheduling, marketing, payments, and logistics. Each integration was evaluated strictly through the lens of our core personas (Maya, Carlos, Priya, Leo, and Fatima) to ensure radical simplicity and zero technical overhead.

---

## 1. Persona Pain Point Matrix

The following table maps the identified pain points of our personas to the proposed integrations:

| Persona | Business Type | Critical Pain Point | Solution Integration |
|---------|---------------|---------------------|----------------------|
| **Maya** | Home Baker | Losing sales because she can't reply to Instagram DMs instantly. | **Meta Graph API** (Unified Inbox + AI Drafts) |
| **Leo** | Music Tutor | Manual back-and-forth emails to schedule lessons. | **Cal.com** (Automated Scheduling) |
| **Priya** | Boutique Owner | Wants to announce new products but Mailchimp is too hard. | **Resend** (AI-driven Email Campaigns) |
| **Carlos** | Handyman | Needs to accept local payment methods like PIX/OXXO. | **Mercado Pago** (LATAM Payment Gateway) |
| **Priya / Maya** | Goods Sellers | Calculating shipping rates and buying post office labels takes too long. | **Shippo** (Automated Labels & Rates) |
| **Fatima** | Food Cart | Needs instant offline alerts for new orders. | **Twilio** (SMS Notifications) |
| **Leo** | Online Tutor | Students struggle with Zoom software downloads. | **Daily.co** (Embedded WebRTC Video) |

---

## 2. Integration Architecture Overview

The following diagram illustrates how these tools integrate into OHC's Multi-Agent OS architecture:

```mermaid
graph TD
    subgraph OHC "OneHumanCorp Platform"
        UI[Frontend UI / Unified Dashboard]

        subgraph Agents "AI Departments"
            Ops[Operations Agent]
            Sales[Sales Agent]
            Mkt[Marketing Agent]
            CS[Customer Success Agent]
            Fin[Finance Agent]
        end

        DB[(Tenant PostgreSQL)]
    end

    subgraph Integrations "External Providers"
        Meta[Meta Graph API]
        Cal[Cal.com]
        Resend[Resend]
        MP[Mercado Pago]
        Shippo[Shippo]
        Twilio[Twilio]
        Daily[Daily.co]
    end

    %% Routing
    UI --> Agents
    Agents --> DB

    CS <--> Meta : IG/FB Messages
    Ops <--> Cal : Calendar Sync
    Mkt --> Resend : Email Campaigns
    Fin <--> MP : Local Payments
    Ops <--> Shippo : Shipping Labels
    Ops --> Twilio : SMS Alerts
    Ops <--> Daily : Embedded Video
```

---

## 3. Comparative Tool Analysis

| Category | Recommended Tool | Alternative Considered | Why We Chose It | Pricing Model | Complexity for OHC Eng |
|----------|------------------|------------------------|-----------------|---------------|------------------------|
| **Social** | Meta Graph API | Chatwoot (Self-Hosted) | Direct API access allows deep AI drafting integration without an intermediary UI. | Free | High (OAuth/Webhooks) |
| **Calendar** | Cal.com | Cron/Google API Direct | Handles timezone math and conflict resolution natively. Open-source friendly. | API Pricing | Medium |
| **Email** | Resend | SendGrid / Mailchimp | Developer-first API, extremely fast, excellent React Email support. | Freemium / Pay-as-you-go | Medium |
| **Payments** | Mercado Pago | dLocal | Dominant market share in LATAM, highly recognizable to local buyers. | Per-transaction fee | High |
| **Logistics**| Shippo | EasyPost | Easier API abstraction for multi-carrier global shipping. | Per-label fee | Medium |
| **SMS** | Twilio | MessageBird | Industry standard reliability, easiest number provisioning via API. | Pay-as-you-go | Low |
| **Video** | Daily.co | Zoom API | Embedded WebRTC means no app downloads for the end customer. | Freemium | Low |

---

## 4. Actionable Recommendations & Implementation Strategy

Based on this research, we recommend the following execution strategy:

### Phase 1: High Impact, Core Revenue Blockers (P0)
1. **Cal.com Integration**: Leo and Carlos cannot operate effectively without scheduling. Implementing this unlocks the entire "Services & Bookings" vertical.
2. **Meta Graph API**: Maya's biggest pain point is missed DMs. Solving this proves the value of the "Customer Success" agent immediately.

### Phase 2: Operations & Growth (P1)
1. **Twilio SMS**: Critical for Fatima's food cart operations and general reliability for non-email-centric businesses.
2. **Shippo**: Unlocks the physical product vertical for users like Priya and Maya, significantly reducing their manual labor.
3. **Resend**: Empowers the Marketing agent to drive repeat business.

### Phase 3: Regional & Niche Expansions (P2)
1. **Mercado Pago**: Expanding LATAM market penetration.
2. **Daily.co**: Enhancing the online service experience, removing the friction of external video clients.

### Technical Guidelines for Implementers
- **Zero Configuration**: Users must never see API keys. Use OAuth flows or platform-managed credentials.
- **Agent First**: Expose these tools as capabilities to the AI agents (via the Hub/Mesh), rather than just dumb UI buttons. The AI should drive the tool usage based on context.
- **Resilience**: Assume webhooks will drop and third-party APIs will timeout. Implement aggressive retry queues and optimistic UI updates for all integrations.
