# Scout: SMS & Notifications (Telnyx)

## Title
Cost-Effective Global SMS & Voice Notifications 📱 (Telnyx API Integration)

## Problem Statement
For business owners like Fatima (food cart operator), email is not the primary communication channel. She needs immediate, high-reliability SMS alerts when a new order arrives, and her customers need text notifications when their food is ready for pickup. Standard SMS providers can be expensive and complex to configure for international use. Telnyx offers a high-performance, cost-effective alternative with better global reach and simpler regulatory compliance.

## Research Report
- **Goal**: Evaluate Telnyx API as the primary SMS and Voice engine for OHC's Customer Success and Operations departments.
- **Features evaluated**:
  - **Programmable Messaging**: High-deliverability SMS and MMS.
  - **Messaging Profiles**: Easily manage 10DLC and Toll-Free registrations for OHC tenants.
  - **Voice API**: Potential for automated "Out for Delivery" or "Order Ready" phone calls.
  - **Global Reach**: Local numbers and reliable delivery in 100+ countries.
- **Benefits for OHC users (Non-technical)**:
  - Much lower cost per message compared to incumbents.
  - Simple "Opt-in" management that keeps them compliant with local laws (GDPR, CCPA).
  - High reliability ensures they never miss a "New Order" notification.
- **Integration Risks**:
  - Regulatory requirements for 10DLC (US) require a streamlined onboarding flow.
  - Handling delivery receipts and inbound messages requires a robust webhook listener.
- **Pricing**: Transparent, wholesale-style pricing (per message); significant savings for OHC as it scales.
- **Cloud vs Standalone**: Native support for Cloud mode. For Standalone, Telnyx can be called from the local backend, with incoming webhooks routed through the Hybrid MCP tunnel.

### Persona Pain Point Summary
| Persona | Pain Point | Solution via Telnyx Integration |
|---------|------------|---------------------------------|
| **Fatima (Food Cart)** | Doesn't check her phone's email app while cooking. | Receives a short, clear SMS: "New Order #123: 2x Halal Platters. Total $24." |
| **Leo (Tutor)** | Students forget their lesson times. | Automated SMS reminder sent 1 hour before the lesson begins. |

## Design Doc
- **Component**: `MessagingNotificationService`
- **Responsibilities**:
  - Manage Telnyx API keys and messaging profiles.
  - Handle 10DLC brand and campaign registration for OHC tenants.
  - Send transactional SMS triggered by OHC events (order placed, booking confirmed).
  - Process inbound SMS replies and route them to the OHC Unified Inbox.
- **User Experience**:
  - A "Notifications" toggle in the OHC app: "Get SMS alerts for new orders."
  - Field to enter their mobile number with a simple verification code.

## Implementation Prompt
"Integrate the Telnyx Messaging API in `src/server/integrations/telnyx/`. Implement a service to send transactional SMS messages and handle inbound message webhooks. Build an abstraction that allows other OHC departments (Operations, Sales) to trigger SMS notifications easily. Acceptance criteria: A merchant receives an SMS notification within 5 seconds of a new order being placed in the OHC system."

## Priority
P0

## Estimated Scope
Medium
