# [billing] Subscription and Recurring Revenue

## Title
Radically Simple AI-Managed Subscriptions

## Problem Statement
Leo (Music Tutor, 22) wants to move away from "one-off" payments to recurring monthly subscriptions for his lessons to ensure stable income. However, setting up subscriptions in tools like Stripe or Shopify is "confusing and technical." He needs an AI agent to handle the billing cycles, failed payment retries, and renewal reminders invisibly.

## Research Report
- **Competitor Audit**:
    - **Shopify**: Subscriptions require 3rd party apps (Recharge, Bold) which add significant cost ($50+/mo) and complexity.
    - **Squarespace**: Has built-in subscriptions, but requires a high-tier plan and offers zero AI management of failed payments.
    - **Patreon / Substack**: Great for creators, but doesn't work for service providers like tutors or lawn care.
- **Data**: Retention for recurring revenue businesses is 2-3x higher than transactional ones.
- **Evidence**: Leo's pain is "Manual Billing Chaos." He wants a "Set it and forget it" system.

## Design Doc
- **Architecture**:
    - Entity Type: `SubscriptionPlan`, `Member`.
    - Integrated with OHC's existing Stripe routing logic.
- **Screen Flow (375px)**:
    - User creates a product and toggles [X] Recurring (Monthly).
    - AI "Accountant" agent monitors successful payments and flags "Churn Risks" (expired cards) in the Action Feed.
- **AI Agent Integration**:
    - "The Ambassador" sends friendly, personalized renewal reminders.
    - "The Accountant" suggests switching a customer to a subscription if they've booked 3 individual sessions in 2 months.

## Implementation Prompt
Implement a native subscription management module that allows merchants to turn any product or service into a recurring revenue stream with one toggle. The AI agent must proactively handle the lifecycle of the subscription.
- **Critical User Journey**: Leo creates "Weekly Guitar Lessons" -> Toggles "Monthly Billing" -> Customer signs up once -> AI handles all future charges and emails.
- **Acceptance Criteria**: Merchants can create recurring products. Customers can manage/cancel via a simple OHC portal. AI identifies and alerts on failed payments with pre-drafted recovery emails.
- **Priority**: P1
- **Estimated Scope**: Medium
