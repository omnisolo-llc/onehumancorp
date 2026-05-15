# [video] Microsoft Teams Integration

## Problem Statement
Small business owners offering online consultations, tutoring, or remote services need an efficient way to generate and share video meeting links. Manually creating a meeting in Teams, copying the link, and emailing it to the client is tedious. Integrating Microsoft Teams into OHC allows for automatic generation of meeting links when appointments are booked, providing a seamless experience for both the owner and the client.

## Research Report
### Overview
Microsoft Teams is a widely used unified communication and collaboration platform. For businesses already entrenched in the Microsoft ecosystem (Office 365), it is the default choice for video conferencing.

### Ease of Use
The integration relies on Microsoft Graph API. The business owner authorizes OHC via OAuth. Once connected, OHC can automatically generate Teams meeting links for any scheduled event and embed those links in confirmation emails or calendar invites sent to the client.

### Reputation
Teams is an enterprise-grade platform known for its stability, security, and deep integration with other Microsoft products. While sometimes perceived as heavy for small businesses, it is unavoidable for those in B2B service sectors.

### Pricing
Included with most Microsoft 365 business subscriptions. The OHC API integration is free.

### Environment
Works in Cloud.

### AI Integration
Low potential for direct AI integration in link generation, but AI could be used post-meeting (if transcriptions are available via Microsoft Graph) to summarize the consultation and generate follow-up tasks in OHC.

## Design Doc
1.  **Connection:** User navigates to "Integrations" -> "Video Conferencing" -> "Connect Microsoft Teams" and completes the OAuth flow.
2.  **Meeting Generation:** When a new appointment is created in OHC (either manually or via a scheduling tool like Acuity), OHC calls the Graph API to create an online meeting.
3.  **Link Distribution:** The generated `joinUrl` is saved to the appointment record in OHC and automatically included in automated confirmation emails/SMS sent to the client.
4.  **Launch:** The business owner can click a "Join Meeting" button directly from their OHC calendar or dashboard to launch Teams.

## Implementation Prompt
Implement an integration with Microsoft Teams to auto-generate video meeting links. Provide an OAuth connection flow using the Microsoft Graph API. Update the OHC Calendar/Appointment module to include an option: "Make this an online meeting (Teams)". When selected, create the meeting via the API, store the join URL, and ensure the URL is exposed in the variables available for automated customer notification templates.

## Priority
P2 (Medium) - Important for B2B service providers, though Zoom or Google Meet might have higher adoption among solo entrepreneurs.

## Estimated Scope
Medium
