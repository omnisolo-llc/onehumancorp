# Auto Quoting and Invoicing

**Priority:** P1
**Scope:** Medium

## Problem Statement
Leo (music tutor) manually creates invoices for each student every month, leading to delayed payments and tracking chaos.

## Research Report
- **Square:** Good invoicing but manual.
- **Data:** Manual invoicing leads to a 15% delay in getting paid on average.
- **Conclusion:** Recurring subscriptions and auto-generated invoices based on booking data will improve cash flow.

## Design Doc
- **Architecture:** `BookingSystem` emits `ServiceCompletedEvent`. `BillingAgent` drafts `Invoice` and sends `PaymentLink`.
- **UX Flow:** Lesson completes. System auto-sends invoice. Owner gets push notification when paid.

## Implementation Prompt
Develop a billing engine that automatically generates and sends invoices or subscription charges when a scheduled service is completed. Acceptance Criteria: System must automatically transition a booking to 'invoiced' and send an email/SMS payment link.
