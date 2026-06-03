# Title: Omnichannel AI Inbox Architecture

## Problem Statement
Small business owners like Carlos (Handyman) and Maya (Baker) receive customer inquiries across multiple fragmented channels: SMS, Email, Instagram DMs, and WhatsApp. Managing these disjointed channels leads to missed messages, slow response times, and lost revenue.

## Research Report
**Competitor Analysis:**
- **Shopify Inbox:** Centralizes chat but lacks deep integration with SMS and social media without third-party apps.
- **ManyChat:** Excellent for social automation but disconnected from inventory, booking, and quoting.
- **Zendesk/Intercom:** Too complex and expensive for non-technical small business owners.

**Gaps Identified:**
OHC lacks a unified, mobile-first omnichannel inbox where all customer communications are aggregated. Crucially, this inbox must be natively powered by the AI Customer Success Agent to auto-draft replies based on real-time business context (e.g., inventory levels, calendar availability).

## Design Doc
### Architecture Diagram
```mermaid
graph TD
    A[Customer SMS] --> B(Twilio / MessageBird Webhook);
    C[Instagram DM] --> D(Meta Graph API Webhook);
    E[Email] --> F(SendGrid Inbound Parse);

    B --> G[Omnichannel Gateway];
    D --> G;
    F --> G;

    G --> H[(PostgreSQL: Unified Conversation Ledger)];
    H --> I[AI Customer Success Agent];
    I -->|Auto-Drafts Reply| J[Mobile App UI (Inbox)];
    J -->|Business Owner Approves/Edits| K[Omnichannel Gateway];
    K -->|Routes back to original channel| A/C/E;
```

### Mobile UX Flow (375px First)
1. **Unified Feed:** Carlos opens the OHC app and sees a single "Inbox" tab with messages from SMS, Email, and Instagram.
2. **Contextual UI:** Tapping a message shows the customer's history, previous quotes, and an AI-generated draft reply.
3. **1-Tap Action:** The AI draft says, "Yes, I can fix that leaky sink on Tuesday at 2 PM. It will be $150." Carlos taps "Approve & Send," and it routes back to the customer via their original channel.

### Key Design Decisions
- **Unified Schema:** A `conversations` and `messages` schema in PostgreSQL with a `channel_type` enum to handle routing.
- **AI Auto-Drafting:** The Customer Success Agent listens to the unified ledger and generates drafts using RAG (Retrieval-Augmented Generation) against the business's FAQ, inventory, and calendar.
- **Zero-Trust Multi-Tenancy:** All webhooks and messages are strictly scoped to the `tenant_id`.

## Implementation Prompt
**For Implementer Agent:**
Implement the Omnichannel AI Inbox architecture.
- **Objective:** Create the data structures (`conversations`, `messages`), the webhook ingestion endpoints for at least one channel (e.g., SMS via Twilio mock), and the AI auto-drafting integration.
- **CUJ:** Customer sends an SMS. The system ingests it, creates a conversation, and the AI agent generates a draft reply. The business owner views the draft in the UI and approves it.
- **Acceptance Criteria:** E2E test verifying message ingestion, draft generation, and outbound routing. Ensure the UI is mobile-first. Validate Zero-Trust tenant isolation.

## Priority
P0

## Estimated Scope
Large
