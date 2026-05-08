# Video Conferencing Integration

## Title
Integrate Zoom for Video Conferencing

## Problem Statement
Service providers offering online consultations struggle with manually creating and sending video links for every booking.

## Research Report
**Tool Evaluated:** Zoom
**Pricing:** Free tier; $15/mo premium
**Cloud/Standalone Support:** Cloud: Yes. Standalone: Yes (API driven).

**Findings:**
Zoom is universally understood by consumers. The API allows automatic meeting creation. It pairs well with scheduling tools. Free tier has a 40-minute limit, Pro is $15/mo.

## Design Doc
When a virtual service is booked, OHC calls the Zoom API to generate a unique meeting link. This link is automatically included in the confirmation email/SMS sent to the customer and displayed in the owner's dashboard.

## Implementation Prompt
Integrate Zoom to automatically generate meeting links for virtual appointments. Ensure the join link is visible in the booking details and included in customer notifications.

## Priority
P2

## Estimated Scope
Medium
