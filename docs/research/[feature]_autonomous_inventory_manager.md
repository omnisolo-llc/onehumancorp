# [feature] Autonomous Inventory Manager

## Title
The Vigilant Manager (Autonomous Inventory)

## Problem Statement
Small business owners (like Maya the baker) often miss sales because popular items sell out and they forget to update inventory or reorder supplies. "Sold out" signs kill momentum, and manual inventory tracking is tedious and error-prone on existing platforms like Shopify.

## Research Report
*   **Gap:** "Sold out" signs kill momentum; manual inventory tracking is tedious.
*   **Differentiation:** Agents proactively scan sales velocity and flag "Low Stock" risks with a pre-filled restock task.
*   **Outcome:** Never miss a sale due to forgotten inventory.
*   **Evidence:** "Sold out" signs kill momentum; manual inventory tracking is tedious. (Source: Top 10 SMB Pain Points - Operational Fatigue 68%)

## Design Doc
*   **Entities:** Product, InventoryLevel, SalesEvent, RestockTask.
*   **Key Relationships:** Product has many InventoryLevels. SalesEvent updates InventoryLevel. RestockTask is linked to Product.
*   **UI/UX (Mobile-First 375px):**
    *   Dashboard feed shows a "Low Stock Alert" card.
    *   Card displays the product name, current stock, and a suggested restock amount based on recent sales velocity.
    *   A single "Approve Restock" button updates the inventory and optionally triggers a reorder notification to a supplier.
*   **AI Agent Integration:** A background agent monitors `SalesEvent` streams. If velocity indicates a stockout within X days, it generates a `RestockTask` and pushes it to the dashboard.

## Implementation Prompt
Implement a background agent that monitors product inventory levels against recent sales velocity. When an item is predicted to run out of stock soon, the agent should proactively create a "Restock Task" in the user's dashboard. This task should include a plain-language explanation (e.g., "Vegan cakes are selling fast! You have 2 left, which usually sell in 1 day.") and a 1-tap "Restock" button to update the quantity. Do not prescribe specific database tables or function names; focus on the event-driven architecture and the user-facing dashboard feed.

## Priority
P1

## Estimated Scope
Medium
