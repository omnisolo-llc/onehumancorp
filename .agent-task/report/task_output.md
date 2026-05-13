# Scout Research Report: Strategic SMB Tool Integrations

## Executive Overview
This comprehensive research report outlines the strategic integration roadmap for One Human Corp (OHC). The objective is to evaluate and prioritize tool integrations that directly alleviate the operational bottlenecks faced by non-technical small business owners, operating across both Cloud and Standalone environments.

The focus is strictly on tools that provide immediate, tangible value to the end-user (e.g., saving time, recovering lost revenue, professionalizing their appearance), deliberately excluding internal infrastructure tooling.

## Investigated Categories & Strategic Findings

1.  **Unified Social Media Integration (P0 - Critical)**
    *   **Finding**: Fragmentation of communication across Instagram, Facebook, and WhatsApp is a leading cause of lost revenue for visual and local service businesses.
    *   **Action**: Prioritize a zero-friction Meta OAuth integration to feed a unified inbox. This directly empowers the OHC agent to act as a front-line SDR.

2.  **Smart Calendar & Scheduling Sync (P0 - Critical)**
    *   **Finding**: Manual scheduling is a massive time sink. Users currently pay $10-$15/mo for tools like Calendly.
    *   **Action**: Develop a native, bidirectional Google/Outlook calendar sync engine and public booking page. This is a massive retention hook.

3.  **Global Payment Links (P0 - Critical)**
    *   **Finding**: Friction in payment collection chokes cash flow.
    *   **Action**: Integrate Stripe Connect as an MVP to allow users to generate secure "Pay Now" links directly from chat interfaces, automating the transition from "Invoice Sent" to "Paid".

4.  **Zero-Friction Email Marketing (P1 - High)**
    *   **Finding**: Standard marketing tools (Mailchimp) are overly complex for simple announcements.
    *   **Action**: Leverage existing CRM data to build a highly simplified, AI-assisted text editor backed by a robust transactional sender (SendGrid/Resend).

5.  **Reliable SMS Notifications (P1 - High)**
    *   **Finding**: Email reminders are ignored, leading to costly no-shows for service businesses.
    *   **Action**: Integrate Twilio for high-reliability SMS. To bypass complex US A2P 10DLC regulations initially, implement a "Bring Your Own Key" (BYOK) model for power users.

6.  **Streamlined Shipping Logistics (P2 - Medium)**
    *   **Finding**: E-commerce sellers waste hours manually buying labels.
    *   **Action**: Integrate an aggregator like EasyPost to enable one-click rate shopping and label PDF generation directly from the order view.

7.  **Automated Video Conferencing (P2 - Medium)**
    *   **Finding**: Manual link generation for remote consultations looks unprofessional.
    *   **Action**: Piggyback on the Calendar API integration to auto-generate Google Meet links for confirmed bookings.

## Architecture & Mode Considerations
All P0 integrations rely heavily on standard OAuth 2.0 flows and webhooks. While this is trivial in the Cloud (multi-tenant) mode, the Standalone mode presents significant challenges regarding webhook delivery to local machines behind NATs.
**Recommendation**: Future architectural planning must include a lightweight OHC Cloud Relay service to securely route essential webhooks (like payment confirmations and incoming DMs) down to Standalone instances via persistent WebSockets.

## Next Steps
- The 7 detailed technical issue briefs (`docs/research/*.md`) have been generated and are staged for review by the Engineering implementation teams.
- **Implementers Directive**: Begin parallel execution on the P0 tasks: Social Inbox, Calendar Sync, and Payment Links.
