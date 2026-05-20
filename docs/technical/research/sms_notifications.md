# [SMS & Notifications] Global SMS Alerts

## Problem Statement
For users like Fatima (food cart operator), checking an app or email constantly isn't feasible while working. She needs an immediate, loud text message on her basic smartphone the second an order is placed so she can start preparing the food.

## Research Report
- **Target Tools**: Twilio API or MessageBird.
- **Competitive Analysis**: Many platforms charge extra for SMS. Offering this out-of-the-box for critical alerts is a strong differentiator for specific personas (food, urgent services).
- **Ease of Use**: Completely invisible setup. The user just enters their phone number and checks a box for "Text me when I get a new order."
- **Pricing**: ~$0.01 - $0.05 per message depending on the country. Costs need to be managed (e.g., limited free texts per month, unlimited on paid plans).
- **Reputation**: Twilio is the gold standard for global SMS delivery.
- **Advantages and Risks**: Ensures operators like Fatima don't miss orders. Risk is high cost per message and strict compliance (A2P 10DLC) rules in the US.
- **Cloud vs Standalone**: Cloud uses central Twilio account. Standalone cannot use central SMS; users would need their own Twilio credentials, rendering it unusable for non-technical users.

## Design Doc
- **Integration Flow**: In the "Operations" or Profile settings, users verify their mobile number and enable SMS alerts.
- **Actions**: When a specific trigger occurs (e.g., Order Paid), the system dispatches an SMS via the Twilio API to the owner's phone.
- **User Experience**: A simple toggle: "Send me a text message for new orders." The received text is concise: "OHC Alert: New order #123 for $15.00 - Chicken Over Rice."

## Implementation Prompt
Build an SMS notification service integrated with Twilio that allows business owners to opt-in to receive text message alerts for critical events, such as new orders or bookings. The feature must include a simple phone number verification flow and toggle switches to control which events trigger an SMS. The notification content must be concise and informative.

## Priority
P1

## Estimated Scope
Small
