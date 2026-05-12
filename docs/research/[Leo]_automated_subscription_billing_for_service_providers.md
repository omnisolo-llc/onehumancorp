# Issue Brief: Automated Subscription Billing for Service Providers (Leo)

## Problem Statement
Leo (Music Tutor, 22) teaches 15 students weekly. His biggest headache is "chasing invoices." He hates the awkwardness of asking parents for money every week. He needs a system that moves clients from "One-off payments" to "Subscriptons" automatically, but most platforms charge high fees or require complex "Pro" plans for recurring billing.

## Research Report
- **Competitor Audit**: Squarespace and Wix hide subscription billing behind their most expensive tiers ($30-$50/mo).
- **Pain Point**: "The Payment Chase" - solopreneurs lose hours of billable time to manual invoicing.
- **Opportunity**: OHC can treat "Recurring Value" as a core primitive, not a premium add-on.

## Design Doc
### High-Level Architecture
- **Inertial Conversion**: When a customer books a second "One-off" session, the Advisor agent suggests: "You've booked twice! Would you like to save 10% by switching to a weekly subscription?"
- **Automated Recovery**: The Accountant agent manages failed cards with "Gentle Reminders" instead of hard account locks.
- **Dashboard Visibility**: Leo sees a "Monthly Recurring Revenue (MRR)" card as his primary health metric.

### Mobile UX Flow (375px)
1. **Owner View**: A "Subscriptions" tab showing active students and their next billing date.
2. **Customer View**: A 1-tap "Upgrade to Subscription" button during the checkout flow for repeat services.

### AI Agent Integration
- **The Accountant**: Managing the subscription lifecycle, failed payments, and financial reporting.
- **The Salesperson**: Proactively identifying "Repeat Customers" and offering subscription upsells.

## Implementation Prompt
Implement a "Subscription Upsell" loop. When a customer completes their second booking for the same service type within 30 days, the "Salesperson" agent should trigger a localized offer to the customer (via email/SMS) to convert to a recurring subscription at a slight discount. The "Accountant" should then manage the Stripe/Payment gateway subscription object and provide a simplified "MRR" view in the owner's dashboard.

## Priority
P2

## Estimated Scope
Medium
