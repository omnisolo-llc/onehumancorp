# Twilio SMS Integration

## Problem Statement
Many small business owners, especially those with non-technical, older, or low-English-proficiency customer bases (e.g., local mechanics, nail salons, community centers), find that email notifications go unread. When an appointment is booked, changed, or an urgent update is needed, email is too slow. They need a way to send instant, reliable text messages directly to their customers' phones to ensure crucial information is seen immediately.

## Research Report
Twilio is the industry leader for programmatic SMS and voice communications globally.
- **Benefits for Users:** Near-instant delivery of critical notifications (e.g., appointment reminders, order readiness). High open rates compared to email.
- **Ease of Use:** From the owner's perspective, this is completely automated. They simply enable "SMS Notifications" in OHC, and the system handles the rest.
- **Reputation:** Twilio is enterprise-grade, offering massive scalability and carrier compliance.
- **Pricing:** Pay-as-you-go pricing. In the US, it is approximately $0.0079 per SMS. International rates vary but are generally affordable for critical alerts.
- **Environment Compatibility:** Works perfectly in both Cloud and Standalone modes. In Standalone, the user can plug in their own Twilio API keys, or OHC can route through a centralized cloud relay if billing is managed centrally.

## Design Doc
```mermaid
graph TD
    Trigger[OHC System Event / Agent Action] -->|Dispatches Notification| OHC_Backend[OHC Backend]
    OHC_Backend -->|Formats Message| SMS_Service[SMS Service Module]
    SMS_Service -->|API Call via Twilio SDK| Twilio[Twilio API]
    Twilio -->|Delivers SMS| CustomerMobile[Customer's Mobile Phone]
    Twilio -->|Delivery Receipt| OHC_Backend
    OHC_Backend -->|Logs Delivery| DB[(SIPDB / Postgres)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class OHC_Backend,SMS_Service,DB premium;
```

When an event occurs in OHC (like a completed booking or an agent identifying an urgent need), the backend formats a concise text message. It securely calls the Twilio API, which delivers the message to the end-user. The delivery status is asynchronously tracked in the OHC database.

## Implementation Prompt
Integrate Twilio to enable automated outbound SMS notifications.
- **User Outcome:** Business owners can toggle "Send SMS Reminders" in their workflow settings. Customers receive professional text messages for important events (like an upcoming appointment), drastically reducing no-show rates.
- **Acceptance Criteria:**
  - Secure integration with the Twilio API for outbound SMS.
  - Setup UI allowing users to input their Twilio credentials (for Standalone mode).
  - Webhook listener to track delivery statuses (Delivered, Failed, etc.).
  - Compliance handling (e.g., appending "Reply STOP to opt out" to messages).

## Priority
P1

## Estimated Scope
Small
