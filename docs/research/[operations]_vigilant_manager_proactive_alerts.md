# [Operations] The Vigilant Manager: Proactive Stock & Booking Alerts

## Problem Statement
Small business owners like **Priya (Boutique)** often realize they are "Sold Out" only after a customer complains or when they manually check their shelf. Missing a restock window leads to lost revenue. Similarly, **Leo (Music Tutor)** misses booking requests because he's teaching and forgets to check his calendar.

## Research Report
- **Competitor Gap**:
    - **Shopify**: Sends "Low Stock" emails, but requires manual intervention to update or restock.
    - **Square**: Strong inventory, but reactive.
    - **Wix**: Basic alerts, no "agentic" follow-up.
- **Data**: "Inventory management" is the #2 cause of operational fatigue (68%) among SMB owners.
- **Opportunity**: Use the `Operations` agent to monitor the event mesh for `order.placed` or `booking.created` events and proactively draft "Restock" or "Schedule Update" tasks.

## Design Doc
- **Architecture**:
    - **Trigger**: `OrderCreated` / `BookingCreated` event on the mesh.
    - **Agent**: `OperationsAgent` (The Manager).
    - **Action**: Check `inventory_count` in `products` table. If < `threshold`, create a `DraftTask` for the owner.
- **Mobile UX (375px)**:
    - Notification: *"Low Stock Alert: Your 'Silk Scarf' has only 2 left. Tap to create a restock order."*
    - Action: 1-tap to "Mark as Restocked" (updates DB) or "Pause Sales" (hides product).
- **AI Integration**: Agent calculates velocity (e.g., "You sold 5 today, you will be out by tomorrow") to provide predictive alerts.

## Implementation Prompt
**Outcome**: Implement a predictive inventory and booking manager that monitors sales velocity and proactively alerts the owner before stock-outs occur.
**Critical User Journey**:
1. Customer buys the second-to-last item.
2. `OperationsAgent` detects the event and the new inventory level.
3. Agent calculates that at current velocity, the item will be gone in 4 hours.
4. Agent creates an "Action Item" in the dashboard with a pre-filled button to "Add 20 to Stock" or "Notify Supplier."
**Acceptance Criteria**:
- Must use real-time inventory tracking from the `products` table.
- Must generate proactive alerts, not just reactive "Out of Stock" notices.
- Notification must be actionable (1-tap).

## Priority
P1

## Estimated Scope
Small
