# MessageBird Global SMS Notifications

**Category:** SMS & Notifications
**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
Business owners in regions with low email open rates need to send order updates and appointment reminders via SMS to ensure they are seen.

## Research Report
MessageBird offers extensive global carrier coverage and competitive pricing. SMS is crucial for immediate notifications.

**Key Advantages & Risks:**
Advantages: Exceptionally high read rates (98%). Superior to Twilio in specific European/Asian markets for pricing.
Risks: A2P 10DLC compliance in the US is extremely rigorous and can delay onboarding for weeks.

**Rough Pricing Estimate:**
Varies by country. ~$0.007 in the US, but significantly higher in parts of Europe.

**Cloud vs. Standalone Modes:**
Cloud: Can be offered as a premium add-on billed centrally.
Standalone: User brings their own MessageBird API key.

## Design Doc
Notification templates can be toggled between Email and SMS. OHC uses the MessageBird REST API to dispatch messages asynchronously.

## Implementation Prompt
Add SMS as a primary notification channel for critical customer updates. Provide a clean interface for customizing short SMS templates.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
Handling international phone number formatting (E.164) is critical; libphonenumber should be utilized to sanitize inputs.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Due to character limits (160 for GSM-7), the template editor must include a real-time character counter and segment estimator.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Webhooks for delivery receipts must be processed so business owners can see if a message failed due to a bad number.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Opt-out handling (replying STOP) must be managed, automatically blacklisting the number in OHC to maintain compliance.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Alphanumeric Sender IDs should be supported for countries that allow them (e.g., UK, AUS) to increase brand recognition.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
