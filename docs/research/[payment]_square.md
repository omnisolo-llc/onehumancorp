# Payment Processing: Square

**Problem Statement:** Retail and service-based SMBs need a seamless way to accept both in-person (POS) and online payments without managing two separate systems.

**Research Report:** Square is a dominant player for SMB offline/online payments, especially in the US/UK/AU.
- Ease of Use: Famous for its simple onboarding and intuitive hardware/software.
- Pricing: Transparent flat-rate pricing (e.g., 2.6% + 10c for in-person), no monthly fees for basic.
- Reputation: Excellent brand recognition among SMBs.
- Cloud vs. Standalone: Cloud-based, but hardware required for POS. API supports deep integration.

**Design Doc:**
- OHC integrates Square Web Payments SDK for online checkout on storefronts.
- OHC syncs inventory and sales data with Square API.
- UI wireframes or screen flow description (375px first): Standard checkout flow with Apple Pay/Google Pay support.
- Mobile UX flow: Owner views combined online/offline sales metrics in a single dashboard.

**Implementation Prompt:** Integrate Square as a primary payment gateway option for OHC storefronts. Support online payments and sync transaction history.
- Acceptance Criteria: Customers can checkout using Square. Transactions appear in the OHC financial dashboard.

**Priority:** P0
**Estimated Scope:** Large
