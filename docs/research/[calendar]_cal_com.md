# Scout: Tool Integration Research Q2

## 2. Calendar & Scheduling

**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync

**Problem Statement**: Small business owners like Leo the Music Tutor and Carlos the Handyman lose potential customers due to back-and-forth scheduling via text and email. They need a public booking link that seamlessly syncs with their personal Google or Outlook calendars without requiring complex technical setup.

**Research Report**:
- **Tool**: Cal.com
- **Target Persona**: Leo (Music Tutor), Carlos (Handyman)
- **Advantages**: Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and custom booking pages out-of-the-box. It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes.
- **Risks**: Ensuring the self-hosted Standalone mode remains lightweight enough not to overwhelm local resources.
- **Pricing**: Free tier available for individuals; highly cost-effective for our free tier users.
- **Compatibility**: Cloud and Standalone (can run self-hosted or via Cal.com's hosted API).

**Design Doc**:
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours and availability preferences.
- Users connect their Google/Outlook calendar via a one-click OAuth button located in the "Operations" dashboard.
- A public booking widget is embedded on the user's storefront.
- When a customer selects a slot on the OHC public page, Cal.com transparently manages the calendar event creation and conflict resolution.
- The business owner receives a simple notification of the new booking in their unified inbox.

**Implementation Prompt**: Embed Cal.com's scheduling infrastructure to allow users to sync their personal calendars. Provide a public-facing booking widget on their storefront that prevents double-booking and automatically updates their synced calendar upon successful booking.

**Priority**: P0

**Estimated Scope**: Medium
