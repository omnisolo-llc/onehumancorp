**Title**: SMS & Notifications Integration: Twilio

## Problem Statement
Many small business customers (especially in service industries or specific demographics) do not reliably check email. Businesses need a way to send critical, timely notifications—like appointment reminders, order updates, or urgent alerts—via SMS text messages directly to customers' phones to reduce no-shows and improve customer satisfaction.

## Research Report
**Tool Evaluated:** Twilio
**Category:** SMS & Notifications
**Overview:** Twilio is a cloud communications platform that provides programmable APIs for SMS, voice, and video.

**Key Features for Small Businesses:**
*   **Global SMS:** Send texts to virtually any country.
*   **Phone Numbers:** Purchase local phone numbers to send messages from.
*   **Reliability:** Industry-leading deliverability and uptime.
*   **Programmable:** Can be integrated into almost any workflow (reminders, 2FA, marketing).

**Environment Compatibility:**
*   **Cloud Mode:** Fully supported via REST API.
*   **Standalone Mode:** Fully supported via REST API.

**Pros:**
*   Extremely reliable and scalable.
*   Pay-as-you-go pricing (fractions of a cent per message).
*   Developer-friendly with excellent SDKs.

**Cons:**
*   Twilio is a developer tool; it has no native end-user UI. OHC must build the entire user interface for composing and managing messages.
*   A2P 10DLC compliance in the US can be complicated for small business owners to register for.

## Design Doc

The integration utilizes Twilio as a headless infrastructure provider, with OHC providing the entire user interface for sending SMS notifications.

```mermaid
graph TD
    System[OHC Automated Workflow] -->|Trigger (e.g., Reminder)| OHC_API[OHC Rust Server]
    Owner[Small Business Owner] -->|Manual SMS Send| OHC_Dashboard[OHC Slint UI]
    OHC_Dashboard -->|SMS Request| OHC_API

    OHC_API -->|Send SMS API Call| Twilio_API[Twilio API]
    Twilio_API -->|Delivers Text| Customer_Phone[Customer Mobile Phone]
```

### High-Level UX Flow:
1.  **Integration Hub:** The business owner enters their Twilio Account SID, Auth Token, and Twilio Phone Number in the OHC integrations tab.
2.  **Configuration:** The user toggles on "Send SMS Reminders for Appointments" in their settings.
3.  **Operation:** 24 hours before an appointment, OHC's background job processor formats a message and sends it via Twilio.
4.  **Display:** The customer's CRM profile in OHC shows a log of SMS messages sent to them.

## Implementation Prompt
**Objective:** Integrate Twilio to enable automated and manual SMS notifications from the OHC platform.
**Acceptance Criteria:**
- Create a configuration UI in Slint for Twilio credentials.
- Implement a backend SMS service wrapper around the Twilio REST API.
- Add UI toggles to enable SMS notifications for specific events (e.g., new order, appointment reminder).
- Ensure the user interface passes the "Grandmother Test" (e.g., "Text Message Alerts").

## Priority
P1

## Estimated Scope
Medium
