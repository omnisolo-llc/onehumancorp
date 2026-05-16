# Title: Automated Appointment Scheduling Integration

## Problem Statement
Small business owners (like tutors, consultants, or salon owners) waste hours playing "email ping-pong" trying to find a mutually agreeable time for appointments. Double bookings are common, and managing timezone differences for remote consultations is frustrating and error-prone. They need a hands-off way for clients to book available slots.

## Research Report
**Tool Analyzed**: Cal.com
**Ease of Use**: Excellent. Provides a clean, modern interface for both the business owner and the person booking.
**Reputation**: Open-source, highly respected, and gaining rapid traction against competitors like Calendly. Known for strong developer experience and flexibility.
**Pricing**: Free for individuals (perfect for single-owner businesses). Team plans start at $12/user/month.
**Environment**: Works seamlessly in Cloud mode. Because it's open-source, it's uniquely suited for Standalone mode, as it could potentially be self-hosted alongside OHC for ultimate privacy.
**AI Integration**: Could integrate with OHC's AI to suggest meeting times in chat or auto-schedule follow-ups based on project milestones.

## Design Doc
**Integration Trigger**: User connects their primary calendar (Google/Outlook) and configures their "working hours" and "appointment types" (e.g., 30-min consultation) in OHC.
**Actions Taken**:
- A unique booking link is generated for the business owner.
- When a client uses the link to book, Cal.com handles availability checks and timezone math.
- Upon booking, a calendar event is automatically created in the owner's calendar.
- OHC detects the new booking and logs it in the customer's CRM profile.
**User View**: The business owner sees an "Appointments" dashboard in OHC listing upcoming bookings. They can easily copy their booking link to share with clients or embed it on their website.

## Implementation Prompt
Integrate Cal.com to provide automated scheduling. The business owner should be able to authenticate their calendar and set basic availability directly within OHC. Generate a shareable booking link. When a client books an appointment, it should automatically appear on the owner's connected calendar, and the OHC dashboard should display a read-only list of upcoming appointments for the week.

## Priority
P0

## Estimated Scope
Medium
