# WhatsApp Business Message Synchronization

**Category:** Social Media Integration
**Priority:** P0
**Estimated Scope:** Large

## Problem Statement
WhatsApp is the default communication tool in regions like LATAM, India, and parts of Europe. Business owners struggle to share WhatsApp access with staff and often mix personal and business communications.

## Research Report
WhatsApp Cloud API offers robust integration capabilities. The Cloud API is hosted by Meta and easier to maintain. SMBs use it for everything from taking orders to customer support.

**Key Advantages & Risks:**
Advantages: Deep penetration in global markets. High read rates. Supports rich media like catalogs.
Risks: Strict opt-in requirements and complex template approval processes can confuse non-technical users.

**Rough Pricing Estimate:**
Based on conversation categories (marketing, utility, service). Varies heavily by country, typically 1 to 5 cents per conversation.

**Cloud vs. Standalone Modes:**
Cloud: Leverage a multi-tenant WhatsApp Business Account managed by OHC.
Standalone: User must configure their own Meta Developer App and register a dedicated phone number.

## Design Doc
Incorporate WhatsApp as a channel in the unified inbox. Incoming messages append to existing customer profiles. Outgoing messages outside the 24-hour window will require pre-approved template selection from the UI. Configuration utilizes Meta's embedded signup.

## Implementation Prompt
Implement WhatsApp Business synchronization so users can manage WhatsApp chats directly within OHC. Provide a user-friendly interface for submitting and selecting message templates. The setup flow should utilize Meta's embedded signup.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
In markets like Brazil, WhatsApp is frequently used to finalize high-ticket purchases, requiring reliable message delivery.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Handling WhatsApp's unique message types (like location pins or contacts) will require specialized UI rendering components.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
The integration must support the ingestion of WhatsApp product catalogs if the merchant uses WhatsApp Commerce.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Compliance with Meta's Commerce Policy must be enforced, meaning restricted items cannot be actively promoted via this channel.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Opt-out flows (like responding 'STOP') must be natively handled to prevent the business from being banned.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
