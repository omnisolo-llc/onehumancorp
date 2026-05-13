# Microsoft Teams Meeting Integration

**Category:** Video Conferencing
**Priority:** P2
**Estimated Scope:** Medium

## Problem Statement
B2B service providers and corporate consultants often standardize on Microsoft Teams. They need their scheduling tool to automatically generate MS Teams links.

## Research Report
Microsoft Graph API provides access to create Teams meetings. Many enterprise-adjacent SMBs require this over Zoom.

**Key Advantages & Risks:**
Advantages: Meets B2B expectations. Leverages existing Office 365 investments.
Risks: The Microsoft Graph API is notoriously complex and Azure AD app permissions are difficult to navigate.

**Rough Pricing Estimate:**
Included in the user's existing Microsoft 365 Business subscription.

**Cloud vs. Standalone Modes:**
Cloud: Supported via a multi-tenant Azure AD App.
Standalone: Requires the user to register an app in their own Azure portal.

## Design Doc
Similar architecture to Zoom. When an appointment is scheduled, OHC calls the Graph API to generate an online meeting. The join web URL is stored and distributed.

## Implementation Prompt
Provide Microsoft Teams as an option for automated meeting link generation, catering to B2B-focused business owners. Automatically attach the generated Teams link to calendar invites.

## Deep Dive Architecture & Product Strategy

### Strategy Implication
Delegated permissions (OnlineMeetings.ReadWrite) must be carefully requested during the OAuth flow to adhere to least-privilege principles.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Teams meeting links can be exceptionally long; ensuring they don't break UI layouts in automated emails is a minor but necessary detail.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Unlike Zoom, Teams meetings can be tightly coupled to an Outlook Calendar event; we should leverage this dual-creation via the Graph API.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Support for generating dial-in numbers (PSTN coordinates) alongside the web link is crucial for enterprise clients.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.

### Strategy Implication
Clear documentation is required to explain to users the difference between personal Microsoft accounts and work/school accounts for this feature.

This directly impacts the engineering roadmap by requiring robust state management and error handling across distributed systems. The UX must abstract this complexity entirely from the small business owner, presenting only clear, actionable alerts if integration synchronicity is lost. Furthermore, considering our core tenet of privacy and data sovereignty, any data passed to these external tools must be explicitly consented to, and data retention policies must be clearly outlined during the OAuth or API key setup flows.

The technical debt associated with maintaining third-party APIs is non-trivial. Webhook schemas evolve, and API versions are deprecated. OHC must implement an active monitoring layer utilizing tools like Sentry to catch schema deviations before they cause widespread integration outages for our users. Rate limiting is another critical factor; implementing a resilient queue system (like Redis or NATS) is necessary to ensure we don't overwhelm external providers during peak business hours.
