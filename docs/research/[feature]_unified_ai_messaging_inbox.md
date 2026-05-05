# Unified AI Messaging Inbox

## Title
Unified AI Messaging Inbox: 1-Tap Agent Approvals for Cross-Channel Communications

## Problem Statement
Small business owners, such as home bakers (Maya) and food cart operators (Fatima), lose up to 30% of potential sales due to slow response times. Managing inquiries across fragmented channels like Instagram DMs, WhatsApp, and email leads to "Operational Fatigue." Owners are overwhelmed and cannot keep up with customer messages while trying to perform the daily operations of their business.

## Research Report
Market analysis highlights a significant gap in current SMB tools. Traditional platforms like Shopify and Wix either lack unified communication features entirely, or bolt on reactive chatbots that fail to understand the nuanced context of the business's operations. Our research indicates that SMB owners don't want another inbox to manage; they want an intelligent assistant that drafts responses proactively based on their inventory, calendar, and operational history. The transition required is moving from **Reactive Tools** to **Proactive Autonomous Teammates**.

### Competitive Landscape: Inbox Capabilities

```mermaid
quadrantChart
    title Unified Inbox Autonomy vs. Context Awareness
    x-axis Low Context --> High Context (Inventory/Calendar Sync)
    y-axis Reactive/Manual --> Proactive/Autonomous
    quadrant-1 "Ideal OHC State"
    quadrant-2 "High Touch / Low Scalability"
    quadrant-3 "Legacy Inboxes (Manual)"
    quadrant-4 "Basic Chatbots (Rules-based)"
    "Shopify Inbox": [0.4, 0.3]
    "Wix Inbox": [0.3, 0.4]
    "Zendesk SMB": [0.2, 0.5]
    "ManyChat (Basic)": [0.7, 0.2]
    "OHC Ambassador": [0.85, 0.9]
```

### Feature Comparison Matrix

| Feature | OHC Ambassador | Shopify Inbox | Wix Inbox | Standard CRM |
| :--- | :--- | :--- | :--- | :--- |
| **Unified Channels** | IG, WhatsApp, Email, Web | Web, Email, Limited Social | Web, Email, Limited Social | Varies (often manual) |
| **Response Generation**| **Proactive Auto-Drafting** | Suggested Replies | Basic Rules | Templates only |
| **Context Awareness** | **Inventory, Calendar, History** | Order Status | Limited | None by default |
| **Approval Flow** | **1-Tap from Mobile Feed** | Manual Send | Manual Send | Manual Send |

## Design Doc

### 1. Webhook Ingestion Layer
- A resilient ingestion layer built to consume webhooks from Meta (Instagram, WhatsApp) and email providers (e.g., Resend).
- Events are normalized and pushed into the NATS hybrid event mesh for processing.

### 2. Proactive Drafting by "The Ambassador"
- The "Customer Success" agent ("The Ambassador") is triggered upon new message ingestion.
- The agent utilizes the Scribe Proactive RAG MCP to query the tenant's context (e.g., product availability, current bookings, previous interactions).
- The LLM (Gemini Pro/GPT-4o) drafts a contextual reply.

### 3. Action Feed Integration
- The drafted response is not sent immediately but posted to the business owner's centralized "Action Feed" (mobile-first, 375px optimized).
- The owner sees a preview of the message and can approve it with a single tap, or edit if necessary.

## Implementation Prompt
1.  **Ingestion Service**: Implement a Go service to receive webhooks from Meta Graph API for Instagram DMs and WhatsApp messages.
2.  **Event Normalization**: Standardize the payload into an internal `CustomerMessageEvent` protobuf and publish to the NATS event mesh.
3.  **Agent Workflow**: Update "The Ambassador" agent (built on the Go orchestration framework) to subscribe to these events.
4.  **Context Retrieval**: Connect the agent to the PostgreSQL/pgvector backend to pull relevant tenant data (inventory status, customer history).
5.  **Draft Generation**: Use the LLM provider interface to generate a response draft.
6.  **Action Feed Storage**: Persist the draft to the database as a pending action item linked to the tenant.
7.  **Mobile UI**: Implement the Flutter/Slint UI for the "Action Feed" displaying the draft with "Approve" and "Edit" buttons. Ensure Glassmorphism design tokens are used.

## Priority
**P1 (Critical)** - Communication lag is the most immediate revenue killer for target personas. High ROI.

## Estimated Scope
- **Backend**: 2-3 weeks (Webhook integration, agent orchestration, NATS wiring).
- **Frontend**: 1-2 weeks (Action Feed UI, 1-tap approval logic).
- **Total**: ~4 weeks for full end-to-end delivery.
