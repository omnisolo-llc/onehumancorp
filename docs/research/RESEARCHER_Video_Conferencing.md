# Video Conferencing Brief

## Problem Statement
Manually creating, copying, and sharing Zoom links for every online consultation or class is repetitive and prone to error.

## Research Report
**Tool Evaluated:** Zoom API
**Findings:** Integrating Zoom allows for automatic meeting link generation when a booking is made. It provides a professional and familiar experience for customers.
**Pricing:** Zoom has a free tier; Pro is ~$15/month.
**Ease of Use:** Customers are generally very familiar with joining Zoom calls. The owner only needs to connect their account once.
**Risks:** Requires the user to navigate the Zoom OAuth flow. Free accounts have a 40-minute time limit, which might interrupt longer consultations.

## Design Doc
**Trigger:** A customer books a virtual appointment via the scheduling tool.
**Action:** A unique Zoom meeting is generated. The link is automatically included in the calendar invite and confirmation emails sent to both the owner and the customer.
**User Experience:** When setting up a service, the owner selects "Virtual Meeting (Zoom)". From then on, links are generated automatically without any manual intervention.

## Implementation Prompt
**Outcome:** Seamless integration with Zoom to automatically generate and share meeting links for virtual appointments.
**Acceptance Criteria:**
- Owner can connect their Zoom account to OHC.
- Booking a virtual service automatically generates a unique Zoom link.
- The link is correctly distributed to the participants.

## Priority
P2

## Estimated Scope
Small
