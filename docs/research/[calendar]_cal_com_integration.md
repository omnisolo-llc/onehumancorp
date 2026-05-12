# OHC Tool Integration: Cal.com for Scheduling

## Title
Implement Cal.com Integration for Unified Booking

## Problem Statement
Small business owners lose leads due to the friction of back-and-forth emails when scheduling appointments, consultations, or classes. They need a simple, centralized way for customers to see availability and book directly without manual intervention.

## Research Report
- **Tool Evaluated:** Cal.com
- **Why Cal.com?** It's open-source, highly customizable, and offers a robust API. It naturally supports multiple calendar providers (Google, Outlook).
- **Ease of Use:** For the business owner, connecting a calendar is a standard OAuth flow. The customer sees a clean, modern booking interface.
- **Pricing:** Free for individuals; affordable team plans. Open-source nature allows for flexible scaling.
- **Reputation:** Strong developer community and growing adoption as a Calendly alternative.

## Design Doc
- **Trigger:** A customer clicks "Book Now" on a service or class listing in the OHC storefront.
- **Action:** OHC requests a booking page/link from Cal.com via API, pre-filled with service details (duration, price). The customer completes the booking, which updates the business owner's connected calendar and triggers an OHC internal event to log the booking.
- **User View:** The business owner sees a "Connect Calendar" button in their OHC settings. Once connected, services automatically display booking widgets.

## Implementation Prompt
Create the Cal.com integration module. The business owner must be able to authenticate their Cal.com account via the OHC settings page. Once connected, specific services (e.g., "1-hour Consultation") should dynamically display a booking widget or link on the storefront. The system must verify successful bookings via webhooks and display them in the OHC dashboard.

## Priority
P1

## Estimated Scope
Medium
