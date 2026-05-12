# Automated Client Scheduling and Booking

## Problem Statement
Manual back-and-forth for scheduling appointments takes too much time and causes double-bookings.

## Research Report
Calendly and Cal.com evaluated. Cal.com offers better open-source integration possibilities and self-hosted options, aligning well with OHC Standalone mode. Calendly is strictly cloud-based and costly.

## Design Doc
Integrate Cal.com APIs to allow OHC to sync with Google/Outlook calendars and generate booking links. The business owner creates event types in OHC, which generate Cal.com links behind the scenes.

## Implementation Prompt
Implement a 'Booking' tab where users can connect their Google Calendar and create shareable booking links that automatically respect their availability.

## Priority
P1

## Estimated Scope
Medium
