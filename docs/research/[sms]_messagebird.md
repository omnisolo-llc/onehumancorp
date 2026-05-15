# [sms] MessageBird SMS & Notifications Integration

## Problem Statement
Small business owners operating globally or in regions with lower email penetration (and lower English proficiency, like Fatima's persona) require reliable SMS capabilities. They need to send appointment reminders, order updates, and promotional messages directly to customers' phones. Integrating MessageBird provides a robust, international SMS gateway directly within the OHC platform.

## Research Report
### Overview
MessageBird (now part of Bird) is a global omnichannel communication platform with strong SMS routing capabilities, particularly outside of North America (where Twilio often dominates).

### Ease of Use
The integration should be transparent to the business owner. They would purchase an SMS add-on or credit bundle within OHC. OHC handles the backend integration with MessageBird, allowing the owner to simply type a message in the unified inbox and hit "Send via SMS."

### Reputation
MessageBird is highly regarded for its global carrier connectivity, competitive international pricing, and high deliverability rates.

### Pricing
Pricing is per-message and varies significantly by country. OHC could absorb this cost into a higher-tier subscription or offer a pay-as-you-go credit system for the business owner.

### Environment
Works in Cloud.

### AI Integration
High potential. AI can auto-translate SMS messages based on the customer's preferred language, summarize long conversations to fit within the 160-character limit, or generate concise promotional texts.

## Design Doc
1.  **Configuration:** OHC administrators configure the global MessageBird API credentials. Business owners do not interact with MessageBird directly.
2.  **Unified Inbox:** In the OHC unified inbox, a user can select "SMS" as the outbound channel when communicating with a customer (provided the customer has a valid phone number on file).
3.  **Automated Notifications:** Business owners can toggle SMS notifications for system events (e.g., "Send SMS when order ships").
4.  **Inbound SMS:** If OHC provisions dedicated phone numbers via MessageBird, inbound SMS replies are routed back into the unified inbox.

## Implementation Prompt
Implement a global SMS gateway using MessageBird. The integration should be configured at the platform level (not per-tenant). Add an "SMS" tab to the unified inbox interface, allowing the business owner to send text messages directly to the customer's phone number. Ensure the character count is clearly displayed and handles multi-part messages correctly. Implement webhook listeners to handle delivery receipts and inbound replies, routing them back to the appropriate customer conversation thread in OHC.

## Priority
P1 (High) - Essential for global reach and diverse user demographics.

## Estimated Scope
Medium
