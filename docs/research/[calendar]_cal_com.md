## [Calendar] Issue Brief: Cal.com Integration for Seamless Scheduling

**Title**: Scout 🔍: Integrate Cal.com for Unified Booking and Scheduling
**Problem Statement**:
Service-based businesses (e.g., consultants, tutors) struggle with the back-and-forth of scheduling appointments. They need a simple, professional way for clients to book time that automatically syncs with their availability.
**Research Report**:
- **Tool**: Cal.com
- **Evaluation**: An open-source alternative to Calendly. Highly customizable, supports multiple integrations, and is self-hostable.
- **Ease of Use**: Very user-friendly. Users can create booking pages easily.
- **Pricing**: Free core tier, reasonable pro features. Open-source version is free (requires hosting).
- **Cloud vs. Standalone**: Excellent fit. Can use the hosted SaaS for Cloud, and the self-hosted version for Standalone deployments.
**Design Doc**:
- User connects their existing calendar (Google, Outlook) to Cal.com via OHC.
- OHC generates a unique booking link for the user's services.
- When a client books, Cal.com handles the calendar invite and conflict resolution.
- OHC receives a webhook to log the booking in the CRM.
**Implementation Prompt**:
Integrate Cal.com into the OHC platform. Provide a UI for users to set their availability and generate booking links. Set up webhooks to capture booking events and sync them with the user's OHC CRM profile. Support embedding the booking widget on the user's OHC-generated storefront.
**Priority**: P2
**Estimated Scope**: Medium
