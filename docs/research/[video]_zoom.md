# Title: Native Zoom Link Generation for Appointments

## Problem Statement
Tutors like Leo manually create a Zoom link for every new lesson and email it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically when a lesson is booked.

## Research Report
- **Tool Evaluated**: Zoom
- **Persona Value**: High. Reduces manual work and looks professional.
- **Advantages**: Ubiquitous, standard OAuth connection process.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing**: API is free for Zoom users, but requires the merchant to have an account.
- **Cloud vs Standalone**: Cloud (OAuth). Standalone (Server-to-Server OAuth).

## Design Doc
- **Integration Trigger**: Service creation flow: user selects "Online Meeting" and connects Zoom. Customer books service.
- **Action**: OHC calls Zoom API to create a meeting, retrieves URL, and embeds it in calendar invites and emails.
- **User Interface**: "Connect Zoom" button during service setup.

## Implementation Prompt
Build a Zoom integration that automatically creates meeting links for online service bookings. Users should be able to connect their Zoom account via OAuth. When a customer books, dynamically generate a Zoom link and share it.
- **Acceptance Criteria**: Merchant connects Zoom. Customer books online service. Unique Zoom link is generated and sent to both parties.

## Priority
P2

## Estimated Scope
Medium
