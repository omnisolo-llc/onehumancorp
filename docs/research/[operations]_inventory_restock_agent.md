# [Operations] Proactive Inventory Restock Agent (The Vigilant Manager)

## Title
Implement the "Vigilant Manager" AI Agent for Proactive Inventory Management

## Problem Statement
Small business owners, especially those selling physical goods (like Maya the Baker or Priya the Boutique Owner), suffer from "Operational Fatigue" (Ranked #2 in Top 10 SMB Pain Points, 68% frequency). Manual inventory tracking is tedious and error-prone. When a product unexpectedly sells out, it kills sales momentum and disappoints customers. Business owners need a system that doesn't just display a "sold out" badge, but proactively anticipates stock-outs and tees up the restock process.

## Research Report
*   **Competitor Landscape:** Shopify and Wix rely on basic low-stock email alerts. They are reactive and require the user to log in, navigate to a complex inventory dashboard, and manually update numbers.
*   **User Evidence:** App Store reviews for legacy platforms frequently cite: "Can't even change a product price easily from my phone without the app crashing or hiding the menu." Reddit (r/shopify) mentions the difficulty of managing fast-moving inventory on mobile.
*   **OHC Differentiation:** Instead of a passive alert, the OHC "Vigilant Manager" agent watches the event mesh for sales velocity. When an item is trending toward stock-out, the agent drafts a "Restock Task" and pushes it to the owner's Action Feed on their 375px mobile dashboard. The owner can approve the restock plan with a single tap.

## Design Doc
*   **Core Entities:** `Product`, `InventoryLevel`, `SalesEvent`, `AgentTask`, `UserActionFeed`.
*   **Key Relationships:** The agent subscribes to `SalesEvent` via the Hybrid Event Mesh (NATS/Redis). It evaluates the `InventoryLevel` against historical sales velocity (stored in `AgentMemory`). It generates an `AgentTask` linked to the `Product`.
*   **Integration Points:**
    *   **Trigger:** NATS/Redis pub/sub event `ohc.order.created` or `ohc.inventory.decremented`.
    *   **Logic:** The Operations Agent (built on the Builtin Agent framework) processes the event.
    *   **Output:** Creates an actionable item in the user's unified dashboard.
*   **UI/UX Flow (Mobile-First, 375px):**
    1.  User opens the OHC app.
    2.  The Home tab displays an "Action Required" card: "Your Vegan Chocolate Cake is selling fast (3 left). Tap to update restock quantity."
    3.  User taps the card. A simple bottom sheet appears with a numeric keypad.
    4.  User enters "+10" and taps "Confirm."
    5.  Inventory is updated, and the card disappears.

## Implementation Prompt
Implement the backend agent logic and the frontend mobile UI for the Proactive Inventory Restock feature.
1.  Create the event listener in the Operations Agent domain that monitors inventory depletion.
2.  Implement the heuristic/LLM check to determine if an item is "at risk" based on current stock and recent sales velocity.
3.  Design the data structure to push an actionable "Restock Request" to the user's dashboard feed.
4.  Build the Flutter/Slint UI component for the "Action Required" card and the subsequent numeric input bottom sheet. Ensure the design strictly adheres to the 375px mobile-first standard and uses the native mobile keyboard for numeric entry. The flow must be complete end-to-end.

## Priority
P1

## Estimated Scope
Medium
