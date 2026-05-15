# Zoom API - Video Conferencing Integration

## Problem Statement
Automated generation of secure, unique video meeting links for online consultations, classes, or support calls is a tedious manual process for service providers.

## Research Report
Zoom provides a comprehensive API for managing meetings.
- **Ease of Use for SMBs**: High. Widespread user familiarity. Customers know how to join a Zoom call.
- **Pricing**: Requires a paid Zoom Pro account for the business owner to utilize the API effectively without 40-minute limits.
- **Reputation**: Ubiquitous and reliable.
- **Competitive Analysis**: The standard choice for video conferencing, despite the requirement for a paid account for advanced features.

## Design Doc
**Trigger**: An appointment is booked that requires a video call (via Cal.com integration).
**Actions**:
- OHC/Cal.com uses the Zoom API to create a unique meeting room.
- The join link is sent to both the business owner and the customer.
**User Experience**: A unique Zoom link is automatically attached to calendar invites for virtual appointments.

## Implementation Prompt
**User-facing Outcome**: A business owner offering virtual services automatically gets unique Zoom meeting links generated for every booked appointment.
**Acceptance Criteria**:
- User can connect their Zoom account via OAuth.
- Unique Zoom meeting links are automatically generated for new virtual bookings.
- Links are correctly formatted and shared with participants.

## Priority
P3 (Low)

## Estimated Scope
Small
