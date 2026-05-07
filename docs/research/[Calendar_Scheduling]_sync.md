# Title: Integrate Calendar Sync and Automated Scheduling

## Problem Statement
Small business owners, especially those offering services (like coaching, consulting, or tutoring), waste hours every week going back and forth over email or text to find a time to meet. Double bookings occur frequently because their personal calendar isn't synced with their business availability. They need an automated way for customers to see their availability, book a slot, and have a meeting link generated instantly without any manual intervention.

## Research Report
We evaluated scheduling and calendar integration APIs suitable for OHC:
- **Nylas:** Provides a robust, unified API to sync with Google Calendar, Outlook, and iCloud. Highly reliable for bidirectional sync and conflict resolution. Also handles timezone complexities automatically. Pricing is per connected account. Excellent documentation, but overkill if we only need simple availability.
- **Cronofy:** Similar to Nylas, specialized in calendar integration. Handles timezone resolution flawlessly. Very reliable webhook notifications for calendar changes.
- **Calendly API:** Instead of building custom sync logic, we could integrate with Calendly. This provides a familiar interface for users, but it means pushing users to a third-party service rather than keeping them inside the OHC ecosystem.
- **Direct Google/Microsoft Graph APIs:** Free to use, but requires managing separate OAuth flows, token refreshes, and distinct API quirks for each provider. High engineering maintenance overhead.
- **Cloud vs. Standalone Compatibility:** Solutions like Nylas rely heavily on webhooks for instant calendar updates. In **Cloud mode**, webhook routing is straightforward. In **Standalone mode**, while polling could be used as a fallback for calendar sync (since real-time accuracy is slightly less critical than messaging), out-of-the-box webhook support would require a relay. Direct API integrations might be easier to manage entirely locally without third-party subscription costs, which is highly beneficial for Standalone users.

**Recommendation:** Utilize Nylas for an abstracted, robust calendar integration to quickly support all major calendar providers, ensuring we handle timezone logic perfectly.

## Design Doc
In the OHC dashboard, the user will have a "Calendar & Bookings" section. They can click "Connect My Calendar" and authorize their Google or Outlook account. OHC will read their events to block off busy times. The user can define their "Working Hours" and "Service Durations." Customers visiting the business owner's OHC booking page will see available slots displayed in the customer's local timezone. When a customer books a slot, OHC will automatically generate a Zoom or Meet link, insert the event into the owner's calendar, and email both parties the invitation.

## Implementation Prompt
Build a calendar connection feature that allows the business owner to securely link their Google or Outlook calendar. Develop a public-facing booking page where customers can view the owner's availability based on their connected calendar and configured working hours. When a customer selects a time, automatically generate a calendar event containing a generated video conference link, and block out that time on the owner's calendar. Ensure the customer booking interface automatically adjusts and displays times in the customer's local timezone to prevent confusion.

## Priority
P0

## Estimated Scope
Medium
