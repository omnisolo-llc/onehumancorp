# Feature Brief: Unified Omni-Channel Inbox

## Priority: P1 (High)
**Strategic Goal:** Centralize Customer Communications.

### 1. Problem Statement
Users suffer from "Scattered Inbox Syndrome". They cannot effectively run their business from their phone if they are constantly switching between Instagram, Facebook Messenger, WhatsApp Business, and Email to track a single customer's order history and conversation.

### 2. The OHC Solution
A single, unified inbox within the OHC app that aggregates messages from all supported social and email channels. Every message is tied directly to the customer's CRM profile and order history within OHC.

### 3. Architecture & Implementation (Research Report)
-   **Channel Connectors:** Utilize Meta Business Suite (Instagram/WhatsApp) and SendGrid (Email) webhooks to ingest messages.
-   **Data Model:** A unified `Message` and `Conversation` table in the OHC-SIP DB, linking external platform IDs to internal OHC Customer IDs.
-   **UI Design:** A mobile-first (375px) chat interface. When viewing a conversation, the user sees the customer's recent orders, LTV (Lifetime Value), and AI-suggested draft replies inline.
-   **Agent Synergy:** This inbox acts as the user-facing surface for "The Ambassador" agent (defined in `[ai-differentiation]-auto-replying-agents.md`).

### 4. Implementation Prompt
Build the unified omni-channel inbox. Create the database schema to normalize messages from Meta Graph API (Instagram/WhatsApp) and Email. Implement the mobile-first UI that displays the aggregated conversation thread alongside the customer's CRM data and order history, integrating directly with the AI drafting system.
