# Unified AI Inbox

## Problem Statement
Small business owners, especially in the services and micro-retail sectors, face "Communication Lag" and "Operational Fatigue". They receive inquiries across Instagram DMs, WhatsApp, and Facebook comments. Managing these separate channels manually leads to missed messages, lost sales, and poor customer experience. They need a single, unified inbox to manage all communications.

## Research Report
- **Competitor Audit:** Platforms like Shopify and Wix require third-party apps with additional costs to achieve omnichannel communication. They do not have built-in, AI-first unified inboxes.
- **Pain Point:** Scattered communications across multiple apps is a top reason for workflow inefficiency.
- **Target Persona:** Maya (Home Baker) and Fatima (Food Cart Operator) rely heavily on social media and messaging apps for their core operations.

## Design Doc
- **Architecture:** The Unified Inbox will integrate with the Meta Business Suite API (Instagram, Facebook, WhatsApp) to aggregate messages into a central OHC event mesh.
- **UI Flow:**
  - **Inbox View:** A single list view containing all messages from connected channels. Each message indicates its source (e.g., an Instagram icon).
  - **Detail View:** A chat interface where the user can read the conversation history.
  - **AI Features:** The "Ambassador Agent" automatically drafts context-aware replies based on previous orders, business rules, and inventory.
- **Mobile UX:** The interface must be primarily designed for a 375px mobile screen, functioning similarly to native messaging apps like iMessage or WhatsApp.
- **AI Integration:** The Customer Support Department's AI agents will process incoming webhooks, draft responses, and optionally auto-reply based on user-defined confidence thresholds.

## Implementation Prompt
Implement the "Unified AI Inbox" by integrating with the Meta Business Suite API. The system should receive incoming messages from Instagram, Facebook, and WhatsApp via webhooks, and route them to a centralized interface. The UI must be mobile-first. The `Ambassador Agent` should be triggered upon receiving a message to generate a drafted reply, which the business owner can review, edit, and send back to the original platform with a single click.

## Priority
P0

## Estimated Scope
Large