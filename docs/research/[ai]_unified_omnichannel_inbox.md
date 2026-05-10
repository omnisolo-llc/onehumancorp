# [AI] Unified Omnichannel Inbox (The Ambassador Agent)

## Problem Statement

Small business owners (like Maya the baker) are suffering from "Operational Fatigue." They lose up to 30% of their sales simply because they cannot respond to Instagram DMs, WhatsApp messages, and website inquiries quickly enough while actually doing their work (baking, fixing, teaching). Managing 3-4 different apps on a mobile device to answer the exact same questions ("Do you do vegan?", "What are your hours?") is a massive point of friction that legacy platforms only solve via expensive, clunky App Store plugins.

## Research Report

*   **Evidence:** 68% of surveyed pain points across r/smallbusiness and App Store reviews cite the "never-ending inbox" and communication lag as primary sources of stress.
*   **Competitor Gap:** Shopify and Wix require paid third-party apps (like Gorgias, which is complex and expensive) to achieve omnichannel support. Their native AI tools (like Sidekick) are reactive and dashboard-bound, not customer-facing.
*   **Strategic Advantage:** By integrating all messaging channels into a single feed and applying an autonomous drafting agent, OHC transforms a painful chore into a 1-tap lock-screen action.

## Design Doc

*   **High-Level Architecture:**
    *   **Ingestion Layer:** Webhooks connected to Meta Graph API (Instagram/WhatsApp) and native OHC web chat.
    *   **Event Mesh:** Messages trigger events on the NATS Hybrid Event Mesh.
    *   **The Ambassador Agent (Worker):** Subscribes to new message events. It queries the business's memory (RAG layer: FAQs, inventory status, past orders) to draft a contextual reply.
    *   **State Machine:** Message states transition from `Received` -> `Drafted` -> `Approved/Edited` -> `Sent`.
*   **Mobile UX Flow (375px First):**
    *   User receives a push notification: "New DM from Sarah on Instagram."
    *   Notification expands to show the AI-drafted reply: *"Hi Sarah, yes we have 3 vegan chocolate cakes left for pickup today!"*
    *   User taps "Approve & Send" directly from the notification/lock screen, or taps the notification to enter the OHC App to edit.
    *   The Inbox UI inside the app is a single chronological feed of all communications, regardless of source platform, utilizing OHC Glassmorphism tokens.
*   **AI Integration Points:** The Ambassador Agent must be context-aware, fetching real-time inventory and pricing data to formulate accurate drafts.

## Implementation Prompt

**Critical User Journey (CUJ):**
A customer messages the business on Instagram asking if a specific item is in stock. The business owner, currently away from their desk, receives a push notification on their phone from the OHC app. The notification displays the customer's question and a highly accurate, AI-drafted response. The owner taps "Approve" on the notification, and the response is instantly sent back to the customer on Instagram.

**Acceptance Criteria:**
*   Implement a unified data model capable of storing messages from diverse sources (IG, WhatsApp, Web).
*   Implement a background agent listener that automatically drafts replies to incoming messages using business context (inventory, FAQs).
*   Create a mobile-first (375px) UI component for the "Action Required" feed, allowing 1-tap approval or editing of AI drafts.
*   Ensure all agent operations adhere to OHC ML-Resilience rules (timeouts, auto-retry, graceful degradation if LLM is unavailable).

**Priority:** P0
**Estimated Scope:** Large
