# Issue Brief: Omni-channel "Zero-Hardware" POS & Inventory Sync

## Problem Statement
Small business owners like Priya (Boutique Owner) struggle to keep their in-store sales in sync with their online inventory. Buying dedicated POS hardware is expensive and adds another "device" to manage. OHC needs to turn any smartphone into a professional POS terminal with automatic, real-time inventory reconciliation.

## Research Report
- **Competitor Audit:**
    - **Shopify POS:** Excellent sync, but often encourages buying their proprietary "POS Go" hardware. Supports "Tap to Pay on iPhone."
    - **Square:** The incumbent for small retail. Very strong in-person, but online sync can feel like a secondary bolt-on.
    - **Stripe Tap to Pay:** Allows NFC payments directly on iPhone/Android via SDK.
- **SMB Pain Point:** 42% of 1-star Shopify reviews mention inventory sync lag or the cost of extra hardware (Source: App Store Analysis).
- **Leapfrog Advantage:** By integrating Stripe's Tap to Pay SDK directly into the OHC mobile app, OHC provides a "Zero-Hardware" entry point. "The Manager" (Operations) ensures that a sale in the kitchen or on the floor instantly updates the website.

## Design Doc
### High-Level Architecture
- **POS Mode:** A dedicated toggle in the Slint mobile app that transforms the UI into a high-speed checkout grid (minimum 54px touch targets).
- **Stripe Terminal Integration:** Use the Tap to Pay SDK to handle NFC payments without external card readers.
- **Event-Driven Sync:** Every POS transaction triggers an `InventoryUpdate` event on the mesh, which "The Manager" uses to update all storefront channels.

### Mobile UX Flow (375px First)
1. **Dashboard:** Tap "Checkout" button.
2. **Product Grid:** Select items (or scan barcode via camera).
3. **Payment:** Select "Tap to Pay."
4. **Processing:** User holds customer's card to the back of the phone.
5. **Success:** Receipt emailed/texted automatically; inventory count drops by 1.

## Implementation Prompt
Implement a "POS Mode" within the mobile dashboard. Integrate with the Stripe Terminal SDK to support "Tap to Pay" on NFC-enabled devices. Ensure that every completed transaction sends a real-time update to the unified `products` table to keep online and offline stock counts perfectly in sync. The UI must be optimized for speed and high-glare environments (High contrast, large buttons).

## Priority
P0

## Estimated Scope
Large
