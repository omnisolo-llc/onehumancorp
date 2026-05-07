# [Omnichannel] One-Tap Mobile POS Sync

## Title
One-Tap Mobile POS Sync for Hybrid Retailers

## Problem Statement
Boutique owners like Priya operate both in-person and online. Managing two separate inventory systems (e.g., Shopify for online, Square for in-store) leads to overselling, stockouts, and massive manual data entry. They need a system where their phone *is* the POS, and it perfectly syncs with their online storefront instantly.

## Research Report
- **Finding:** Omnichannel retailers grow revenue 20-30% faster, but 40% of SMBs cite "inventory synchronization" as their biggest operational hurdle.
- **Competitor Analysis:**
  - *Shopify:* Shopify POS is powerful but requires extra monthly fees for Pro features and separate hardware.
  - *Wix:* Offers POS integrations but often feels clunky and bolted-on.
  - *Square:* Best-in-class POS, but the online store builder (Square Online) is rigid and lacks deep customization or AI agents.
- **Evidence:** Trustpilot and App Store reviews for major platforms are filled with complaints about delayed inventory syncs leading to canceled orders and angry customers.
- **Recommendation:** OHC must treat the mobile app as a first-class POS terminal. Inventory must be unified natively, without requiring third-party sync apps.

## Design Doc
- **High-Level Architecture:**
  - A single, unified `InventoryItem` entity underlying both online orders and in-person sales.
  - A "Point of Sale" mode within the OHC mobile app.
  - Direct integration with mobile payment gateways (e.g., Tap to Pay on iPhone/Android).
- **Mobile UX Flow (375px first):**
  1. User opens OHC app and taps the "Sell In-Person" FAB (Floating Action Button).
  2. Camera scanner opens to scan barcode, or user taps visual product tiles.
  3. User taps "Charge $X.XX".
  4. System prompts "Tap card on phone" (using Tap to Pay).
  5. Success screen shows updated unified inventory count.
- **Agent Integration:** AI agent automatically flags low-stock items after an in-person rush and drafts a reorder email to suppliers.

## Implementation Prompt
Build a "Point of Sale" interface directly into the OHC mobile app. The owner should be able to select items from their existing product catalog, calculate a total, and simulate an in-person transaction (or use Tap to Pay). Crucially, this transaction must immediately deduct from the exact same inventory pool used by the online storefront. Provide a simple end-of-day summary showing combined online and in-person sales.

## Priority
P1

## Estimated Scope
Large
