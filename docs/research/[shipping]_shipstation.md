# Shipping & Logistics: ShipStation

**Problem Statement:** E-commerce SMBs spend hours manually copying addresses, comparing carrier rates, and printing labels one by one.

**Research Report:** ShipStation aggregates orders from multiple channels and provides discounted shipping rates.
- Ease of Use: Good, though can be slightly overwhelming initially due to feature density. Very powerful once configured.
- Pricing: Starts around $9.99/month.
- Reputation: Industry standard for SMB e-commerce.
- Cloud vs. Standalone: Cloud-based.

**Design Doc:**
- New orders in OHC are pushed to ShipStation via API.
- User prints labels in ShipStation; ShipStation sends tracking info back to OHC via webhook.
- UI wireframes or screen flow description (375px first): Order detail page shows shipping status and tracking link.
- Mobile UX flow: Owner can mark an order as "Shipped" and input a tracking number manually if not using ShipStation, or view the auto-synced ShipStation status.

**Implementation Prompt:** Integrate ShipStation to automate order fulfillment. Push OHC orders to ShipStation and receive tracking updates.
- Acceptance Criteria: Orders sync seamlessly. Tracking numbers are attached to OHC orders and customers are notified.

**Priority:** P1
**Estimated Scope:** Medium
