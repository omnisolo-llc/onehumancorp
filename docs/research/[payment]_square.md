## 4. Payment Processing: Square

**Title:** Integrate Square Payments for Omnichannel Retail
**Problem Statement:** Retailers often sell both in-person and online. They need unified inventory and payment processing without managing separate systems.
**Research Report:**
- **Tool evaluated:** Square
- **What problem it solves for which persona:** Connects in-person POS sales with online storefront sales for local shop owners.
- **Ease of Use:** Renowned for its hardware simplicity and straightforward dashboard.
- **Pricing:** Usually 2.9% + 30¢ per online transaction. No monthly fee for basic.
- **Reputation:** Industry leader for small business omnichannel retail.
- **Advantages & Risks:**
  - *Advantages:* Hardware ecosystem, well-known brand, unified inventory.
  - *Risks:* API can be complex due to the breadth of features (catalog, inventory, customers, payments).
- **Cloud/Standalone Mode:** Cloud integration is standard. Standalone requires secure local handling of OAuth tokens.
**Design Doc:**
- **Trigger:** Customer checks out online, or owner uses OHC to process a manual order.
- **Action:** Payment is routed through Square API, inventory is updated simultaneously.
- **User View:** Unified sales dashboard showing both online and in-person revenue.
**Implementation Prompt:**
Create a payment gateway module that allows standard checkout using a third-party provider. The user must be able to input credit card details securely. Success: A test transaction successfully processes and appears in the OHC dashboard as 'Paid'.
**Priority:** P0
**Estimated Scope:** Large
