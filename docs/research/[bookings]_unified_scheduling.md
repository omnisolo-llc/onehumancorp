# [Bookings] Unified Scheduling & Quoting for Handymen (Carlos)

## Title
Unified Scheduling & Quoting for Handymen

## Problem Statement
Carlos, a 42-year-old handyman, operates purely via word-of-mouth. He has no website, no booking system, and manual quoting causes him to miss leads when busy.

## Research Report
Service businesses are underserved by traditional e-commerce platforms like Shopify. Wix offers bookings, but it is siloed from the main CRM. Competitor analysis reveals a gap for an integrated, automated quoting tool.

## Design Doc
- **UI Flow:** Mobile dashboard showing upcoming jobs. A feature to snap a photo of a broken pipe and have AI generate a preliminary quote based on standard hourly rates.
- **Architecture:** `Booking` entity tied directly to `Quote` and `Invoice` entities.
- **AI Agent Integration:** Vision model to analyze job photos and draft quotes for approval.

## Implementation Prompt
Build a unified booking and quoting module where service providers can track availability, and AI assists in drafting estimates based on customer-provided photos and descriptions.

## Priority
P1

## Estimated Scope
Medium
