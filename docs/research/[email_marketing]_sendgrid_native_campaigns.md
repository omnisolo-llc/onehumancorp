# Native Email Campaigns Powered by SendGrid

**Category:** Email Marketing
**Priority:** P1
**Estimated Scope:** Large

## Problem Statement
Small business owners find external tools like Mailchimp too complex for simple newsletters. They want a basic, built-in way to email their customer list directly from OHC.

## Research Report
OHC can offer native email campaigns using a transactional email provider like SendGrid under the hood. This provides a seamless UX.

**Key Advantages & Risks:**
Advantages: Huge value-add. Keeps users entirely within the OHC ecosystem. Simple UX.
Risks: High risk of IP blacklisting if users import spam lists. Handling bounce/complaint webhooks is complex but mandatory.

**Rough Pricing Estimate:**
SendGrid costs ~$19.95/mo for 50k emails. In Cloud mode, OHC absorbs or marks this up.

**Cloud vs. Standalone Modes:**
Cloud: OHC manages the master SendGrid account and uses Subusers or API Keys per tenant.
Standalone: The user must provide their own SendGrid API key.

## Design Doc
A 'Campaigns' tab allows users to select segments. A simple WYSIWYG editor is provided. OHC dispatches the emails via SendGrid's API and processes webhooks to track opens.

## Implementation Prompt
Implement a native email campaign sender within OHC. Business owners should be able to write an email, select customers, and click send. Handle deliverability and mandatory unsubscribe links automatically.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
We must implement a strict list-cleaning protocol before allowing users to import external CSVs to protect domain reputation.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The WYSIWYG editor should prioritize mobile responsiveness, as over 60% of marketing emails are read on phones.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
SendGrid's Event Webhook can fire thousands of times per minute during a large campaign; the ingestion pipeline must be highly scalable.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
A/B testing capabilities could be added in a later phase, allowing users to test subject lines easily.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Unsubscribe links must be immutable and automatically appended to the footer of every outbound campaign to comply with CAN-SPAM.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
