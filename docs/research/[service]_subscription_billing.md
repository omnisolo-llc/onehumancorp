# [Service] Subscription Billing for Tutors (Leo)

## Title
Integrated Subscription Billing and Calendar Sync for Tutors

## Problem Statement
Leo, a 22-year-old music tutor, offers online and in-person lessons. He struggles with manual booking chaos, lacks a subscription billing option, and has no AI follow-up system for missed payments or reschedules.

## Research Report
The tutoring and recurring service market relies heavily on disparate tools (Calendly + Venmo/Stripe). Existing platforms like Squarespace bolt on membership features poorly. There is a massive demand for a unified scheduling and recurring billing primitive.

## Design Doc
- **UI Flow:** "Create Service" flow includes a toggle for "Recurring Weekly/Monthly". The checkout flow enforces saving a payment method.
- **Architecture:** `Service` entity linked to `Subscription` engine, integrated with Stripe Billing for dunning management. `Booking` engine syncs via Google Calendar API.
- **AI Agent Integration:** AI agent monitors failed payments and drafts polite follow-up emails, or suggests alternate time slots if a cancellation occurs.

## Implementation Prompt
Implement a recurring service product type that combines calendar availability constraints with automated subscription billing and AI-driven cancellation recovery.

## Priority
P1

## Estimated Scope
Large
