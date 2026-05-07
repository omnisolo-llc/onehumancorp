# Calendly Meeting Booking Integration

## Problem Statement
Small business owners (like consultants, personal trainers, or tutors) spend too much time going back and forth with clients via email or text to find a time to meet. They need a simple way to let clients see when they are free and book a time, without double-booking over their existing appointments. They shouldn't need to manually create calendar invites or generate Zoom links.

## Research Report
Calendly is a widely recognized scheduling tool that simplifies booking.
- **Ease of Use**: Very simple for both the business owner to set up and the client to use. It offers a clean, straightforward interface.
- **Capabilities**: Syncs with Google Calendar, Outlook, and others. Automatically generates meeting links (e.g., Zoom, Google Meet) upon booking. Handles time zones automatically.
- **Competitors**: Cal.com, Acuity Scheduling. Calendly is the most recognized brand with a very polished user experience tailored to individuals and small businesses.
- **Reputation**: Excellent reputation for reliability and ease of use.
- **Pricing**: Free tier includes one active event type (great for a single business owner). Paid tiers start at $10/month per seat, offering multiple event types and advanced integrations.
- **Deployment**: Exposes a robust API and webhooks. Suitable for Cloud (webhooks) and Standalone (can be integrated via standard REST APIs, though webhook reception requires internet accessibility).

## Design Doc
The integration will embed Calendly's booking interface directly into the business owner's OHC storefront or customer portal. OHC will store the user's Calendly personal link. When a booking is made, a webhook from Calendly will notify OHC, allowing OHC to display upcoming appointments in the business owner's daily summary. The system will rely on Calendly's native calendar syncing to handle conflict resolution.

## Implementation Prompt
Add an "Appointments" section in the OHC settings where users can paste their Calendly personal link. Once provided, display a booking widget on their public business page. In the OHC dashboard, show a list of "Upcoming Appointments" fetched from the Calendly integration. The user should not have to configure API keys manually; use OAuth if possible, or a simple link-pasting method.

## Priority
P1

## Estimated Scope
Small
