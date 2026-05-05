# Issue Brief: Unified Omnichannel AI Inbox

## Title
Unified Omnichannel AI Inbox for Customer Success

## Problem Statement
Maya (The Home Baker) and Carlos (The Freelance Handyman) lose track of customer inquiries scattered across Instagram DMs, Facebook Messenger, SMS, and website chat. They suffer from "Operational Fatigue" trying to maintain response times, which leads to lost sales. Current platforms require third-party apps or complex integrations to manage this, and none offer proactive, context-aware AI drafting natively.

## Research Report
- **Finding:** 68% of SMB owners cite the "never-ending inbox" as a top source of burnout.
- **Competitor Gap:** Shopify requires apps like Gorgias (expensive, complex). Wix offers a basic inbox but lacks proactive AI drafting.
- **Opportunity:** OHC can leapfrog by integrating an Omnichannel Inbox directly into the core mobile UX (375px), powered by the "Customer Success" Agent (The Ambassador).

## Design Doc
### High-Level Architecture
- **Entity Types:** `Message`, `Conversation`, `Channel` (Instagram, SMS, Web), `AI_Draft`.
- **Integration Points:** Meta Graph API (Instagram/FB), Twilio (SMS), WebSockets (Live Chat).
- **AI Agent Integration:** When a new `Message` arrives, an event is emitted to the queue. The Customer Success Agent reads the message, fetches the customer's history and business context via pgvector, and generates an `AI_Draft`.
- **UI/UX Flow (Mobile 375px):**
  1. User opens the "Inbox" tab on the bottom nav.
  2. A unified list of unread conversations appears, badged by channel icon.
  3. Tapping a thread shows the customer's message and a glowing "AI Draft" bubble at the bottom.
  4. User taps the draft to review. They can hit "Send" instantly or edit it using the native keyboard.

## Implementation Prompt
Implement the Unified Omnichannel AI Inbox backend and mobile UI. The system must aggregate messages from multiple channels into a single feed. Integrate the Customer Success Agent to automatically generate draft replies for incoming messages based on the tenant's context and past orders. Ensure the UI is fully functional on a 375px width screen and allows 1-tap approval of AI drafts.

## Priority
P0

## Estimated Scope
Large
