# Unified Omnichannel Inbox with AI Drafts

## Problem Statement
SMBs are consistently losing sales opportunities because their customer communications are dangerously scattered. A potential customer asks a pricing question on Instagram, a vendor sends an inquiry via WhatsApp, and a client replies to an email. The business owner inevitably misses the IG DM in the noise, resulting in the loss of a valuable catering gig. The context switching required to jump between four different applications to run a single business is exhausting and unsustainable.

## Research Report
Detailed analysis of forums like r/ecommerce indicates that 'managing messages' ranks as a top-3 daily stressor for business owners. While Shopify offers an 'inbox' feature, it is largely focused on their proprietary webchat widget. Meta's Business Suite attempts to consolidate Instagram and Facebook communications but entirely ignores Email and SMS channels. OHC has a critical opportunity to capture immense value by providing a single, truly unified view of all customer interactions, augmented with AI that proactively pre-drafts the most likely response (e.g., automatically answering 'What are your hours this weekend?').

## Design Doc
### Architecture Vision
- **Entities**: ConversationThread, Message, Channel (IG, WA, Email), DraftResponse.
- **UX Flow**:
  1. The mobile app features a single, prominent 'Inbox' tab.
  2. Incoming messages display clear iconography indicating their source channel.
  3. When the user taps to open a specific message, the AI has already analyzed the intent and drafted a highly contextual reply based on the business's internal knowledge base.
  4. The user simply taps a 'Send' button to approve the draft, or they can tap into the text field to manually edit it.
- **Mobile UX**: The interface should closely mirror the familiar design language of Apple Messages, but enhanced with smart suggestion chips situated directly above the keyboard.
- **Agent Integration**: A background Concierge Agent continuously listens to incoming webhooks from all integrated channels, analyzes the semantic intent of the message, and generates corresponding DraftResponse records.

## Implementation Prompt
**Outcome**: Develop a centralized inbox UI where users can seamlessly read and reply to messages originating from multiple diverse sources, featuring AI-suggested responses primed for 1-tap approval.
**Critical User Journey**:
1. A customer sends an inquiry via an Instagram DM.
2. The OHC owner receives a unified push notification.
3. The owner opens the OHC app, views the message, and sees an accurate, pre-drafted reply.
4. The owner taps 'Approve & Send', resolving the inquiry in seconds.
**Acceptance Criteria**: The system must support robust, multi-channel threaded conversation views. It must prominently and accurately feature the AI-drafted responses.

## Priority
P0

## Estimated Scope
Large
