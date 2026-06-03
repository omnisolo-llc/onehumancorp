# [operations] Agentic Booking & Quoting

## Problem Statement
Service businesses, like handymen or tutors, often lose leads because they cannot respond instantly when a customer submits an inquiry on their website. The current booking tools are rigid and do not handle complex quoting needs.

## Research Report
Based on reviews of Shopify, Wix, and Durable, users frequently complain about the lack of integrated, intelligent follow-up. For example, Durable provides a simple CRM but requires the user to manually draft quotes. Our review of `r/sweatystartup` shows contractors losing up to 30% of leads due to response delays.
Our platform must provide an "Agentic Operations Manager" to auto-draft quotes and secure deposits instantly.

## Design Doc
- **Architecture**: A new "Teammate Mesh" workflow connecting the public storefront form to the `Salesperson` and `Manager` agents.
- **Data Model**: `Quote` entity linked to `Lead` and `Service`.
- **UI/UX**:
  - Customer fills out natural language request on mobile-first storefront (375px optimized).
  - Business owner receives an immediate push notification with an AI-drafted quote.
  - Owner clicks "Approve & Send" (1-tap).
  - Customer receives SMS link to approve quote and pay deposit via Stripe integration.

## Implementation Prompt
Implement a unified quoting and booking flow. Add a "Get a Quote" block to the storefront builder. When a user submits this form, the AI should parse the request and generate a draft quote based on the business's service catalog. Send a notification to the business owner to approve the quote with one tap, which then sends a payment link to the customer.
Ensure the UI passes the Grandmother test with 44x44px touch targets.

## Priority
P0

## Estimated Scope
Large
