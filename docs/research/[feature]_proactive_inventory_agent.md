# Proactive Inventory Management Agent

## Problem Statement
Owners like Priya (boutique) struggle to keep track of inventory across online and offline channels. Manual tracking leads to "sold out" items remaining listed (angering customers) or popular items not being restocked in time (lost revenue). The "Operational Fatigue" of managing this is a top 3 pain point.

## Research Report
*   **Competitor Audit:** Wix and Shopify offer inventory tracking, but they are passive (they just show a number). The user must remember to check the dashboard.
*   **User Pain Point:** Operational Fatigue (68% frequency). The burden of constant monitoring.
*   **OHC Advantage:** Treat AI as a Teammate. The Vigilant Manager agent monitors inventory events and proactively suggests actions.

## Design Doc
*   **UX Flow (375px Mobile First):**
    1.  **Dashboard Feed:** A new card appears in the daily feed: "Your 'Summer Dress - M' is selling fast. Only 2 left. Want me to draft a reorder email to your supplier?"
    2.  **Action Buttons:** [Draft Email] [Ignore for now]
    3.  **Review:** If drafted, user reviews a plain-language email and taps [Send].
*   **Architecture (High Level):**
    *   Event Source: Order placed event on the NATS Hybrid Event Mesh.
    *   Agent Action: Inventory Agent subscribes to events, checks stock levels, and applies predictive logic based on sales velocity.
    *   Output: If threshold met, Agent creates an Action Item in the `consolidated_memory` or task queue for user review.

## Implementation Prompt
Create a proactive inventory agent that listens to sales events and surfaces actionable restock alerts to the user's dashboard feed.
*   **Critical User Journey:** Item stock drops below threshold -> Agent generates an alert -> User sees alert on mobile dashboard -> User clicks to action (e.g., draft restock request).
*   **Acceptance Criteria:**
    *   Must be driven by the event mesh (no polling).
    *   Alerts must be in plain, non-technical language.
    *   Must include a 1-tap action to resolve the issue.

## Priority
P1

## Estimated Scope
Medium
