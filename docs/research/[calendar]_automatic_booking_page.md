# Automatic Booking Page for Consultations

**Problem Statement:** Sarah (a tutor) spends hours emailing back and forth to find a time that works for her students. She wants to just send a link where students can pick a time, and it automatically shows up on her Google Calendar.

**Research Report:** Google Calendar API is the standard. Cal.com offers an open-source, robust scheduling infrastructure that handles timezone math, double-booking prevention, and integrates with Google Calendar and Outlook. It's free for individuals and highly regarded.

**Design Doc:** The user clicks "Connect Calendar" and signs in with Google. OHC generates a public booking link (e.g., `ohc.app/book/sarah`). When a customer picks a time, an event is automatically created on the user's Google Calendar and the customer receives an email confirmation.

**Implementation Prompt:** Add a Google OAuth flow for calendar access. Create a public-facing booking page that reads free/busy times from the user's calendar. When a time is selected, create a calendar event and notify both parties.

**Priority:** P0

**Estimated Scope:** Small
