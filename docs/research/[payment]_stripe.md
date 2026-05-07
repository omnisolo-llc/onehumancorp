# Title: Standardize on Stripe for Global Payments and POS

**Problem Statement:** All personas need to accept payments online (Maya, Leo) or in-person (Priya, Fatima).

**Research Report:** Stripe is the backbone of global commerce, supporting 135+ currencies, online checkout, Billing (subscriptions for Leo), and Terminal (in-person POS for Priya). It also offers Connect for multi-tenant SaaS.

**Design Doc:** Deepen Stripe integration using Stripe Connect. Enable Stripe Connect to handle onboarding and compliance. Integrate Terminal support to allow physical tap-to-pay via the mobile app, alongside seamless online checkout.

**Implementation Prompt:** Upgrade the payment flow to use Stripe Connect. Add support for Stripe Terminal in the Flutter mobile app so users can take tap-to-pay in-person.

**Priority:** P0

**Estimated Scope:** Large
