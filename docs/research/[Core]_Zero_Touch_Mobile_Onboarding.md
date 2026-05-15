# Zero-Touch Mobile Onboarding

**Priority:** P0
**Scope:** Medium

## Problem Statement
Maya (baker) finds Shopify too complex. She needs a storefront live in under 5 minutes using only her phone.

## Research Report
- **Shopify:** Mobile setup is clunky.
- **GoDaddy Airo:** Quick but aggressive upselling and low quality.
- **Conclusion:** An onboarding flow that builds the store via a natural language conversation (or simple image uploads) will drastically reduce drop-off rates.

## Design Doc
- **Architecture:** `OnboardingAgent` interacts with `User`. Generates `StoreConfig`, `ThemeConfig`, and initial `Products`.
- **UX Flow:** Chat interface: "What do you sell?" -> Uploads photos -> Agent builds store instantly.

## Implementation Prompt
Create an onboarding sequence driven entirely by an AI conversation. The user answers 3-5 questions or uploads a menu/price list, and the system provisions a fully functional storefront. Acceptance Criteria: Store must be live and able to accept orders within 3 minutes of starting.
