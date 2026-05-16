# [Calendar & Scheduling] Cal.com Integration Evaluation

## Title
Automated Scheduling via Cal.com

## Problem Statement
Business owners spend too much time going back and forth via email/WhatsApp to schedule meetings or services. They need a simple booking page that syncs with their personal calendar to prevent double-booking.

## Research Report
- **Strategy**: Leverage Cal.com's robust scheduling engine.
- **Persona**: Consultants, tutors, professional services, salons.
- **Advantages**: Open-source alternative to Calendly, great developer experience, very clean UI for both merchant and customer.
- **Risks**: Learning curve for initial setup; self-hosting involves maintenance overhead.
- **Pricing**: Free for individuals, reasonable team plans. Can be self-hosted.
- **Compatibility**:
  - **Cloud**: Managed integration via Cal.com API.
  - **Standalone**: Connect to a self-hosted Cal.com instance or public API.

## Design Doc
- **Trigger**: Business owner shares availability.
- **Action**: OHC provisions a Cal.com booking link or embeds the widget on the business's webpage.
- **User Interface**: Business owner connects Google/Outlook Calendar, sets working hours in OHC, and OHC generates a shareable booking link.

## Implementation Prompt
Create a "Scheduling" tab where the business owner can define weekly availability. Provide a "Share Booking Link" button that copies a unique URL. Customers visiting this URL see available slots and book a session, which appears on the owner's dashboard.

## Priority
P1

## Estimated Scope
Large
