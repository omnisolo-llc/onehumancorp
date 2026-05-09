# Calendar Scheduling Integration

**Problem Statement:**
Booking consultations or services often involves back-and-forth emails or texts ("What time works for you?"). This creates friction, lost revenue, and accidental double-bookings for busy small business owners.

**Research Report:**
* **Tool Evaluated:** Cal.com API
* **Ease of Use:** Extremely high for the end client. The business owner only needs to connect their primary calendar once.
* **Pricing:** Open source core; premium API features scale with usage.
* **Reputation:** Excellent developer experience and highly customizable.
* **Hybrid Context:** Ideal for OHC. Cal.com can be integrated via API in Cloud mode, and its open-source nature means it could potentially be bundled or self-hosted alongside Standalone mode in the future.

**Design Doc:**
* **Trigger:** A business owner enables "Online Booking" in OHC and sets their availability.
* **Action:** OHC generates a public booking link powered by Cal.com logic.
* **User Experience:** The owner shares their OHC booking link. Clients click it, see available slots, and book a time. The appointment automatically appears on the owner's OHC dashboard and syncs to their personal Google/Outlook calendar.

**Implementation Prompt:**
Build a scheduling interface in the OHC dashboard where users can define their working hours and connect an external calendar (Google/Outlook) to prevent conflicts. Generate a public-facing booking page for the business. When a client books a slot, the system must record the appointment and block out that time.

**Priority:** P1
**Estimated Scope:** Medium
