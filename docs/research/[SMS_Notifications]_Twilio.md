**Title**: Twilio Integration for Reliable SMS Notifications
**Problem Statement**: Email open rates are low, and for non-English speaking or less tech-savvy users, SMS is the only reliable way to confirm appointments, send payment links, or provide updates.
**Research Report**: Twilio is the global leader in SMS infrastructure. While its raw API is developer-focused, OHC can abstract this away. Twilio has excellent global carrier coverage and handles opt-out compliance well. Pricing is very low per message.
**Design Doc**:
- **Trigger**: An appointment is booked, or an invoice is due.
- **Action**: OHC sends a templated SMS via Twilio to the customer.
- **User Experience**: The business owner buys a phone number through OHC (powered by Twilio) or connects an existing Twilio account. They toggle "Send SMS Reminders" on, and OHC handles the rest transparently.
**Implementation Prompt**: Build an SMS notification toggle for appointment reminders and invoice links. Integrate Twilio in the backend to deliver these messages. Allow the business owner to customize the text message template using simple placeholders like `[Customer Name]` and `[Time]`.
**Priority**: P0
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.
