# Microsoft Teams Integration Issue Brief

## Title
Integrate Microsoft Teams for Automated Video Consultations

## Problem Statement
B2B small businesses and consultants who use the Microsoft ecosystem spend too much time manually creating Teams meeting links and emailing them to clients. They need meetings to be automatically generated and linked when a client books a slot.

## Research Report
- Microsoft Teams is deeply integrated into the corporate world. For B2B consultants using OHC, offering a Teams meeting link looks professional and aligns with their clients' tools.
- It requires using the Microsoft Graph API, which can be complex due to Azure AD enterprise permissions.
- Pricing: Included with most Microsoft 365 business plans.
- Competitors: Zoom (easier API, broader consumer adoption), Google Meet (better for G-Suite users).
- Integration: Graph API `onlineMeetings` endpoint.
- Cloud/Standalone: Cloud mode requires a registered multi-tenant Azure App. Standalone mode might be challenging due to Azure AD setup requirements for individual users.

## Design Doc
- Users connect their Microsoft 365 account via an OAuth flow in the "Integrations" tab.
- When an appointment is booked (e.g., via the Acuity integration or manually), OHC requests an `onlineMeeting` URL from the Graph API.
- The generated Teams link is automatically added to the calendar event and the confirmation email sent to the client.

## Implementation Prompt
Implement a Microsoft Teams video conferencing integration using the Microsoft Graph API. Implement the OAuth2 flow to authenticate users and obtain tokens for the `OnlineMeetings.ReadWrite` scope. Create a function that generates a new Teams meeting link for a given time slot and returns the join URL.

## Priority
P3

## Estimated Scope
Large
