# Unified Instagram Direct Messages for SMB Inboxes

**Category:** Social Media Integration
**Priority:** P0
**Estimated Scope:** Large

## Problem Statement
Small business owners miss crucial leads and customer inquiries because they have to constantly switch between their primary inbox and the Instagram app on their phone. This causes delayed responses, lost sales, and frustrated customers. Managing Instagram DMs alongside emails is a major headache.

## Research Report
Instagram Direct (via Messenger API for Instagram) is the dominant messaging channel for retail and service SMBs. Integrating this allows business owners to manage conversations without technical hurdles. The Messenger API provides webhooks for real-time message receiving and standard REST endpoints for sending.

**Key Advantages & Risks:**
Advantages: Captures leads directly from social media. Eliminates app-switching. Increases response rate.
Risks: The strict 24-hour response window enforced by Meta means if a business owner replies late, the message delivery will fail.

**Rough Pricing Estimate:**
Primarily based on API usage, generally free for standard business use up to 1000 conversations/month.

**Cloud vs. Standalone Modes:**
Cloud: Can be handled seamlessly via an official OHC Meta App.
Standalone: Requires users to provide their own Meta App ID and Secret, which adds friction to the setup process.

## Design Doc
The integration will listen for incoming messages via Meta Webhooks and route them into the OHC unified inbox. Users will see Instagram DMs alongside standard emails. When replying, the action will send a request back to the Meta Graph API. The configuration screen will feature a simple 'Connect Instagram' button initiating the OAuth flow.

## Implementation Prompt
Create an integration that allows business owners to connect their Instagram Professional accounts to OHC. They should be able to receive incoming DMs in their existing OHC inbox and reply directly from there. Ensure the 24-hour response window is clearly indicated in the UI.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
Market research indicates that over 70% of fashion boutiques use Instagram DMs as their primary pre-sales support channel.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Users expect a seamless multimedia experience; thus, image and short video attachments must be properly rendered within the unified inbox.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
To prevent API rate limit issues, a robust queuing mechanism needs to be implemented on the backend to batch read receipts.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Analytics dashboards should expose the average response time for Instagram DMs, specifically highlighting interactions approaching the 24-hour cutoff.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Future enhancements could include AI-assisted auto-responses or quick-reply templates specific to Instagram.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
