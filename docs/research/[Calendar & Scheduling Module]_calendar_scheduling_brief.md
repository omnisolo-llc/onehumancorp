# Calendar & Scheduling Module

## Problem Statement
Manual appointment booking leads to double-booking and wasted time back-and-forth over email. Business owners need a simple link to share for automatic booking synced with their personal calendar.

## Research Report

**Market Context:**
Scheduling software may refer to:

Appointment scheduling software, for business appointments
Employee scheduling software
Job scheduler, for computer program execution

**Evaluated Tools:**

#### In-Depth Evaluation: Calendly
**Market Position**: The dominant player in automated scheduling. 'Send me your Calendly' has become a verb.
**Pricing**: Generous free tier, with essential paid features starting around $10/mo.
**Integration Approach**: Calendly's webhooks are robust. We need to ensure that when a meeting is booked via Calendly, it immediately reflects in OHC's internal calendar to prevent double-booking. Standalone mode requires an OHC cloud relay to catch the webhook and push it to the local app.
**Persona Impact**: Eliminates the 'when are you free?' email dance. Fatima can just send a link for cake tasting consultations.

#### In-Depth Evaluation: Acuity Scheduling
**Market Position**: Acquired by Squarespace, very popular with service-based businesses (salons, consultants).
**Pricing**: Starts around $16/mo.
**Integration Approach**: Offers deep customization. The integration would similarly rely on OAuth and webhooks. The challenge is syncing Acuity's specific service types with OHC's generic task/meeting models.

#### In-Depth Evaluation: Doodle
**Market Position**: Best for group scheduling (finding a time that works for 5 people), less for 1-on-1 bookings.
**Pricing**: Free tier exists, premium starts around $6/mo.
**Integration Approach**: Less relevant for OHC's primary booking flow unless the business frequently organizes group workshops.

## Design Doc
Implement a two-way sync service connecting to Google Calendar and Outlook APIs. Users connect their account via OAuth. OHC generates a unique booking page URL. When a client books, a calendar event is created, and if applicable, a Zoom/Meet link is auto-generated.

## Implementation Prompt
Build a customizable booking page interface where business owners can define their working hours and meeting types. Integrate this with a backend service that handles calendar availability checking and event creation.

## Priority
P1

## Estimated Scope
Medium
