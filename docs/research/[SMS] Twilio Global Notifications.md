# Title: SMS Notifications for Critical Alerts
## Problem Statement
Email open rates are low, and many customers (especially older demographics or those with low English proficiency) prefer text messages. Business owners need a reliable way to send booking reminders and order confirmations directly to customers' phones to reduce no-shows and increase trust.

## Research Report
* **Tool:** Twilio Programmable SMS
* **What it does:** Sends SMS and WhatsApp messages globally.
* **Ease of Use for Owners:** Medium. While Twilio is developer-focused, OHC will abstract this so the owner just toggles "Send SMS Reminders" on or off.
* **Pricing:** Pay-as-you-go per message (e.g., ~$0.0079 per SMS in the US). International rates vary.
* **Cloud vs. Standalone:**
  * Cloud: OHC can bill the owner for SMS usage or provide a limited free quota.
  * Standalone: Owner must provide their own Twilio API key and manage their own billing.

## Design Doc
* **Trigger:** Event occurs in OHC (e.g., "Booking Confirmed", "24 hours before appointment").
* **Action:** OHC triggers an API call to Twilio with a templated message.
* **User Experience:** The owner toggles a switch to enable SMS. Customers receive a simple text message with their appointment details and a link to reschedule.

## Implementation Prompt
Build automated SMS notifications. For cloud users, implement a seamless toggle; for standalone users, allow input of a Twilio API key. The acceptance criteria: when a customer books an appointment and provides a phone number, they must receive an immediate SMS confirmation, and the owner must see a log that the SMS was delivered successfully.

## Priority
P0

## Estimated Scope
Medium
