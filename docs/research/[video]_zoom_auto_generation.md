# Zoom Meeting Auto-Generation for Appointments

**Category:** Video Conferencing
**Priority:** P0
**Estimated Scope:** Medium

## Problem Statement
Consultants and tutors spend tedious minutes manually creating Zoom links for every online booking. They want this to happen automatically.

## Research Report
Zoom is the dominant video conferencing platform. The Zoom API allows for programmatic creation of meetings.

**Key Advantages & Risks:**
Advantages: Saves significant time. Professional presentation to customers.
Risks: Zoom's strict OAuth refresh token lifetimes can cause unexpected disconnections requiring user re-authentication.

**Rough Pricing Estimate:**
Free API access. User requires their own Pro Zoom account if meetings exceed 40 minutes.

**Cloud vs. Standalone Modes:**
Cloud: A centralized OHC Zoom OAuth app makes connection a 1-click process.
Standalone: User must create a Server-to-Server OAuth app in Zoom Marketplace, which is highly technical.

## Design Doc
When a customer books an 'Online' service type, OHC automatically authenticates with Zoom, creates a unique meeting link, and injects it into emails.

## Implementation Prompt
Automate Zoom link generation for online appointments. A business owner should simply connect their Zoom account once. Any online booking should automatically generate a unique Zoom meeting.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
The integration should default to enabling 'Waiting Rooms' and 'Passcodes' to prevent Zoombombing and ensure privacy.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
When an appointment is rescheduled in OHC, the integration must automatically update the start time via the Zoom API.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
If an appointment is canceled, the associated Zoom meeting should be explicitly deleted to keep the host's Zoom dashboard clean.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
For businesses with multiple staff, the integration must support assigning meetings to different Zoom sub-accounts or licensed users.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The generated Join URL must be distinctly highlighted in the confirmation email UI to prevent customer confusion.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
