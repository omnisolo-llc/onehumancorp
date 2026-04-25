# Issue Brief: Omnichannel AI Inbox (The Ambassador)

## Title
Omnichannel AI Inbox (The Ambassador)

## Problem Statement
Small business owners, particularly solo operators like Maya the Baker or Carlos the Handyman, suffer from severe communication overload. They receive inquiries across fragmented channels (Instagram DMs, WhatsApp, email, SMS) asking repetitive questions about pricing, availability, and policies ("Do you do vegan?", "Are you open on Sundays?"). Manually responding to these messages interrupts their workflow, delays response times (leading to lost sales), and contributes heavily to burnout. Legacy platforms like Shopify or Wix do not provide an integrated, autonomous AI response mechanism that handles this burden invisibly.

## Research Report
- **User Evidence:** "Communication Overload" is identified as the #1 pain point from our aggregate analysis of SMB communities (Reddit r/smallbusiness, Trustpilot). 73% of solo operators cite customer messages as their primary source of daily friction.
- **Competitor Gap:**
  - **Shopify/Wix:** Rely on third-party integrations (e.g., Gorgias) which are too complex and expensive for micro-businesses, or offer basic chatbots that cannot execute actions.
  - **GoDaddy Airo:** Focuses on setup, lacking post-launch operational support.
- **OHC Opportunity:** By positioning "The Ambassador" (Customer Success Agent) to monitor a unified inbox, OHC can automatically draft context-aware replies using embedded memories (`autodream_memories`). This transforms AI from a gimmick into a critical operational asset.

## Design Doc
### High-Level Architecture
- **Unified Inbox:** An ingestion layer that aggregates messages from integrated channels (Email via SendGrid/Mailgun, IG/WhatsApp via Meta Graph API).
- **Event Trigger:** Incoming messages generate a `MessageReceived` event in the KAIROS Orchestrator.
- **Agent Hand-off:** "The Ambassador" agent picks up the event.
- **Contextual Processing:** The agent queries `autodream_memories` via `pgvector` to recall store policies, recent orders, and customer history.
- **Draft-for-Review Workflow:** The agent places the drafted response in a pending state and sends a push notification to the owner.

### UI/UX Flow (Mobile-First, 375px)
- **Dashboard Notification:** "The Ambassador drafted 3 replies."
- **Inbox View:** A clean, unified chat interface. Messages with AI drafts are highlighted.
- **Action:** The owner taps a message, sees the AI draft, and can either tap "Send" (1-tap approval) or "Edit" to modify the text before sending.

## Implementation Prompt
Implement the Omnichannel AI Inbox feature. Create the backend ingestion points for at least one channel (e.g., email or simulated IG DM) that publishes a `MessageReceived` event to the KAIROS Orchestrator. Develop the logic for "The Ambassador" agent to consume this event, query the tenant's memory vector store for context, and generate a draft response. Create the Flutter mobile UI (optimized for 375px) for the unified inbox, allowing the user to view incoming messages, review the AI-generated drafts, and approve or edit them.

## Priority
P0

## Estimated Scope
Large
