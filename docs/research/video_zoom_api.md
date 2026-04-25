# Zoom API Integration for Auto-Generated Lesson Links

## Problem Statement
Online tutors and consultants like Leo manually create Zoom links for every booking and email them to clients. This manual step often leads to forgotten links, confused clients, and delayed meetings.

## Research Report
- **Tool**: Zoom Meeting API
- **Evaluation**: The most ubiquitous video conferencing tool. The API allows creating unique meeting links programmatically.
- **Ease of Use for Persona**: The user clicks "Connect Zoom". From then on, every online booking magically includes a unique Zoom link.
- **Pricing**: API access is available for free/basic accounts (up to 100 API requests/day) and Pro accounts.
- **Reputation**: Ubiquitous and highly reliable.

## Design Doc
- **Integration Point**: "Operations" department.
- **Trigger**: Customer completes booking an "Online Video" service.
- **Actions**:
  - OHC calls Zoom API (via OAuth token of the business owner) to create a meeting for the scheduled time.
  - Zoom returns `join_url`.
  - OHC saves the URL to the booking and includes it in confirmation emails and calendar invites.
- **User View**: A "Connect Zoom" button in Settings. When viewing an upcoming appointment, a "Join Meeting" button is prominently displayed for both the business owner and the customer.

## Implementation Prompt
Add a "Connect Zoom" button via OAuth. When an online service is booked, use the Zoom API to generate a unique meeting link. Display this link in the booking confirmation UI, the owner's dashboard, and the automated calendar invites/emails.

## Priority
P2

## Estimated Scope
Medium
