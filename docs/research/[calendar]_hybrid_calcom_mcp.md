# Cal.com Integration
## Problem Statement
Small business owners, such as Leo the music tutor or Carlos the handyman, need an easy way to schedule appointments with their customers without back-and-forth emails.

## Research Report
**Tool**: Cal.com
**Ease of use**: High. Provides an intuitive interface for both the business owner and the customer. Includes pre-built templates and a user-friendly booking experience.
**Pricing**: Includes a generous free tier for individuals (unlimited bookings, 1 event type). Team features start at $12/user/month.
**Reputation**: Highly regarded, used by major companies (Vercel, GitHub) and built as an open-source alternative to Calendly. Strong developer ecosystem.

## Design Doc
**Cloud Mode**: Uses the Cal.com API v2 to sync calendars, create event types, and generate booking links. The integration can use webhooks to notify the OHC platform when an appointment is booked or canceled.
**Standalone Mode**: Can integrate with the local Cal.com self-hosted instance or utilize local calendar files (e.g., CalDAV) to provide offline scheduling capabilities that sync when online.
**Triggers**: Customer requests a booking, business owner sets up availability.
**User Experience**: Business owner configures their calendar and event types in the OHC dashboard. Customers see a seamless booking interface overlaid or embedded on the business website or social media.

## Implementation Prompt
Integrate Cal.com into the OHC platform to provide seamless appointment scheduling.
**Acceptance Criteria**:
1. Business owners can link their existing calendars (Google, Outlook, Apple) to OHC via Cal.com.
2. Business owners can create and manage event types (e.g., 30-min consultation, 1-hour service) directly from the OHC dashboard.
3. Customers can book appointments using a Cal.com booking interface embedded within the business's OHC-generated website.
4. Appointment bookings, modifications, and cancellations automatically update the business owner's connected calendar and OHC dashboard.

## Priority
P1

## Estimated Scope
Medium
