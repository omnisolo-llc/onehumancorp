# Title: Integrate Nylas for Calendar Sync and Meeting Generation

**Problem Statement:** Leo (Tutor) and Carlos (Handyman) need reliable booking systems that sync with their personal Google/Outlook calendars to avoid double booking.

**Research Report:** Nylas offers powerful Email, Calendar, and Contacts APIs. Calendly provides a great standalone service, but Nylas allows deep, invisible integration (white-label) directly into the OHC platform. Nylas supports 250+ providers.

**Design Doc:** Integrate Nylas Calendar API. Users securely connect their calendar accounts via OAuth. When a customer books a time through the public storefront, the system automatically checks availability and creates an event on the business owner's personal calendar.

**Implementation Prompt:** Build a booking flow where a customer selects a time slot. Use Nylas to check the business owner's availability and create calendar events upon booking.

**Priority:** P0

**Estimated Scope:** Medium
