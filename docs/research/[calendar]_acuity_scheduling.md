# Acuity Scheduling Integration Issue Brief

## Title
Integrate Acuity Scheduling for Automated Client Bookings

## Problem Statement
Service-based small business owners like Carlos the Handyman waste hours emailing back and forth with clients to find a suitable appointment time. They need a self-service booking system that automatically syncs with their calendar.

## Research Report
- Acuity Scheduling (by Squarespace) is a powerful, customizable scheduling tool designed specifically for small and medium businesses.
- It handles complex scheduling rules, timezone conversions, and even takes payments upfront.
- Pricing: Reasonable monthly plans, though lacking a robust free tier compared to Calendly.
- Competitors: Calendly is simpler but less customizable for complex service businesses. Cal.com is open-source but might be too complex for non-technical users.
- Integration: Acuity has a comprehensive REST API and webhooks for syncing appointments.
- Cloud/Standalone: Works well in Cloud. Standalone mode might require webhooks to be proxied or direct API polling.

## Design Doc
- Users connect their Acuity Scheduling account in the "Appointments" section.
- OHC automatically pulls in the user's available appointment types and booking links.
- The "Concierge" AI agent can provide the booking link to clients in automated email replies or SMS messages.
- New bookings trigger webhooks that update the OHC unified calendar view.

## Implementation Prompt
Implement an integration with Acuity Scheduling using their API. Allow users to connect their account, fetch their appointment types, and display their booking link in the OHC dashboard. Set up webhook listeners to receive notifications about new, rescheduled, or canceled appointments and update the OHC internal calendar accordingly.

## Priority
P1

## Estimated Scope
Medium
