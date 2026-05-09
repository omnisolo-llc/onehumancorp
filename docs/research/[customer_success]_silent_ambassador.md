# The Silent Ambassador (1-Tap Customer Response)

## Problem Statement
Solopreneurs lose up to 30% of their sales because they are too busy baking, fixing, or teaching to reply to Instagram DMs or website chats immediately. Customers expect instant answers, but non-technical business owners suffer from operational fatigue and "never-ending inbox" syndrome.

## Research Report
Based on a synthesis of Reddit (r/smallbusiness, r/ecommerce) and App Store reviews for legacy platforms:
*   **Operational Fatigue (68% frequency):** SMB owners constantly answer the same 5 questions across multiple platforms.
*   **Communication Lag (40% frequency):** High drop-off rates when messages are missed during working hours or sleep.
*   **Competitor Gap:** Shopify Sidekick and Wix are reactive tools. They require the user to open a chat and prompt the AI. OHC must leapfrog this by using a proactive, event-driven agent teammate.

## Design Doc
*   **Architecture:** The NATS Event Mesh listens for `MessageReceived` events from integrated channels (e.g., Meta Graph API). An LLM worker (using Anthropic/OpenAI) is triggered. The agent queries `BusinessMemory` (FAQs, inventory state, past interactions) to draft a contextual response. The draft is placed into the "Pending Agent Actions" queue.
*   **UI Flow:** Mobile-first (375px native). A push notification arrives: "Draft reply ready for [Customer Name]." The user taps to open the Action Feed. The screen displays the customer's message and the AI's drafted reply. Two large, easy-to-tap buttons are presented: "Approve & Send" or "Edit."
*   **AI Integration:** Real-time event consumption, RAG for business context, and structured output for the message draft.

## Implementation Prompt
Implement the event listener and the UI feed for "Pending Agent Actions." The backend system must read incoming customer messages, draft a response utilizing the OHC context protocol (redacting PII securely), and surface it for 1-tap approval in the mobile dashboard. The mobile UI must use the OHC premium Glassmorphism CSS tokens and the Inter font for clear readability.

## Priority
P0

## Estimated Scope
Large
