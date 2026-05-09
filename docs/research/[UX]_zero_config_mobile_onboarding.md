# [UX] Zero-Config Mobile Onboarding

## Problem Statement
The drop-off rate between signing up for a platform and actually launching a store is massive. Competitors like Shopify require users to navigate complex web-based dashboards to configure shipping zones, tax rates, and payment gateways before going live. For a user like Maya (baker selling via Instagram), this is an insurmountable barrier. She needs a store live in minutes, directly from her phone.

## Research Report
- **Competitive Gap:** Most platforms treat mobile as a secondary companion app. GoDaddy Airo attempts mobile-first setup but results in generic, low-quality storefronts.
- **User Pain:** App Store reviews for major e-commerce platforms frequently mention "impossible to set up on my phone" and "too many steps."
- **Data Point:** Mobile-only usage is the primary internet interaction mode for a growing percentage of the global population, especially in emerging markets (potential future expansion).

## Design Doc
- **Architecture Idea:** Abstract all complex configuration into sensible defaults based on the user's location and business type, powered by a setup wizard agent.
- **UX Flow (Mobile 375px First):**
  1. User downloads app/opens PWA.
  2. Enters Business Name & Industry.
  3. AI Agent instantly provisions a default store (default shipping = local pickup + flat rate national, default tax = local rate).
  4. User is presented with a "Your store is live" screen and a single button: "Add your first product (Take Photo)."
- **Key Relationships:** Tight integration between the onboarding flow, user profile generation, and default tenant settings.

## Implementation Prompt
Design and implement a mobile-first (375px native), zero-configuration onboarding flow. The system must allow a user to launch a functional store by only providing their business name and type. All other settings (shipping, taxes, layout) must be intelligently defaulted by the backend. The primary metric for success is reducing the "Time to First Product" to under 2 minutes using only a mobile device.

## Priority
P0

## Estimated Scope
Medium
