# Auto-Generate Video Links for Meetings via Zoom

**Problem Statement**
When a client books an online consultation, I have to manually create a Zoom meeting and email them the link. I sometimes forget, which causes confusion and delays. I want the video link to be created and sent automatically when they book.

**Research Report**
Zoom is the most widely recognized video conferencing tool. Its API allows for automatic meeting creation. For non-technical users, OHC can handle the OAuth connection and do the rest automatically. It has a free tier for short meetings, and Pro plans at $15/month. Link generation is instantaneous. It operates securely in both Cloud and Standalone environments.

**Design Doc**
The user will connect their Zoom account via the OHC settings. When a new meeting or consultation is scheduled (either manually or via the calendar integration), OHC will automatically generate a unique Zoom meeting link and attach it to the appointment details, visible to both the owner and the customer.

**Implementation Prompt**
Integrate Zoom to automatically generate meeting links. Allow the user to connect their Zoom account. Whenever a new online appointment is created in OHC, automatically generate a Zoom link and attach it to the appointment record. Acceptance criteria: A valid Zoom link is generated and saved when an appointment is scheduled.

**Priority:** P2
**Estimated Scope:** Small
