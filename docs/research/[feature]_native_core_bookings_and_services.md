# Native Core Bookings and Services

## Problem Statement
Service-based businesses (like Leo the music tutor and Carlos the handyman) struggle with disjointed workflows. They often have to use separate platforms for their website (e.g., Squarespace), booking (e.g., Calendly), and payments/invoicing. This leads to manual work, missed bookings, and a poor customer experience. They need a unified system where booking a service is as native and simple as buying a physical product.

## Research Report
Based on a deep competitor audit and SMB pain point analysis:
- **Pain Point #3**: "Manual Quoting and Booking" is a major source of lost time and revenue.
- **Competitor Gap**: Shopify is designed for physical products and requires complex workarounds or third-party apps for bookings. Wix has a separate bookings app that is not deeply integrated into the core product catalog.
- **AI Differentiation**: OHC's Operations agent can manage the calendar intelligently, and the Salesperson agent can follow up on abandoned quotes or bookings automatically.

## Design Doc
- **Core Entity**: `ServiceProduct` (a specialized product type) and `Booking` (links a `ServiceProduct`, `Customer`, and `TimeSlot`).
- **Key Relationships**: `Booking` links to `Calendar` and `PaymentIntent`.
- **UI Wireframes/Flow (Mobile-First 375px)**:
  - **Service Creation Flow**: A simple form to define a service (e.g., "1-Hour Plumbing Fix"), set a price, and define duration.
  - **Availability Setup**: Mobile-friendly toggle for working hours.
  - **Customer Booking Flow**: Clean calendar interface for the customer to select a time, followed by a native checkout screen for deposits.
- **AI Agent Integration**: The Operations agent manages the calendar, preventing double bookings. The Salesperson agent identifies users who started a booking but didn't finish and drafts a follow-up.

## Implementation Prompt
Implement a native booking system. This should include data models for services and bookings, integrating them into the core product catalog. The frontend must provide a mobile-first interface for the business owner to define services and availability, and a seamless booking flow for the end customer. The backend should handle calendar logic and integrate with the Operations AI agent to manage scheduling conflicts.

## Priority
P0

## Estimated Scope
Large
