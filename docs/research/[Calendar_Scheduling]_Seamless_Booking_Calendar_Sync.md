## [Calendar & Scheduling] Seamless Booking & Calendar Sync
**Title**: Integrate Cal.com for Zero-Friction Booking

**Problem Statement**: Service-based business owners (consultants, tutors, handymen) waste hours going back and forth over email or text to find a time to meet with clients. They need a simple link to send clients that automatically syncs with their personal calendar.

**Research Report**:
- **Persona Context**: Solo entrepreneurs and service businesses whose core product is their time.
- **Solution Evaluated**: Calendly vs. Cal.com. Both offer excellent user experiences. Cal.com is open-source, highly embeddable, and developer-friendly.
- **Ease of Use**: Very high. Owners just share a link.
- **Advantages**: Cal.com supports round-robin scheduling, Zoom/Meet link generation, and handles timezones effortlessly. Open-source nature aligns with OHC's hybrid model.
- **Risks**: Syncing personal calendars (Google/Outlook) requires sensitive OAuth scopes, which might concern some privacy-focused standalone users.
- **Pricing Estimate**: Cal.com is free for individuals, $15/user/month for teams.
- **Cloud/Standalone Support**: Works in Cloud (via Cal.com API) and Standalone (self-hosting or local API integrations).

**Design Doc**:
- **Triggers**: A customer books a slot via the generated booking link.
- **Actions**: OHC receives a notification, blocks out the time on the owner's synced calendar, and generates a notification.
- **User Interface**: A "Scheduling" tab where the owner clicks "Connect Google/Outlook Calendar" and sets their available hours. OHC provides a shareable booking link. Upcoming appointments appear on the OHC dashboard.

**Implementation Prompt**:
Create a scheduling feature that allows users to connect their Google or Outlook calendar. Generate a public booking page for their clients. When a client books, the appointment should appear on the OHC dashboard and block the corresponding time on the user's connected calendar.

**Priority**: P1
**Estimated Scope**: Medium
