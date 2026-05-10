# SMB Pain Point: Lost Leads and Lack of Marketing Time

## Problem Statement
Small business owners lose significant revenue because they lack the time and systems to follow up on leads or re-engage past customers. A potential client might ask for a quote and never hear back after the initial response, or a customer might abandon a cart. Owners know they *should* do marketing, but complex CRM setups and email campaign builders are too intimidating.

## Research Report
*   **The Issue:** "I'm a mechanic, not a marketer." Owners forget to follow up.
*   **Competitor Landscape:** Platforms offer "Abandoned Cart" emails, but they are often generic templates that the owner has to configure. Proactive CRM features usually require expensive add-ons or entirely separate platforms (like HubSpot or Mailchimp).
*   **The OHC Advantage:** Proactive, Agentic Re-engagement. The system shouldn't wait for the user to set up a campaign; it should proactively suggest actions based on user behavior and system data.

## Design Doc
*   **Core Entities:** `Lead`, `InteractionHistory`, `ProposedAction`
*   **Integration Points:**
    *   State machine monitoring for "stale" leads or abandoned actions.
    *   LLM to generate personalized, context-aware follow-up messages.
    *   Push Notification service (to alert the owner).
*   **UX Flow (Mobile First):**
    1.  The owner receives a push notification: "Carlos, you sent a quote to Sarah 3 days ago. Want me to send a quick follow-up?"
    2.  The owner opens the notification to see the AI-generated draft SMS/email.
    3.  The owner taps "Send" or "Edit." The action is completed in seconds.

## Implementation Prompt
Develop the "Proactive Re-engagement" monitoring system. This involves creating a background worker or chron job that evaluates the state of `Leads` and `Orders` (e.g., quotes sent > 48 hours ago with no response). When a candidate for re-engagement is found, the system must trigger an LLM prompt to generate a contextual follow-up message draft and queue a `ProposedAction` notification for the business owner to review via the mobile app.

## Priority
P2

## Estimated Scope
Medium
