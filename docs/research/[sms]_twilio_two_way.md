# Twilio Two-Way SMS Inbox

**Category:** SMS & Notifications
**Priority:** P2
**Estimated Scope:** Large

## Problem Statement
Customers often reply to automated SMS notifications, but those replies go nowhere. Business owners want to text their customers and manage those conversations.

## Research Report
Twilio is the industry standard for programmable SMS. Integrating two-way SMS transforms notifications into a conversational channel.

**Key Advantages & Risks:**
Advantages: Deepens customer relationships. Highly requested feature for high-touch service businesses.
Risks: Provisioning phone numbers requires KYC (Know Your Customer) compliance. Managing state between SMS threads is tricky.

**Rough Pricing Estimate:**
$1.15/month per phone number + $0.0079 per message sent/received.

**Cloud vs. Standalone Modes:**
Cloud: OHC manages sub-accounts via Twilio Organizations.
Standalone: User uses their own Twilio Account SID and Auth Token.

## Design Doc
Users can 'Buy a Number' within OHC. Incoming SMS triggers a Twilio webhook to OHC, which routes the message to the unified inbox.

## Implementation Prompt
Build a two-way SMS feature that allows business owners to text with their customers exactly like they email them. Incoming text messages should appear in the OHC unified inbox.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
MMS support (sending images) is highly desired by businesses like auto mechanics to send pictures of parts; this must be supported via Twilio MediaURLs.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Spam filtering for incoming messages must be considered, potentially leveraging Twilio's Advanced Opt-Out features.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Conversations via SMS lack subject lines; the UI must intelligently group chronological messages into threads for readability.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Toll-free numbers vs local numbers have different verification requirements that the onboarding UI must explain clearly.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
If a business has multiple staff members, SMS routing logic might need to assign incoming texts based on recent customer interactions.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
