# Title: Twilio Integration for Global SMS Notifications

## Problem Statement
Many small business customers, especially those with lower English proficiency or less technical background, do not check emails reliably. SMS is the only way to ensure they see appointment reminders or payment links, reducing no-shows and late payments.

## Research Report
Twilio is the industry standard for programmatic SMS and WhatsApp.
- **Ease of use:** Developers handle the API. Business owners just buy a number or register an Alphanumeric Sender ID.
- **Pricing:** Very cheap per message (cents), scales with usage.
- **Reputation:** Top tier, extremely reliable globally.
- **Key advantages:** Global reach, unmatched reliability, and simple API.
- **Risks:** The new A2P 10DLC regulations in the US require businesses to register their traffic, which can be confusing for small business owners and delay their ability to send messages.
- **Environment:** Cloud works perfectly. Standalone works perfectly via outbound API calls.

## Design Doc
- User goes to "Notifications" and enables SMS.
- User configures a Twilio API key (or OHC provides a managed number).
- Automated workflows (e.g., "Appointment tomorrow", "Invoice due") trigger an outbound SMS API call.
- Delivery status is logged and visible to the business owner.

## Implementation Prompt
Integrate Twilio to send outbound SMS. Allow users to input their Twilio Account SID and Auth Token. Create a trigger mechanism so that upcoming appointments send an SMS reminder 24 hours in advance. Log all sent messages in a "Communications" tab.

## Priority
P0

## Estimated Scope
Small
