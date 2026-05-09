# Video Conferencing Integration

**Problem Statement:**
Consultants, tutors, and remote service providers need an easy way to conduct meetings. Manually creating Zoom links, copying them, and emailing them to clients is tedious and prone to errors.

**Research Report:**
* **Tool Evaluated:** Google Meet (via Google Workspace API)
* **Ease of Use:** High, as most users already have a Google account. Generating the link is seamless once connected.
* **Pricing:** Included in standard Google Workspace subscriptions.
* **Reputation:** Ubiquitous, reliable, and requires no software installation for the client.
* **Hybrid Context:** Fully supported via cloud APIs.

**Design Doc:**
* **Trigger:** A virtual appointment is booked via the OHC scheduling system.
* **Action:** OHC requests a Meet link from Google and attaches it to the appointment record.
* **User Experience:** When a client books an online lesson, both the owner and the client immediately see a "Join Video Call" button on their dashboard/confirmation page. The link is automatically added to their calendar invites.

**Implementation Prompt:**
Enhance the appointment booking system to support "Virtual" locations. When an appointment is created with this location type, automatically generate a Google Meet link (requiring the owner to have connected their Google account). Display the join link prominently on the appointment details view.

**Priority:** P2
**Estimated Scope:** Medium
