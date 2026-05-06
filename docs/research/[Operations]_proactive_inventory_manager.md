# 🔍 Scout: The Vigilant Manager (Proactive Operations)

## Title
The Vigilant Manager (Proactive Operations)

## Problem Statement
Small business owners like Priya (Boutique Owner) and Fatima (Food Cart) lose sales and customer trust because they forget to restock popular items. "Sold out" signs kill momentum, but manual inventory tracking across online and physical channels is tedious and error-prone. They need an invisible system that watches their stock levels and sales velocity, and automatically prepares restock orders before they run out.

## Research Report
- **Strategy**: Autonomous event-driven inventory monitoring and supply ordering.
- **Target Persona**: Priya (Boutique Owner), Fatima (Food Cart)
- **Advantages**: Prevents lost revenue from stockouts. Saves hours of manual counting.
- **Risks**: Placing incorrect orders if sales data is inaccurate. Needs a strong approval gate.
- **Competitor Gap**: Shopify and Wix offer low-stock alerts, but they are passive notifications. OHC will proactively generate the restock task (e.g., drafting the supplier email or creating the purchase order) for 1-tap approval.
- **Data**: Operational fatigue is the #2 pain point for SMBs (68%).

## Design Doc
- **High-Level Architecture**:
  - A background agent ("The Manager") listens to `OrderPlaced` and `InventoryUpdated` events.
  - The agent calculates sales velocity and predicts stockout dates.
  - When a product is predicted to run out within a configurable threshold (e.g., 7 days), the agent flags a "Low Stock Risk".
  - The agent drafts a restock order based on the supplier details stored in the business profile.
  - The drafted order is queued in the Dashboard Action Feed.
- **UI Flow**:
  - User receives an alert in their Action Feed: "You are projected to run out of 'Vegan Cake' by Thursday. Tap to approve restock order from Supplier X."
  - User reviews the drafted order and quantity.
  - User taps "Approve" to send the order.

## Implementation Prompt
Implement "The Manager" operations agent. The agent should subscribe to order events and track inventory levels. It needs to calculate the run-rate of items and generate a "Restock Recommendation" when inventory is projected to fall below a safe threshold. The agent should draft the communication or purchase order for the supplier and place it in the user's action queue for 1-tap approval. Do not implement the actual supplier integration yet, focus on the agentic detection and drafting logic.

## Priority
P1

## Estimated Scope
Medium
