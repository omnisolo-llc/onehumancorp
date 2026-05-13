# Acuity Scheduling Embedded Booking

**Category:** Calendar & Scheduling
**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
Service-based businesses already use advanced scheduling tools like Acuity and don't want to migrate their entire setup to OHC. They just want their existing booking system to work smoothly.

## Research Report
Acuity Scheduling has a massive footprint in the service industry. OHC can offer a deep embedding and webhook integration. This captures lead data while Acuity handles the complex logic.

**Key Advantages & Risks:**
Advantages: Respects existing workflows. Avoids rebuilding complex scheduling logic (resource routing, classes).
Risks: Dependency on a third-party iframe. Webhook delivery failures could result in missing customer data.

**Rough Pricing Estimate:**
Free for OHC to integrate. The user pays their existing Acuity subscription (approx $20-$50/month).

**Cloud vs. Standalone Modes:**
Cloud: Fully supported via shared webhook receiver.
Standalone: Supported, provided the standalone instance is publicly routable to receive webhooks from Acuity.

## Design Doc
Users paste their Acuity scheduling link. OHC provides a native-feeling embed block. In the background, OHC registers webhooks with Acuity so bookings sync to the OHC CRM.

## Implementation Prompt
Build an Acuity Scheduling integration that allows users to embed their Acuity booking page into their OHC storefront. Automatically capture customer details from new Acuity appointments into the OHC unified customer list.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
Acuity allows custom form fields during booking; mapping these dynamic fields to OHC's static customer schema is a key challenge.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The embedded iframe should be styled dynamically using URL parameters to match the OHC storefront's theme colors.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Handling appointment cancellations via webhook is crucial to ensure OHC does not trigger automated follow-up campaigns for canceled sessions.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
We should investigate if Acuity's API allows querying historical appointments during the initial setup to populate the CRM immediately.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Consider adding support for Acuity's class/group booking features, which differ slightly in payload structure from 1-on-1 sessions.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
