# Mobile-First 10-Minute Setup

## Problem Statement
Small business owners (like Maya the baker or Carlos the handyman) find existing platforms like Shopify too complex. They want to set up their business online using just their phone in under 10 minutes, without needing a desktop or understanding technical jargon like "DNS" or "Payment Gateways."

## Research Report
- 65% of 1-star App Store reviews for legacy builders cite "too confusing" or "needs a desktop to actually work."
- Durable showed users love 30-second website generation, but lack the backend tools to actually run the business.
- Users want to be guided by plain language, not a complex dashboard.

## Design Doc
- **Core Entities**: `Tenant`, `Storefront`, `BusinessProfile`
- **UX Flow**:
  1. Welcome Screen (Mobile 375px optimized).
  2. 3-Question Vibe Check (Business name, what you sell, brand vibe).
  3. Loading screen while AI generates the initial layout.
  4. Instant preview.
  5. One-tap "Connect Bank" (Plaid/Stripe simplified).
- **AI Integration**: AI generates the initial storefront structure, theme, and copy based on the 3 questions.

## Implementation Prompt
Implement a mobile-first wizard that guides a new user from account creation to a fully functional, AI-generated storefront in under 5 screens. The flow must use plain language only (e.g., "Where should we send your money?" instead of "Configure Payment Gateway"). The final screen must present a live, shareable URL. Ensure all technical fields are hidden by default behind the Progressive Disclosure Pattern.

## Priority
P0

## Estimated Scope
Medium
