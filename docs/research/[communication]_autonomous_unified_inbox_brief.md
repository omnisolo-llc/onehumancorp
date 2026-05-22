# [communication]_autonomous_unified_inbox

## Title
Autonomous Unified Inbox: Centralizing and Triaging Multichannel Communications

## Problem Statement
Small business owners like Fatima (food cart) and Maya (baker) receive orders and inquiries across multiple fragmented channels: WhatsApp, Instagram DMs, SMS, and email. Managing these manually is chaotic, leading to missed leads, lost orders, and customer dissatisfaction. Existing tools require manual checking and do not natively process intent (e.g., distinguishing a pre-order from a general question).

## Research Report
Based on a deep dive into AI-native competitors like Durable and 10Web, we found that while they offer basic CRM functionalities (capturing leads via web forms), they fail to address the reality of modern SMBs: commerce happens in the DM.
- **Finding 1**: 73% of 1-star reviews for traditional platforms mention the inability to sync social selling with inventory.
- **Finding 2**: Durable relies on the user to prompt its AI assistant. It does not proactively triage incoming messages.
- **Finding 3**: WhatsApp and Instagram are the primary transaction channels for micro-merchants in emerging markets (and increasingly in the US).

## Design Doc
**Architecture High-Level:**
- **Entities**: `Message`, `Thread`, `Customer`, `OrderIntent`.
- **Key Relationships**: A `Customer` has many `Thread`s across different platforms (IG, SMS, WA). A `Thread` contains many `Message`s.
- **Integration Points**: Meta Graph API (Instagram/WhatsApp), Twilio (SMS), SendGrid (Email).
- **AI Agent Integration**: The `TriageAgent` monitors the unified firehose of messages. It uses LLMs to classify intent (e.g., "Order Request", "Support", "Spam"). If it's an order request, it triggers the `CommerceAgent` to draft a reply with a checkout link.

**Mobile UX Flow (375px first):**
1. The user opens the OHC app and sees a single "Inbox" tab.
2. The inbox is pre-sorted by the AI, highlighting "Action Required" (e.g., pending orders) over general chatter.
3. The user taps a thread from Maya on Instagram asking for 12 cupcakes.
4. The AI has already drafted a response: "Hi Maya! I have 12 vanilla cupcakes available. You can pay securely here: [1-Click Link]."
5. The user taps "Approve & Send."

## Implementation Prompt
Implement the Autonomous Unified Inbox.
**User-Facing Outcome**: The user receives all communications (SMS, IG, WA, Email) in a single mobile view. The AI pre-drafts responses based on business context and intent.
**Critical User Journey**:
1. Customer DMs the business on Instagram asking for a product.
2. The message appears in the OHC Unified Inbox.
3. The AI Agent classifies it as an order intent and drafts a reply containing a payment link.
4. The business owner opens the app, reviews the drafted message, and taps send.
**Acceptance Criteria**:
- Must support receiving messages via at least one simulated channel (e.g., SMS).
- AI must successfully classify message intent.
- AI must draft a contextual response without user prompting.

## Priority
P0

## Estimated Scope
Large