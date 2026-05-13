# ShipStation Order Synchronization

**Category:** Shipping & Logistics
**Priority:** P0
**Estimated Scope:** Medium

## Problem Statement
E-commerce business owners spend hours manually copying order addresses from OHC into their shipping software to print labels.

## Research Report
ShipStation is the most popular multi-carrier shipping software for SMBs. The integration requires OHC to act as a 'Custom Store' for ShipStation.

**Key Advantages & Risks:**
Advantages: Solves fulfillment for medium-to-large sellers. Unlocks access to hundreds of global carriers negotiated by ShipStation.
Risks: Building a Custom Store API requires conforming exactly to their legacy XML schema, which can be rigid and brittle.

**Rough Pricing Estimate:**
Free for OHC. Users pay ShipStation (starting at $9.99/mo).

**Cloud vs. Standalone Modes:**
Cloud & Standalone: Both supported. ShipStation actively polls the OHC URL, so Standalone instances must be accessible via public internet.

## Design Doc
OHC will implement a standardized XML API endpoint conforming to ShipStation's Custom Store spec. Orders will flow automatically.

## Implementation Prompt
Build a ShipStation Custom Store integration. Orders placed in OHC should instantly appear in ShipStation. When a label is printed in ShipStation, OHC should mark the order as fulfilled and attach tracking.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
ShipStation's polling mechanism relies heavily on the 'Last Modified' timestamp; database triggers must reliably update this field on any order mutation.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Custom fields in OHC (like gift messages or special instructions) must be mapped correctly to ShipStation's notes fields.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The integration should support mapping OHC shipping methods (e.g., 'Expedited') to specific requested services in ShipStation.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Handling partial fulfillments (split shipments) sent back from ShipStation requires careful state management in the OHC order table.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Authentication for the XML endpoint should utilize simple Basic Auth over HTTPS, as per ShipStation's documentation.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
