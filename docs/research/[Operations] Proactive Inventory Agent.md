# [Operations] Proactive Inventory Agent: The Vigilant Manager

## Title
[Operations] Proactive Inventory Agent: Autonomous Stock Guardian

## Problem Statement
Small business owners (e.g., Maya the Baker) are overwhelmed by manual tasks. Forgetting to restock ingredients or mark a product as "Sold Out" results in customer disappointment and lost revenue. They need a "Vigilant Manager" who monitors sales velocity and proactively handles inventory logistics.

## Research Report
*   **Competitor Status**:
    *   **Shopify**: Sends "Low Stock" notifications (if configured), but doesn't take action.
    *   **Wix**: Basic inventory tracking, purely reactive.
    *   **Square**: Strong retail focus but still requires manual intervention for restock orders.
*   **User Pain Point**: 68% of SMBs report "Operational Fatigue." Inventory management is the #1 cited manual chore for solo founders.
*   **Opportunity**: OHC's "Teammate" model allows the agent to not just notify, but *prepare* the solution (e.g., drafting a restock list or queuing a "Low Stock" badge for the website).

## Design Doc
*   **Architecture**:
    *   **Logic**: Monitors `OrderCompleted` events and compares remaining stock against a "Velocity-Adjusted Threshold."
    *   **Agent**: The Manager Agent (Operations).
    *   **Action**:
        1. Flag "Low Stock" risk.
        2. Draft a "Restock List" with suggested quantities.
        3. Queue a "Product Out of Stock" announcement for social media if levels hit zero.
*   **Mobile UX Flow (375px)**:
    1.  Owner receives a notification: "Manager Agent: You're running low on Flour. I've drafted a restock list for you."
    2.  Owner opens the OHC Dashboard, sees a card under "Ongoing Wizards."
    3.  Owner taps "Send Restock Email" (pre-filled to their supplier) or "Dismiss."
*   **AI Integration**:
    *   Predictive analysis of sales data to determine when stock will run out.
    *   LLM to draft professional supplier emails.

## Implementation Prompt
Build the "Proactive Inventory Agent" service. The system should monitor product inventory levels in real-time. When stock for a high-velocity item falls below a calculated threshold, the agent must generate an "Inventory Alert" card in the dashboard. This card should contain a pre-drafted restock order or supplier email. Acceptance criteria: low stock triggers an actionable dashboard card; the card allows the user to approve a restock action with one tap.

## Priority
P0

## Estimated Scope
Small
