**Title**: Integrate Zoom for OHC

## Problem Statement
When a client books an online consultation, I have to manually create a Zoom meeting and email them the link. It's tedious and error-prone.

## Research Report
**Tool Evaluated:** Zoom

**Findings:** Zoom's API allows automatic meeting creation. Authentication requires OAuth 2.0 (Server-to-Server for internal apps, or standard OAuth for user-facing integrations). It's globally recognized and trusted by consumers.

**Pricing:** API access requires a Pro account or higher (starts at $15.99/mo).

**Cloud vs Standalone Mode:** Cloud utilizes standard OAuth. Standalone may require Server-to-Server OAuth configuration by the user.

## Design Doc
If a booked service is marked as 'Online', OHC calls the Zoom API to generate a unique meeting link using the owner's connected Zoom account. The link is automatically added to the calendar invite and confirmation email.

## Implementation Prompt
Create an integration where business owners can connect their Zoom account. When an online service is booked, automatically generate a unique Zoom meeting link and share it with both the owner and the customer.

## Priority
P2

## Estimated Scope
Medium
