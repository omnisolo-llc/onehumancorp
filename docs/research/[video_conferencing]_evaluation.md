# [Video Conferencing] Microsoft Teams/Zoom Integration Evaluation

## Title
Auto-Generate Meeting Links for Appointments

## Problem Statement
Small business owners offering virtual consultations spend unnecessary time manually creating meeting links and emailing them to clients. This workflow is error-prone and unprofessional.

## Research Report
- **Strategy**: Native OAuth integration with Zoom or Microsoft Graph API.
- **Persona**: Tutors, online consultants, B2B services.
- **Advantages**: Highly professional. Parity with industry standards. Keeps the scheduling and delivery flow entirely automated.
- **Risks**: OAuth permissions can be granular and confusing to configure for the initial developer setup.
- **Pricing**: Free tiers exist; Teams included in Microsoft 365.
- **Compatibility**:
  - **Cloud**: OAuth.
  - **Standalone**: Server-to-Server OAuth or User OAuth.

## Design Doc
- **Trigger**: Customer books an online service.
- **Action**: OHC schedules a meeting via API and attaches the join URL.
- **User Interface**: User connects their Zoom/Microsoft account. During service creation, they set location to "Online Meeting".

## Implementation Prompt
Build an integration that dynamically creates meeting links for online service bookings. Users authenticate their account. Upon booking, OHC generates a unique link and attaches it to the appointment record and outgoing notifications.

## Priority
P2

## Estimated Scope
Medium
