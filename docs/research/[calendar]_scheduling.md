# Calendar & Scheduling Integration

## Title
Automated Appointment Scheduling (Cal.com)

## Problem Statement
Small business owners (like consultants, personal trainers, or salon owners) waste significant time playing "email tag" to find a suitable time for client appointments. Manually checking availability, handling timezones, and creating meeting links is error-prone and frustrating for both the owner and the customer.

## Research Report
*   **Target Tools:** Cal.com (open-source alternative to Calendly).
*   **Pros:** Open source, highly customizable, robust API, excellent developer experience. Better suited for embedding into platforms than Calendly.
*   **Cons:** Can be overwhelming if exposing all advanced features to non-technical users.
*   **Ease of Use for Non-Technical Users:** High. Customers just click a link and pick a time. Business owners just need to connect their Google/Outlook calendar once.
*   **Pricing:** Free tier available. Paid tiers for advanced features (routing, workflows). API usage might require enterprise/platform pricing.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Easily integrates via OAuth and webhooks.
    *   *Standalone:* Can still work seamlessly as long as the user's Cal.com account acts as the source of truth, though webhook callbacks to local instances will face similar challenges to Meta APIs.

## Design Doc
1.  **Connection Flow:** In "Settings", user connects their external calendar (Google/Outlook) and configures their "Available Hours" (e.g., 9 AM - 5 PM).
2.  **Booking Page Generation:** OHC generates a unique public booking link (e.g., `ohc.com/book/acme`).
3.  **Customer Experience:** Customer clicks the link, sees a simple calendar interface, selects a time, and confirms.
4.  **Notification & Sync:** Both parties receive an email confirmation. The appointment is automatically blocked off on the owner's connected calendar.

## Implementation Prompt
Build a "Booking Page" feature. Allow the business owner to connect their Google Calendar or Outlook Calendar and set their working hours. Generate a public, shareable URL where clients can view available time slots and book appointments. When a client books a slot, automatically add it to the owner's connected calendar and send a confirmation email to both the owner and the client. Do not expose complex routing or round-robin features; keep the setup simple and straightforward.

## Priority
P1 (high)

## Estimated Scope
Medium
