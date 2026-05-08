# Title: Meta Graph API Integration for Unified Inbox

## Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram DMs, Facebook comments, and WhatsApp. It's overwhelming, and missed messages mean lost sales. They need a single place to see and reply to all customer inquiries.

## Research Report
The Meta Graph API allows businesses to connect their Instagram, Facebook, and WhatsApp accounts to external tools.
- **Ease of use:** Requires OAuth flow, which can be tricky for non-technical users but is standard.
- **Pricing:** WhatsApp requires some payment per conversation after the first 1000, while IG/FB is free to connect.
- **Reputation:** Meta is the industry standard for these channels.
- **Key advantages:** Massive reach, combines three major platforms into a single API.
- **Risks:** The OAuth review process for Meta apps can be notoriously difficult and slow. API changes frequently break integrations.
- **Environment:** Cloud works perfectly via Webhooks. Standalone might be tricky since webhooks require a public endpoint, but polling or local tunneling could be evaluated.

## Design Doc
- User goes to "Integrations" and clicks "Connect Facebook/Instagram".
- OHC redirects the user through the Meta OAuth flow.
- OHC registers a webhook to listen for incoming DMs and comments.
- Incoming messages show up in the OHC unified inbox.
- User replies from OHC, which triggers a webhook to send the message back to the customer via Meta.

## Implementation Prompt
Create a Meta Integration flow where users can log in with Facebook, select the pages they manage, and authorize OHC. Ensure incoming messages appear in the Unified Inbox, and any reply from the business owner is sent back correctly. The UI should show which platform the message came from.

## Priority
P1

## Estimated Scope
Medium
