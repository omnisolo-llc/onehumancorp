# [Feature] The Vigilant Manager (Proactive Operations Agent)

## Problem Statement
Small business owners like Priya (boutique owner) struggle with inventory sync. "Sold out" signs kill momentum, and manual tracking is tedious and prone to error. They often don't realize a popular item is out of stock until a customer complains.

## Research Report
- **Finding:** Trustpilot reviews for existing platforms show frustration with inventory management complexity.
- **Source:** Interviews/YouTube videos on "how to start an online boutique" consistently cite inventory tracking as the hardest part.
- **Comparison:** Most platforms offer basic low-stock alerts, but they require complex initial setup. OHC's agent will analyze sales velocity and create actionable restock tasks.

## Design Doc
- **High-Level Architecture:**
  - **Entity Types:** `InventoryLevel`, `SalesVelocity`, `RestockTask`.
  - **Key Relationships:** `RestockTask` linked to `Product`.
  - **Integration Points:** Agent monitors order events and inventory levels.
- **UI Wireframes/Screen Flow:**
  - **Mobile UX (375px first):**
    1. Dashboard shows an "Action Required" card: "Your vegan cake is selling 2x faster than usual. Only 3 left. Restock?"
    2. Button: [Create Restock Order] [Dismiss].
- **AI Agent Integration:** Agent runs periodically or on order events to calculate velocity and threshold risks.

## Implementation Prompt
**User-Facing Outcome:** The system proactively warns the owner before an item runs out, using plain language and contextual data (e.g., "selling faster than usual").
**Critical User Journey:**
1. Product sales spike.
2. Agent detects trend and low inventory.
3. Agent places an actionable card in the feed.
4. Owner taps to reorder or adjust inventory.
**Acceptance Criteria:**
- Must analyze sales velocity, not just static thresholds.
- Alert must be placed in the main dashboard feed.
- Action must be 1-tap executable.

## Priority
P1

## Estimated Scope
Medium
