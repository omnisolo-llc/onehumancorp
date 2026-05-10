# Title: Automated Scheduling via Acuity Integration
## Problem Statement
Service-based small business owners spend too much time going back and forth with clients to find meeting times. They need an easy way to let clients book available slots without manual coordination.

## Research Report
**Tool Evaluated:** Acuity Scheduling
- **Ease of Use:** Very high, intuitive setup for service businesses.
- **Pricing:** Starts at $20/month.
- **Reputation:** Highly trusted by coaches, consultants, and salons.
- **Advantages:** Excellent timezone handling, robust booking page customization.
- **Risks:** Less free tier value compared to alternatives.
- **Environment:** Fully supported in both Cloud and Standalone modes.

## Design Doc
The integration will authenticate via OAuth. OHC will sync availability and automatically generate a booking link for the business owner to share. When a client books, the appointment will instantly appear on the OHC calendar, and automated reminder sequences will be triggered.

## Implementation Prompt
Implement an Acuity Scheduling integration that syncs the business owner's availability and pulls new bookings into the OHC calendar. The user should be able to share a booking link directly from their OHC dashboard and see new appointments populate automatically.

## Priority
P1

## Estimated Scope
Medium
