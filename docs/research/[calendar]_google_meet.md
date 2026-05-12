# Title
Native Google Calendar & Meet Sync

# Problem Statement
Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs an automated system where customers can pick an available slot, and the system instantly books the calendar and provides a video link without double-booking or manual email back-and-forth.

# Research Report
- **Tool:** Google Calendar API and Google Meet (via Google Workspace).
- **Target Persona:** Leo (Music Tutor), Carlos (Handyman) and other service providers.
- **Advantages:** Google OAuth is universally understood. Non-technical users are very familiar with Google Calendar.
- **Risks:** Generating Google Meet links programmatically requires the user to have a Google Workspace account.
- **Pricing:** Free for basic calendar syncing. Workspace is required for Meet links.
- **Compatibility:** Works seamlessly in Cloud and Standalone environments as it relies on standard OAuth 2.0 and REST API calls. Standalone requires secure local storage of OAuth refresh tokens.

# Design Doc
- **Integration Trigger:** User enables "Online Bookings" in the Operations department and clicks "Sync Google Calendar".
- **User Flow:** User completes Google OAuth, granting calendar read/write permissions.
- **Action Flow:** OHC reads the user's free/busy schedule to display available slots on their public storefront. When a customer books a slot, OHC creates a Calendar Event, optionally attaches a generated Google Meet link (if it's an online service), and adds the customer as an attendee so they receive the invite automatically.

# Implementation Prompt
Build a two-way sync integration with Google Calendar. The system must fetch the business owner's free/busy times to dynamically update availability on their OHC booking page. When a new booking is made, it must create an event in the owner's Google Calendar and automatically generate and attach a Google Meet link for virtual services (like tutoring). Ensure calendar conflicts are strictly prevented.
- **Acceptance Criteria:** Merchant can connect Google Calendar. Customers can view availability and book natively. Events sync to Google Calendar. Meeting links are auto-generated.
- **Priority:** P1
- **Estimated Scope:** Medium
