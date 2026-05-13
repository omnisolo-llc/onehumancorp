# Alipay and WeChat Pay Support

**Category:** Payment Processing
**Priority:** P2
**Estimated Scope:** Medium

## Problem Statement
Businesses catering to Chinese consumers lose sales because they only accept Western credit cards. They need the ability to accept Alipay and WeChat Pay seamlessly.

## Research Report
Alipay and WeChat Pay are ubiquitous in China. Integrating these often requires a cross-border payment aggregator (like Stripe or Adyen).

**Key Advantages & Risks:**
Advantages: Captures a highly lucrative, fast-growing demographic. High trust factor for Chinese consumers.
Risks: Extremely complex direct integration; reliance on aggregators like Stripe is mandatory to avoid massive compliance overhead.

**Rough Pricing Estimate:**
Aggregators typically charge 2.9% + 30¢ or slightly higher for APMs.

**Cloud vs. Standalone Modes:**
Cloud: Readily supported if leveraging Stripe's APM features.
Standalone: Supported, assuming the user's Stripe account is approved for these APMs.

## Design Doc
Expose Alipay and WeChat Pay as toggles. The checkout flow will display a dynamically generated QR code. OHC will rely on webhooks to confirm authorization.

## Implementation Prompt
Enable Alipay and WeChat Pay options for businesses using supported payment gateways. The checkout process must smoothly present the necessary QR codes to the buyer.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
Stripe's Payment Element makes exposing these APMs relatively straightforward on the frontend.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
WeChat Pay requires specific currency configurations (often forcing settlement in specific local currencies) that must be accounted for.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The UI must elegantly handle the scenario where a user scans a QR code but the webhook is delayed.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Mobile optimization is paramount, as many users will be completing the checkout flow directly within the WeChat in-app browser.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Proper error handling for regional blocks or failed cross-border authorizations must be clear to the end buyer.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
