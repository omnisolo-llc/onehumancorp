# Issue Brief: Unified Omni-Channel Inbox for SMBs

## Title
Unified Omni-Channel Inbox for SMBs

## Problem Statement
Small business owners, especially those like Priya (boutique owner) and Maya (baker), are missing sales because customer communications are fragmented. A customer might DM on Instagram, email a question, and then follow up via WhatsApp. The merchant has to constantly switch between 4-5 apps on their phone to keep track, leading to dropped conversations, lost context, and ultimately lost revenue.

## Research Report
- **Competitor Analysis:**
  - *Shopify:* Shopify Inbox centralizes some chats but heavily pushes its own chat widget; external integrations can be clunky.
  - *Wix:* Wix Inbox exists but struggles with seamless real-time syncing across all modern social channels.
  - *Square Online:* Good for in-person, but online communication tools are basic.
- **User Pain Points:**
  - "I lost a $500 catering order because the customer DMed me on Instagram and I only checked my email that day." (r/smallbusiness)
  - "Juggling WhatsApp, IG, Facebook, and Email is a full-time job." (Trustpilot review)
- **Data:** 68% of consumers say they prefer to message a business rather than call, and they expect a response on their preferred channel within an hour.

## Design Doc
- **High-Level Architecture:**
  - **Entity Types:** `Merchant`, `Customer`, `ChannelConnector` (IG, Email, SMS), `UnifiedMessageThread`.
  - **Key Relationships:** A `UnifiedMessageThread` aggregates messages from multiple `ChannelConnector`s for a single `Customer` and `Merchant`.
  - **Integration Points:** Webhooks for social platforms (Meta Graph API), SMTP/IMAP integration, Twilio/SMS integration.
- **UI Wireframes/Screen Flow:**
  - *Mobile UX Flow (375px first):*
    1.  **Main Inbox:** A single, clean feed of all incoming messages. Icons indicate the source channel (e.g., a small Instagram logo next to the message).
    2.  **Thread View:** A continuous chat history with a customer, regardless of which channel they used for a specific message.
    3.  **Quick Actions:** Integrated buttons within the chat to "Send Quote", "Create Order", or "Request Payment" without leaving the inbox.
- **AI Agent Integration Points:**
  - The unified inbox is the primary interface where the AI Auto-Reply agent (from the AI Differentiation brief) operates, highlighting which messages were handled by AI and which need human attention.

## Implementation Prompt
**User-Facing Outcome:** The merchant has a single "Inbox" tab in the OHC app that receives and sends messages across Instagram, WhatsApp, Email, and SMS seamlessly.

**Critical User Journey (CUJ):**
1. Customer A sends an Instagram DM. It appears in the OHC Inbox.
2. Merchant replies from the OHC app. The reply is sent back as an Instagram DM.
3. Customer A later emails the merchant. The email appears in the *same* thread in the OHC Inbox.
4. Merchant taps "Create Order" directly within the chat thread, sending a checkout link back to the customer.

**Acceptance Criteria:**
- The inbox must support at least two distinct mock channels for testing.
- The UI must clearly differentiate between channels visually while maintaining a unified thread per customer.
- Must include E2E tests validating the flow from receiving a message to sending a reply and generating a checkout link within the chat context.

## Priority
P1

## Estimated Scope
Medium
