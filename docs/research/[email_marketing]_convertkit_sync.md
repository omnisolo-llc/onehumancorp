# ConvertKit Audience Synchronization

**Category:** Email Marketing
**Priority:** P2
**Estimated Scope:** Medium

## Problem Statement
Creators use ConvertKit for complex email automation, but their customer purchase data is in OHC. They waste time manually exporting and importing CSV files to keep segments up to date.

## Research Report
ConvertKit is popular among creators. Their API allows for rich subscriber management, including adding tags based on purchase behavior.

**Key Advantages & Risks:**
Advantages: Seamlessly hands off marketing to a specialized tool. Keeps creator workflows intact.
Risks: API rate limits during massive sales events (e.g., Black Friday) could stall the sync queue.

**Rough Pricing Estimate:**
Free API usage. User pays ConvertKit based on subscriber count (starting at $9/month).

**Cloud vs. Standalone Modes:**
Cloud: Supported. OHC manages the outbound API queue.
Standalone: Supported. User inputs their own ConvertKit API Key.

## Design Doc
User enters ConvertKit API Secret. User maps OHC product purchases to ConvertKit tags. When an order is completed, OHC sends a background job to update the subscriber profile.

## Implementation Prompt
Create an integration with ConvertKit that automatically syncs OHC customers to the user's ConvertKit audience. Provide an interface to specify which tags should be applied when a customer buys a product.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
ConvertKit's subscriber-centric model means a user is uniquely identified by email; we must gracefully handle email updates in OHC.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The tag mapping UI should dynamically fetch available tags from ConvertKit rather than requiring manual text entry.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
If a customer requests data deletion (GDPR), OHC must ideally broadcast a deletion request to ConvertKit as well.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Consider supporting ConvertKit's 'Custom Fields' API to sync OHC lifetime value (LTV) metrics for advanced segmentation.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
A manual 'Force Sync' button is highly recommended for users who suspect data discrepancies.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
