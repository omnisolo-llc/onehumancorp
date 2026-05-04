# OHC The Silent Ambassador (Customer Success)

## Problem Statement
Solopreneurs and small business owners are experiencing high levels of operational fatigue due to the "never-ending inbox." They struggle to respond to customer inquiries (like "do you do vegan cakes?") across multiple platforms (Instagram DMs, email, website chat) promptly. As a result, 40% report losing sales due to communication lag because they cannot answer messages while they are working, sleeping, or managing other aspects of the business.

## Research Report
- **Market Context:** According to our analysis of r/smallbusiness and app store reviews, "Communication Lag" is a top 10 pain point for SMBs.
- **Competitor Gaps:** Shopify offers a reactive "Sidekick" AI chatbot, but it does not act autonomously. Wix currently lacks autonomous background agents.
- **OHC Opportunity:** Solopreneurs need a system that watches for incoming events and autonomously drafts context-aware replies for quick approval, dramatically reducing response times and saving hours per week.

## Design Doc
- **High-Level Architecture:**
  - An event-driven AI agent that subscribes to incoming messages via the `ohc-core` event mesh.
  - The agent accesses the tenant's business memory (e.g., product catalog, FAQ) stored in the pgvector database.
  - It generates a drafted response and queues it into a central "Action Required" dashboard feed.
- **UI Wireframes/Screen Flow:**
  - **Dashboard View:** A clean list of pending drafts in the "Action Required" section.
  - **Draft Detail View:** Displays the original customer message and the AI's drafted reply.
  - **Interaction:** A single "Approve & Send" button for 1-tap execution, along with a "Edit" button for manual tweaks.
- **Mobile UX Flow (375px First):**
  - Push notification alerts the user of a new draft.
  - Tapping the notification opens a native mobile view where the user can read and approve the message with one tap, even from the lock screen.
- **AI Agent Integration:**
  - The agent functions as a background worker processing tasks securely and isolating data per tenant.

## Implementation Prompt
Implement the "Silent Ambassador" feature that allows users to seamlessly handle incoming customer inquiries.
- **Critical User Journey (CUJ):**
  1. A customer sends a message to the business.
  2. The AI Agent intercepts the message and drafts a reply.
  3. The business owner opens the OHC mobile app.
  4. The owner sees the drafted message in the "Action Required" feed.
  5. The owner clicks "Approve" to send the message immediately.
- **Acceptance Criteria:**
  - The feature must be accessible from the 375px mobile view.
  - The drafted message must appear within the app's dashboard feed.
  - The "Approve" action must result in the message being dispatched.
  - The feature must integrate with the existing event-mesh for incoming messages.

## Priority
P0

## Estimated Scope
Medium
