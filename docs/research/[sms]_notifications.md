# SMS & Notifications Integration

## Title
SMS Appointment Reminders & Notifications (Twilio)

## Problem Statement
Many small businesses serve customer bases with low English proficiency or those who simply ignore emails. "No-shows" for appointments cost them money. They need a reliable way to send critical alerts, like appointment reminders or order ready notifications, via SMS directly to their customers' phones.

## Research Report
*   **Target Tools:** Twilio SMS API.
*   **Pros:** Industry standard, highly reliable, global reach. Excellent developer documentation.
*   **Cons:** Strict A2P 10DLC compliance rules in the US require business registration to avoid carrier filtering. Can be complex for a micro-business to navigate regulatory approvals.
*   **Ease of Use for Non-Technical Users:** High usage simplicity, but the *setup* (A2P registration) is notoriously difficult for non-technical users. We must abstract this entirely.
*   **Pricing:** Around $0.0079 per message in the US, but international rates vary wildly. Phone number rental is ~$1/month.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Ideal. We can handle compliance at the platform level or guide users through a unified flow.
    *   *Standalone:* The user would need to provide their own Twilio credentials, forcing them to deal with A2P 10DLC compliance themselves, which is a terrible UX.

## Design Doc
1.  **Phone Number Provisioning:** In Settings, the user requests an SMS number. (Behind the scenes, OHC provisions a Twilio number and handles compliance).
2.  **Automated Triggers:** The user toggles "Send SMS Reminders 24h before appointment".
3.  **Manual Broadcast:** In the Customers list, the user can select a customer and click "Send SMS" to send an ad-hoc message (e.g., "Your car is ready for pickup").

## Implementation Prompt
Implement a "Text Message Alerts" feature. Allow the business owner to toggle SMS reminders for upcoming appointments. When toggled, the system should automatically send an SMS to the customer 24 hours before their scheduled time. Also, provide a simple text box on a customer's profile to send them a direct, ad-hoc SMS message. Ensure you include standard opt-out language ("Reply STOP to cancel") in automated messages to adhere to carrier guidelines.

## Priority
P1 (high)

## Estimated Scope
Medium
