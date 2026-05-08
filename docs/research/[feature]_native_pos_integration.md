# [Feature] Native Point of Sale (POS) Integration

## Problem Statement
Small business owners with physical presence (like Priya, boutique owner) face a disconnect between online and in-store sales. Managing inventory across two disjointed systems leads to errors and lost revenue.

## Research Report
- **Finding:** Lack of POS integration is a frequent complaint for platforms that focus primarily on online sales.
- **Source:** r/Etsy and similar communities highlight the pain of manual inventory synchronization.
- **Comparison:** Square Online is strong here, but Shopify's native POS is complex. OHC needs a seamless, mobile-first POS solution.

## Design Doc
- **High-Level Architecture:**
  - **Entity Types:** `POSDevice`, `Transaction`, `InventorySync`.
  - **Key Relationships:** `Transaction` linked to `Tenant` and updates `InventoryLevel`.
  - **Integration Points:** Hardware card readers, payment gateways.
- **UI Wireframes/Screen Flow:**
  - **Mobile UX (375px first):**
    1. Quick-add product to cart screen.
    2. Tap [Charge] -> Connect to card reader or manual entry.
    3. Successful transaction -> Real-time inventory deduction.
- **AI Agent Integration:** Agent analyzes in-store vs. online sales trends to optimize stock placement.

## Implementation Prompt
**User-Facing Outcome:** Owners can process in-person sales directly from the OHC mobile app, instantly syncing with their online inventory.
**Critical User Journey:**
1. Customer purchases item in-store.
2. Owner scans/selects item in OHC app.
3. Payment processed, inventory instantly updated.
**Acceptance Criteria:**
- Must support hardware card readers (e.g., Stripe Terminal).
- Real-time inventory sync is mandatory.
- Seamless flow from product selection to payment.

## Priority
P2

## Estimated Scope
Large
