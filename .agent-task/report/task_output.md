# [Architecture] Omni-Channel AI Communications & Agentic Operations Hub

**Priority:** P0
**Estimated Scope:** Large

## Problem Statement

Small business owners on OneHumanCorp are managing multiple fragmented communication channels (Instagram DMs, WhatsApp, SMS, Email, Web Chat) simultaneously. For personas like Maya (the baker selling via IG DMs) and Carlos (the handyman replying to text queries), context switching and delayed response times directly lose them revenue. They cannot afford to sit at a desktop inbox all day, nor do they want to hire a virtual assistant. The platform lacks a unified, intelligent communication layer that seamlessly integrates with business operations (quoting, scheduling, ordering) without manual intervention, while being perfectly native to a 375px mobile screen.

## Research Report

### Findings
- **Market Expectations:** Modern buyers expect response times under 5 minutes on social channels (Instagram, WhatsApp).
- **Competitor Analysis:**
  - **Shopify Inbox:** Solid multi-channel chat but relies heavily on predefined static quick replies. Doesn't possess deep agentic integration (e.g., cannot negotiate a custom cake quote autonomously over IG DMs).
  - **Wix Inbox:** Basic centralized messaging. Requires manual intervention for any transactional process like booking or invoicing.
  - **Intercom / Gorgias:** High capability but overly complex and expensive ($50-$100+/mo) for a micro-business owner. Built for support teams, not solo founders.
- **The Gap:** OHC is missing an integrated, AI-first hub where messages from *all* channels route into a single tenant-isolated inbox. More importantly, we lack the architectural hooks allowing OHC's AI Departments (CS, Sales) to autonomously read, reason about, and respond to these messages with context (inventory, calendar, pricing) natively, while allowing the human owner to seamlessly take over the thread on their mobile device.

## Design Doc

### Architecture Overview

The Omni-Channel AI Communications Hub introduces a centralized, tenant-isolated messaging bus and an event-driven Agentic Gateway.

```mermaid
graph TD;
    subgraph External Channels
        IG[Instagram DM API]
        WA[WhatsApp Business API]
        SMS[Twilio SMS]
        WEB[Web Storefront Chat]
    end

    subgraph OHC Communication Hub
        GW[Omni-Channel Webhook Gateway]
        Ingest[Message Ingestion & Normalization]
        Ledger[(Unified Conversation Ledger)]
        PubSub[Tenant-Isolated Event Bus]
    end

    subgraph AI Operations
        Router[LLM Intent Router]
        SalesAgent[Sales & Quoting Agent]
        CSAgent[Customer Support Agent]
    end

    subgraph Human Interface
        Mobile[Tauri Mobile Client 375px]
    end

    IG --> GW
    WA --> GW
    SMS --> GW
    WEB --> GW

    GW --> Ingest
    Ingest --> Ledger
    Ingest --> PubSub

    PubSub --> Router
    Router --> SalesAgent
    Router --> CSAgent
    PubSub --> Mobile

    SalesAgent --> Ledger
    CSAgent --> Ledger
    Ledger -.-> External Channels
```

### Key Design Decisions
- **Unified Conversation Ledger:** All messages are normalized into a standardized schema regardless of origin (SMS vs IG). The schema strictly enforces multi-tenancy at the database row level.
- **Event-Driven Agent Triggering:** Inbound messages are published to a tenant-isolated Pub/Sub bus. An Intent Router determines if an AI agent (Sales, Support) should reply automatically or if it requires human escalation.
- **SPIFFE/SPIRE Zero-Trust:** Agent responses back to the Conversation Ledger are signed and validated via mTLS to ensure the AI only acts within the boundaries of the specific tenant's identity.

### Mobile UX Flow (375px first)
1. **The Unified Inbox:** A bottom navigation tab opens a clean, macOS glassmorphic list of active conversations. Each row shows the customer name, channel icon (IG, SMS), and preview.
2. **AI Co-Pilot Indication:** Threads where the AI agent is currently handling the conversation show a subtle glowing spark icon.
3. **Thread View:** Standard chat interface. AI responses are differentiated by a distinct bubble style. The user can hit a "Take Over" button to pause the AI and reply manually.
4. **Action Cards:** Within the chat, if the AI generated a quote or a payment link, it appears as an interactive Glassmorphism card directly in the thread that the human owner can tap to approve or edit before it's sent.

## Implementation Prompt

**Role:** Backend & Mobile Implementer
**Context:** We are building the Omni-Channel AI Communications Hub to centralize all business messaging (IG, WhatsApp, SMS, Web) and allow OHC's AI agents to autonomously handle sales and support inquiries.
**Task:** Implement the unified conversation ledger and the event routing system that connects inbound external messages to both the mobile inbox UI and the AI agent routing layer.
**Acceptance Criteria:**
- Create a normalized data model for multi-channel conversations that strictly enforces multi-tenant isolation.
- Implement the webhook gateway to receive and normalize messages from at least one external channel (e.g., simulated SMS or Web Chat).
- Build the real-time event bus (Pub/Sub) that broadcasts inbound messages to the mobile client and the AI intent router.
- Develop the 375px mobile UI (Tauri) for the Unified Inbox, including the "AI Co-Pilot" visual indicators and the chat thread view with the "Take Over" interaction.
- Ensure all agentic interactions with the communication ledger are authenticated via SPIFFE/SPIRE zero-trust policies.
- Do NOT prescribe the underlying database tech or API frameworks; use OHC standard stack patterns.
