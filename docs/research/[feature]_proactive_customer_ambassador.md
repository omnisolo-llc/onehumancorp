# Feature Issue Brief: The Silent Ambassador (Proactive Customer Success Agent)

## Title
Implement The Silent Ambassador for Proactive Customer Communication

## Problem Statement
Small business owners like Maya (baker) and Carlos (handyman) lose sales because they cannot respond to customer inquiries immediately while working. They suffer from "Operational Fatigue" responding to the same questions manually across multiple platforms.

## Research Report
- **Pain Point**: Operational Fatigue (68% frequency) and Communication Lag (40% frequency) are top SMB complaints.
- **Competitor Gap**: Shopify Sidekick requires the user to initiate a prompt. OHC must leapfrog this by acting proactively.
- **Evidence**: "Losing sales because DMs aren't answered while the owner is sleeping or working." (Source: SMB Pain Point Audit).

## Design Doc
- **High-Level Architecture**: The agent listens to the OHC event mesh for incoming customer messages (DMs, emails). It accesses the business's episodic memory and product catalog to draft a highly contextual reply.
- **Mobile UX Flow (375px First)**:
  1. User receives a push notification on their lock screen: "Draft reply ready for Carlos."
  2. Tapping opens the OHC mobile app to the "Action Required" feed.
  3. The drafted message is shown with a clear "Approve & Send" or "Edit" button.
  4. The UI uses Glassmorphism and requires no jargon (e.g., "Customer asked about X, send this?").
- **AI Integration**: Triggers on message receipt; drafts via LLM; queues for 1-tap human approval.

## Implementation Prompt
**To Implementer Agent:**
Build the "Silent Ambassador" feature. Implement an event listener that detects incoming customer messages. Pass the message and relevant business context to the built-in LLM to generate a draft reply. Surface this draft in the user's action feed (mobile-optimized dashboard) requiring a simple 1-tap approval to send. Ensure the UI adheres to the "Grandmother Test" (no complex configurations, ≥44x44px touch targets). Do not prescribe specific database tables or API contracts. Focus on completing the Critical User Journey (CUJ) from message receipt to 1-tap approval.

## Priority
P0

## Estimated Scope
Medium
