# Title: Automate Meeting Links with Zoom API

**Problem Statement:** Leo (Tutor) spends too much time manually creating and emailing Zoom links for every lesson he books.

**Research Report:** Zoom has a dominant market share and offers robust APIs for meeting creation. Nylas also supports Zoom integration, but direct Zoom API might offer more control for Webinar/Education features.

**Design Doc:** Automatically generate and attach meeting links to service bookings. The link should be included in confirmation emails and calendar events sent to both the student and the tutor.

**Implementation Prompt:** Modify the service booking flow so that if the service is "Virtual", a Zoom link is automatically generated and attached to the booking confirmation email and calendar event.

**Priority:** P1

**Estimated Scope:** Small
