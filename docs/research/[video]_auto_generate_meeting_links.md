# Auto-generate Meeting Links for Online Consultations

**Problem Statement:** Sarah does online tutoring. Right now, she manually creates a Zoom link for every student and emails it to them. She forgets sometimes and they both sit waiting. She needs the video link to be generated automatically when they book.

**Research Report:** Zoom's API is ubiquitous but their OAuth approval process for public apps is stringent. Alternatively, integrating with Google Meet (via Calendar API) is completely transparent if they already use Google Calendar.

**Design Doc:** When a user configures a "Service" in OHC, they can check "This is an online meeting". When a customer books, OHC uses the connected Google Calendar to automatically attach a Google Meet link to the event, which is shared with the customer.

**Implementation Prompt:** Update the booking flow so that if an online service is selected, the integration automatically injects a video conferencing link (e.g., Google Meet via Calendar API) into the booking confirmation and calendar invite.

**Priority:** P2

**Estimated Scope:** Small
