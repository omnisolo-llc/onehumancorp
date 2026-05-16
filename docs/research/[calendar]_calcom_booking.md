# Integrate Cal.com for Zero-Config Booking & Calendar Sync

**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Service providers like Carlos the Handyman lose customers due to back-and-forth scheduling via text. They need a public booking link that syncs automatically with their personal Google or Outlook Calendar.

**Research Report**:
- Cal.com is an open-source scheduling infrastructure that handles timezone math, calendar conflict resolution, and booking pages natively.
- **Ease of Use**: Highly intuitive for non-technical users to set their availability.
- **Pricing**: Free tier available for individuals.
- **Reputation**: Strongly respected open-source alternative to Calendly.
- **Cloud vs Standalone**: Perfectly compatible with both Cloud (SaaS) and Standalone OHC modes due to its open-source and embeddable nature.
- **Key Advantages**: Eliminates scheduling friction, free tier suitable for SMBs.
- **Key Risks**: Requires reliable synchronization with third-party calendars.

**Design Doc**:
- Users connect their calendar via a one-click OAuth button in the "Operations" tab.
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- A public booking widget is displayed on their OHC storefront. When a customer books a slot, Cal.com manages the calendar event and conflict resolution transparently.

**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.

**Priority**: P0
**Estimated Scope**: Medium
