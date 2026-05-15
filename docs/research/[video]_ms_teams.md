## 7. Video Conferencing: Microsoft Teams

**Title:** Integrate MS Teams for B2B Video Consultations
**Problem Statement:** Not all clients use Zoom. Some business owners, particularly those doing B2B consulting, need to generate MS Teams meeting links automatically when a calendar slot is booked.
**Research Report:**
- **Tool evaluated:** Microsoft Teams (via Microsoft Graph API)
- **What problem it solves for which persona:** Provides seamless enterprise-grade video conferencing for consultants and professional service providers.
- **Ease of Use:** Familiar to enterprise clients, though the Graph API is notoriously complex to work with.
- **Pricing:** Included in Microsoft 365 Business Basic (around $6/user/month).
- **Reputation:** The standard for B2B.
- **Advantages & Risks:**
  - *Advantages:* High trust, bundles with email and calendar.
  - *Risks:* Integration via Microsoft Graph API is highly complex and requires rigid permissions.
- **Cloud/Standalone Mode:** Cloud mode requires verified multi-tenant Azure AD app. Standalone requires complex individual tenant setups.
**Design Doc:**
- **Trigger:** A consultation is booked.
- **Action:** OHC requests an 'OnlineMeeting' resource via Graph API and attaches the URL to the calendar invite.
- **User View:** The booked appointment details in OHC display a 'Join Teams Meeting' button.
**Implementation Prompt:**
Integrate with a video conferencing API to generate dynamic meeting links upon booking confirmation. Display this link in both the business owner's dashboard and the customer's confirmation email.
**Priority:** P3
**Estimated Scope:** Large
