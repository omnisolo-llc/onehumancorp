# Issue Brief: Tap-to-Pay Mobile POS Integration Strategy

## Problem Statement
Physical vendors (farmers market stalls, pop-up shops) need a reliable way to take in-person payments without purchasing expensive, easily lost hardware dongles. Relying solely on cash or peer-to-peer apps (Venmo/CashApp) complicates tax reporting and completely bypasses inventory tracking.

## Research Report
Apple and Google have recently opened up their NFC hardware capabilities for 'Tap-to-Pay on iPhone/Android'. While Square's massive early success was built on proprietary hardware, the future of micro-retail is hardware-less. By integrating Tap-to-Pay capabilities directly into the core OHC mobile admin app, any owner's smartphone instantly becomes a fully synchronized cash register.

## Design Doc
**High-Level Architecture & Entities:**
- Native mobile module interacting with iOS/Android NFC APIs (likely via Stripe Terminal SDK).
- `Transaction`: Financial record linked to an `Order`.

**Mobile UX Flow:**
1. **Cart Creation:** Customer wants to buy a physical item in-store. Owner adds item to cart in OHC app.
2. **Payment Selection:** Owner taps 'Tap to Pay'.
3. **Execution:** Customer taps their contactless credit card or Apple Pay to the owner's phone.
4. **Completion:** Payment is processed securely, receipt is texted, and inventory is instantly deducted.

**AI Agent Integration Points:**
- Minimal AI involvement here; this is purely a high-reliability hardware/infrastructure integration.

## Implementation Prompt
Design and implement the foundation for a Tap-to-Pay solution utilizing modern mobile device capabilities (via Stripe Terminal or similar abstracted SDKs) to allow in-person transactions without external hardware. Ensure this flow is tightly coupled with the existing centralized inventory management system.

**Critical User Journey (CUJ):**
1. Admin user creates a draft order consisting of existing catalog items.
2. Admin initiates 'Tap to Pay' flow.
3. Customer presents payment instrument to device.
4. Transaction succeeds, order state updates to 'Paid', and inventory decrements.

**Acceptance Criteria:**
- The backend must support creating an order state that is pending terminal payment capture.
- The system must provide a simulated mock endpoint to test successful and failed Tap-to-Pay interactions.
- Inventory must successfully decrement only upon confirmed payment capture.

## Priority
P2

## Estimated Scope
Large
