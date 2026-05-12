# [Calendar & Scheduling] Calendly Integration

**Problem Statement**: Service-based businesses (consultants, salons, tutors) spend too much time going back and forth with clients to find a suitable meeting time. They need a way to share a booking link that automatically syncs with their availability.

**Research Report**:
- **Target Persona**: Consultants, coaches, salons, any appointment-based business.
- **Ease of Use**: Calendly is widely known and very user-friendly. Connecting it via OAuth is straightforward.
- **Pricing**: Calendly has a free tier, but team or advanced features require paid plans (~$10-$15/mo).
- **Reputation**: Industry standard, highly reliable.
- **Cloud/Standalone**: Works in both. Cloud can handle webhooks for new bookings. Standalone can poll or use webhooks if exposed.

**Design Doc**:
- **Trigger**: User connects Calendly account. Client booked a meeting via Calendly link.
- **Action**: OHC creates a new contact or updates an existing one when a booking is made. A notification is added to the dashboard.
- **User View**: Business owner sees upcoming Calendly appointments on their OHC dashboard and can view client details linked to the booking.

**Implementation Prompt**: Add a Calendly integration where users can connect their account. Display upcoming Calendly events on the OHC dashboard and automatically add/update customer records based on new bookings.

**Priority**: P2
**Estimated Scope**: Medium
