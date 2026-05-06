# Integrate Twilio for Global SMS Notifications

## Problem Statement
Many small businesses interact with clients who may not regularly check email or who lack high English proficiency (e.g., relying heavily on direct phone communication). For critical updates like appointment reminders, order confirmations, or urgent service changes, SMS is the most reliable channel. OHC needs a robust way to send automated SMS notifications globally.

## Research Report
**Findings & Data**: Twilio is a premier cloud communications platform providing programmable tools for making and receiving phone calls and text messages.
**Ease of Use**: While the API is developer-focused, the end-user experience in OHC will be seamless. The business owner only needs to provide their Twilio credentials to enable the feature.
**Features**: Exceptional global carrier coverage, high deliverability rates, and built-in compliance handling (e.g., handling "STOP" messages).
**Pricing**: Pay-as-you-go pricing per message segment. Costs vary significantly by destination country but are generally very low for domestic messages.
**Reputation**: Industry standard for programmable SMS. Highly reliable.

## Design Doc
**Integration flow**:
1.  **Connection**: The user enters their Twilio Account SID, Auth Token, and Sender Phone Number into the OHC settings.
2.  **Configuration**: The user toggles which OHC events should trigger an SMS (e.g., "Appointment Reminder 24h before", "Order Shipped").
3.  **Execution**: When the defined event occurs, OHC formats a concise message and sends it via the Twilio API to the customer's registered phone number.
4.  **Logging**: The SMS delivery status is logged in the customer's activity feed within OHC.

## Implementation Prompt
**User-Facing Outcome**: The business owner can input their Twilio credentials to unlock SMS capabilities. They can then enable automated SMS reminders for bookings or order updates, ensuring their customers receive critical information directly on their phones.
**Acceptance Criteria**:
- UI to configure Twilio API credentials.
- Settings panel to toggle specific SMS notification triggers (e.g., booking reminders).
- Background worker/process to dispatch the SMS via Twilio API at the correct time.
- Basic formatting of SMS templates to ensure they are concise.
- Works in both Cloud and Standalone modes.

## Priority
P1

## Estimated Scope
Medium
