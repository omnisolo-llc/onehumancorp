# [Operations] The Vigilant Manager: Proactive Inventory Intelligence

## Problem Statement
Small business owners (like Priya the boutique owner) often run out of stock on high-demand items because inventory tracking is a secondary, manual chore. "Sold out" signs kill sales momentum and frustrate customers. Existing tools only provide passive dashboards; the owner has to proactively log in and check the numbers.

## Research Report
The SMB Pain Point analysis (`smb_pain_points_top_10.md`) highlights "Operational Fatigue" as a massive burden.
*   **Competitor Failure:** Shopify and Wix offer low-stock alerts, but they are often buried in email notifications or require complex app configurations. They do not proactively draft the solution.
*   **Opportunity:** Move from passive notification to active remediation. An AI agent should not just say "you are low on stock," it should say "you are low on stock, I have drafted the re-order email to your supplier. Approve?"

## Design Doc
**High-Level Architecture:**
*   The backend runs a scheduled cron-like background job or listens to inventory reduction events via the event mesh.
*   The "Vigilant Manager" agent evaluates current stock levels against historical sales velocity (e.g., selling 5 units/day with 10 units left means 2 days of runway).
*   If a threshold is breached, the agent identifies the primary supplier for the item and drafts a restock order/email.
*   The proposed action is pushed to the user's dashboard feed as a `PENDING_APPROVAL` task.

**Mobile UX Flow (375px First):**
1.  Dashboard feed displays a high-priority card: "🚨 Low Stock Risk: Vanilla Candles."
2.  The card contains a brief, plain-language summary: "You're selling fast and will run out by Thursday."
3.  Below the summary, the AI offers a drafted action: "Re-order 50 units from Supplier A."
4.  User taps `Approve` to automatically send the restock email, or `Edit Quantity` to adjust.

## Implementation Prompt
Implement the backend "Vigilant Manager" agent that continuously analyzes inventory depletion rates and supplier data to predict stock-outs. Build the corresponding mobile-first UI component for the dashboard that surfaces these predictions as actionable, 1-tap approval cards rather than simple alerts.
*   **Critical User Journey (CUJ):** The system detects an item is selling faster than usual and will run out in 3 days. The agent drafts a restock email to the supplier. The owner sees the card on their mobile dashboard, taps "Approve", and the order email is dispatched.
*   **Acceptance Criteria:** The agent must accurately calculate runway based on recent sales velocity, not just static thresholds. The mobile UI must clearly explain *why* the alert was generated in plain language, avoiding jargon.

## Priority
P1

## Estimated Scope
Medium
