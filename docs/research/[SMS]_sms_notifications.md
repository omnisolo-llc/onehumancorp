# [SMS] OHC Tool Integration Research Brief: SMS & Notifications

## Title
Reliable Global SMS Notifications for Customers

## Problem Statement
Many customers, particularly in regions with lower email penetration or those with lower tech proficiency (like the persona Fatima), rely heavily on SMS for critical updates (e.g., appointment reminders, order confirmations, shipping updates). Small business owners need a reliable way to send these notifications automatically from OHC without manually texting from their personal phones.

## Research Report
The SMS market is commoditized but highly regulated (especially in the US).

**Evaluated Tools:**

1. **Twilio (twilio.com)**
    *   **Focus:** The industry giant for communications APIs.
    *   **Pros:** Unmatched global reach, reliability, and feature set.
    *   **Cons:** Can be complex to set up due to regulatory compliance. Pricing can add up quickly.
    *   **Pricing:** Pay-as-you-go (~$0.0079 per message in the US).
    *   **Modes:** Cloud and Standalone.

2. **MessageBird (Bird)**
    *   **Focus:** Omnichannel communications.
    *   **Pros:** Great global coverage, often more cost-effective outside the US compared to Twilio.
    *   **Cons:** Less mindshare than Twilio, but technically very capable.
    *   **Pricing:** Pay-as-you-go.
    *   **Modes:** Cloud and Standalone.

3. **Plivo**
    *   **Focus:** Cloud communications platform.
    *   **Pros:** Often more aggressive pricing than Twilio.
    *   **Cons:** Smaller ecosystem.
    *   **Pricing:** Pay-as-you-go.
    *   **Modes:** Cloud and Standalone.

**Recommendation:**
**Twilio** remains the safest and most robust choice for a platform like OHC, despite the regulatory overhead. Their extensive documentation and reliability make them the standard. However, **MessageBird** is a very strong alternative if international pricing becomes a primary concern. We will recommend an abstracted SMS provider architecture to allow switching between them.

## Design Doc
**Integration Approach: Abstracted SMS Provider**

1.  **Configuration:**
    *   Business owner configures their SMS provider in OHC settings.

2.  **Notification Triggers:**
    *   OHC defines standard notification events (e.g., Order Confirmed, Appointment Reminder).
    *   Business owner can toggle which events send an SMS.

3.  **Sending (Action):**
    *   When an event occurs, OHC uses the configured SMS provider to dispatch the message.
    *   The message content uses simple templating (e.g., "Hi {{name}}, your order is confirmed.").

## Implementation Prompt
**Objective:** Implement a flexible SMS notification system.

**Acceptance Criteria:**
1.  Establish a standardized mechanism for sending SMS messages.
2.  Implement an initial integration with a leading SMS provider (e.g., Twilio).
3.  Add a configuration model to store provider credentials per tenant.
4.  Implement an event listener that triggers SMS messages for key events (e.g., Order Confirmation) based on tenant configuration.

## Priority
P1

## Estimated Scope
Small
