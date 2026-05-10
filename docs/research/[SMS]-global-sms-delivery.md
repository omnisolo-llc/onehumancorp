# SMS & Notifications: Global SMS Delivery

## Title
Automated SMS Alerts for Customers

## Problem Statement
Many customers (especially in certain demographics or regions) do not check email frequently. Small business owners need to send appointment reminders, delivery updates, or important alerts via SMS to reduce no-shows and improve service.

## Research Report
- **Tools Evaluated:** Twilio, MessageBird, AWS SNS, Vonage.
- **Ease of Use:** Twilio is the developer standard but can be complex regarding A2P 10DLC compliance in the US.
- **Pricing:** Varies wildly by country. US is ~$0.007/msg, UK is ~$0.04/msg.
- **Reputation:** Twilio has the best global coverage. MessageBird is strong in Europe.
- **Cloud vs Standalone:** Works well in both via outbound API calls.

## Design Doc
- **Trigger:** An appointment is booked, or an order is shipped.
- **Action:** OHC triggers an SMS via the provider API.
- **User View:** Users can toggle "Send SMS Reminders" in their settings. They see a log of sent SMS messages on the customer's profile.

## Implementation Prompt
Add SMS notification capabilities for critical events like appointment reminders or order updates. The business owner should simply toggle SMS "On", and OHC handles the routing (via a provider like Twilio). Provide a simple way for the business owner to see if an SMS was delivered and handle basic opt-outs (STOP).

## Priority
P1

## Estimated Scope
Medium
