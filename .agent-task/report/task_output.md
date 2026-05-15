# [SMS & Notifications] Integrate Global SMS Notifications for Critical Alerts

## Title
Integrate Global SMS Notifications for Critical Alerts

## Problem Statement
Small business owners, especially those with low-English proficiency or those in regions where SMS is the primary communication channel (e.g., Fatima), struggle to receive timely and reliable alerts for critical business events. Email notifications are often missed, ignored, or sent to spam. This results in missed appointments, delayed responses to urgent customer queries, and a lack of real-time awareness about critical business operations. They need a simple, reliable way to receive SMS alerts directly on their phones.

## Research Report
I evaluated Twilio, MessageBird, and Vonage as potential SMS notification providers.

*   **Twilio:** The industry standard. Excellent global carrier coverage and reliability. Pricing is competitive but varies significantly by country. It offers robust opt-out compliance (STOP messages) built-in. However, setting it up can be slightly daunting for a completely non-technical user if they had to do it themselves (though OHC would handle the backend integration). Reputation is top-tier.
*   **MessageBird:** Strong international presence, sometimes slightly better pricing in specific regions compared to Twilio. Good reliability and API.
*   **Vonage (formerly Nexmo):** Another solid global player. Similar features and pricing to Twilio and MessageBird.

**Recommendation:** Twilio is the safest bet for global coverage, reliability, and robust opt-out compliance. It handles the complexities of global SMS delivery very well.

**Pricing Estimate:** Typically around $0.0079 to $0.05 per message depending on the destination country. This is very affordable for critical alerts.

**Ease of Use (for the user):** The business owner should only have to enter their phone number and verify it via a one-time code. No complex configuration needed on their end.

## Design Doc
This integration will connect OHC to an SMS provider (like Twilio).
1.  **User Onboarding:** A new section in the OHC Settings allows the user to opt-in to SMS notifications by providing a mobile number.
2.  **Verification:** A standard SMS verification flow (OTP) will ensure the number is valid and belongs to the user.
3.  **Triggers:** The integration will listen for specific, high-priority events within OHC (e.g., new urgent booking, failed payment alert).
4.  **Action:** When a trigger fires, OHC will construct a short, plain-text message and send it to the verified phone number via the SMS provider's API.
5.  **Opt-Out:** Standard "Reply STOP to cancel" functionality must be supported.
6.  **Environment Support:** This integration will work in both Cloud (OHC manages the API keys and billing) and Standalone modes (the user provides their own API keys for their SMS provider).

## Implementation Prompt
Implement a new setting that allows business owners to receive critical business alerts via SMS.
*   Add a simple UI in the settings where users can input their mobile phone number and select which types of critical alerts they want to receive via SMS.
*   Ensure the user receives a confirmation text to verify their number.
*   When a selected critical event occurs, the user should receive a brief, clear text message summarizing the event.
*   The system must gracefully handle failed message deliveries without crashing the main application.

## Priority
P1

## Estimated Scope
Medium
