# [SMS] Critical Notifications and Reminders

## Problem Statement
Many customers, particularly in certain demographics or regions with less reliable internet access, respond better to SMS than email. Small business owners currently have to manually text clients for appointment reminders or delivery updates, which is time-consuming and hard to track.

## Research Report
**Tools Evaluated:** Twilio, MessageBird

*   **Ease of Use:** For the business owner, configuring automated SMS is highly valuable. However, the initial setup with providers like Twilio requires purchasing a phone number and navigating complex regulatory compliance (like A2P 10DLC registration in the US). This is a high barrier for non-technical users.
*   **Pricing:** Generally pay-as-you-go per message (e.g., fraction of a cent to a few cents depending on the country), plus a small monthly fee for the dedicated phone number.
*   **Reputation:** Twilio is the gold standard for developer SMS APIs; MessageBird is a strong competitor, especially internationally.

## Design Doc
**Trigger:** An event occurs in OHC (e.g., Appointment booked, Order shipped) where SMS is enabled.
**Action:** OHC formats a brief text message and dispatches it via the SMS API.
**User Sees:** An "Automations" or "Notifications" settings page where they can toggle "Send SMS Reminders" on or off. They can customize a short template (e.g., "Hi {{name}}, your appointment is at {{time}}."). The platform should ideally abstract away the phone number purchasing complexity if possible (e.g., in cloud mode).

## Implementation Prompt
Implement an SMS notification system (e.g., integrating Twilio). Create a UI where the business owner can enable SMS for specific events (like appointment reminders). The UI should allow them to edit a short message template with variables. In standalone mode, provide a clear "Advanced Mode" toggle where the user can input their Twilio Account SID, Auth Token, and sending phone number.

## Priority
P1

## Estimated Scope
Medium

## Mode Compatibility
*   **Cloud:** Excellent. The cloud platform could theoretically manage a master account and handle the number provisioning and compliance on behalf of the user, charging them via OHC billing.
*   **Standalone:** Requires the user to bring their own API keys and handle their own number purchasing and compliance via the Twilio console, which introduces friction.
