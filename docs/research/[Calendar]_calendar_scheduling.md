# [Calendar] OHC Tool Integration Research Brief: Calendar & Scheduling

## Title
Automated Meeting Scheduling and Calendar Sync

## Problem Statement
Small business owners (consultants, tutors, service providers) waste hours playing "email ping-pong" to find a time to meet with clients. They need a way to share a link where clients can pick an available time, and have that appointment automatically show up on their personal calendar (Google/Outlook) with an auto-generated Zoom or Google Meet link.

## Research Report
The scheduling space is mature. Building a reliable calendar sync and timezone engine from scratch is notoriously difficult, making third-party integration essential.

**Evaluated Tools:**

1. **Calendly**
    *   **Focus:** The industry standard for scheduling.
    *   **Pros:** Everyone knows it. Huge integration ecosystem.
    *   **Cons:** Expensive for teams. Their API is good, but embedding it natively without it looking like Calendly is hard.

2. **Cal.com**
    *   **Focus:** Open-source scheduling infrastructure.
    *   **Pros:** Incredible developer experience. We can self-host it for Standalone mode, or use their managed Cloud offering. White-labeling is a core feature (Cal.com Atoms). Handles timezones, Google/Outlook OAuth, and video links (Zoom/Meet) seamlessly.
    *   **Cons:** Slightly newer than Calendly, but rapidly becoming the developer default.
    *   **Pricing:** Free for individuals. Platform/API pricing is flexible.

**Recommendation:**
**Cal.com** is the definitive winner, especially given OHC's dual Cloud/Standalone architecture. Cal.com's open-source nature means we could potentially embed or self-host the scheduling engine for Standalone users, while using their managed API for Cloud users. Their React components ("Atoms") allow for deep, white-labeled integration.

## Design Doc
**Integration Approach: Cal.com API & Atoms**

1.  **Setup:**
    *   Business owner connects their Google/Outlook calendar via OHC (powered by Cal.com's OAuth under the hood).
    *   They define their availability hours in the OHC UI (which syncs to Cal.com).

2.  **Booking Experience (User View):**
    *   Customers visit the business owner's OHC-hosted profile page.
    *   They see a calendar widget (rendered via Cal.com Atoms) showing available slots in their local timezone.
    *   Customer selects a slot, enters their name and email, and confirms.

3.  **Action:**
    *   Cal.com handles the calendar insertion for both parties.
    *   Cal.com auto-generates a video conference link if configured.
    *   A webhook from Cal.com notifies OHC of the booking.
    *   OHC creates an `Appointment` record linked to the `Customer`, enabling future follow-ups or billing.

## Implementation Prompt
**Objective:** Integrate Cal.com for scheduling and appointment management.

**Acceptance Criteria:**
1.  Implement an API client to communicate with the Cal.com API (create event types, fetch availability).
2.  Create an `Appointment` database model to store booking details (Time, Duration, CustomerId, Status, MeetingLink).
3.  Implement a webhook receiver that listens for `BOOKING_CREATED` and `BOOKING_CANCELLED` events from Cal.com.
4.  When a `BOOKING_CREATED` webhook is received, verify the signature, extract the customer email, find or create the Customer in OHC, and save the `Appointment` record.

## Priority
P0

## Estimated Scope
Medium
