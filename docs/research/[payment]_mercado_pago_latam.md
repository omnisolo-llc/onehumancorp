# Mercado Pago Integration for LATAM

**Category:** Payment Processing
**Priority:** P0
**Estimated Scope:** Large

## Problem Statement
Business owners in Brazil, Mexico, and Argentina need a local payment gateway that supports local payment methods like PIX and OXXO to avoid cart abandonment.

## Research Report
Mercado Pago is the dominant payment processor in Latin America. Their Checkout Pro provides a hosted payment page, handling local compliance and complex methods.

**Key Advantages & Risks:**
Advantages: Unlocks massive LATAM market. Automatically supports highly popular local methods like PIX and Boleto.
Risks: Asynchronous payment confirmations (e.g., OXXO cash payments) mean orders remain 'Pending' for days.

**Rough Pricing Estimate:**
Standard processor fees (e.g., 3-5% + flat fee) paid by the merchant. No direct cost to OHC.

**Cloud vs. Standalone Modes:**
Cloud: Supported via Mercado Pago Connect (OAuth).
Standalone: Supported via user-provided API credentials.

## Design Doc
Add Mercado Pago as an alternative payment provider. The customer is redirected to the Mercado Pago hosted flow. Webhooks reliably confirm the payment.

## Implementation Prompt
Integrate Mercado Pago to unlock the LATAM market. Provide a simple setup process. Ensure the checkout experience supports local payment methods seamlessly and updates order statuses.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
Handling multi-currency conversions gracefully is essential, as merchants might price in USD but charge in BRL or MXN.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The Checkout Pro integration completely offloads PCI compliance, which is highly desirable for standalone deployments.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
PIX payments are nearly instant; the webhook receiver must process these confirmations in real-time to avoid customer anxiety.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
We need to document the testing process thoroughly, as Mercado Pago's sandbox environment has specific requirements for test credit cards.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Refunds via Mercado Pago API must be supported directly from the OHC order dashboard to prevent users from needing the MP dashboard.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
