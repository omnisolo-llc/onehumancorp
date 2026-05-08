# [Feature] The Silent Ambassador: 1-Tap Inbox Agent

## Title
The Silent Ambassador: 1-Tap Inbox Agent

## Problem Statement
Small business owners (like Maya the Baker or Carlos the Handyman) lose up to 30% of potential sales because they cannot reply to customer inquiries instantly while actively working. They suffer from "operational fatigue," constantly switching between Instagram DMs, emails, and SMS to answer the same repetitive questions about pricing, availability, and location.

## Research Report
*   **Competitor Landscape:**
    *   *Shopify Inbox:* Centralizes messages but requires manual typing or setting up rigid, static quick replies. Sidekick is a tool for the merchant, not an automated responder for customers.
    *   *Wix Inbox:* Similar to Shopify; centralizes but lacks autonomous drafting based on deep business context.
*   **User Pain Points:** "Operational Fatigue" ranks #2 in our Top 10 SMB Pain Points (68% frequency). Users describe the "never-ending inbox" as a massive source of stress.
*   **OHC Differentiation:** OHC treats AI as a proactive teammate. Instead of waiting for the user to write a reply, the agent watches the event mesh, drafts the perfect reply based on the store's "memory," and presents it for 1-tap approval.

## Design Doc
*   **High-Level Architecture:**
    *   Inbound messages (from various channels) are normalized and published to the NATS Event Mesh.
    *   The `SilentAmbassadorAgent` subscribes to the `message.received` topic.
    *   The agent queries the `VectorRepository` (Business Memory) to find context (store policies, product details, previous interactions with this customer).
    *   The agent drafts a response and creates an `ActionItem` entity linked to the `Message` entity.
    *   The UI displays a feed of pending `ActionItems`.
*   **UI/UX Flow (Mobile-First 375px):**
    *   User receives a push notification: "Agent drafted 3 replies for you."
    *   User opens the OHC app to the "Action Feed".
    *   Each item shows the customer's message and the Agent's drafted reply in a clean, glassmorphism card (`backdrop-filter: blur(20px)`).
    *   User has two large buttons: **Approve (Send)** or **Edit**.
    *   If Approve is tapped, the event mesh routes the message back to the original channel.

## Implementation Prompt
Implement the "Silent Ambassador" feature to solve operational fatigue for SMBs. Create the background agent logic that listens for incoming communications, synthesizes a context-aware draft reply using the business's stored memory, and surfaces this draft in the user's Action Feed. The Critical User Journey (CUJ) is: A customer asks a question -> the system drafts a reply -> the business owner opens the app and hits "Approve" with 1 tap. Ensure the UI is mobile-first (375px) and utilizes premium OHC design standards. Do not prescribe specific database schemas or API contracts; focus on the event-driven drafting and the 1-tap approval experience.

## Priority
P0

## Estimated Scope
Large
