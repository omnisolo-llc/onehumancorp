# Video Conferencing: Zoom

## Problem Statement
Tutors, consultants, and therapists need to easily generate a video meeting link when a client books an online session. Manually creating links and emailing them is tedious and prone to errors.

## Research Report
Zoom API allows programmatic meeting creation.
- *Ease of Use*: OAuth flow is standard, but the API has many configuration options to simplify.
- *Pricing*: Free tier allows basic API usage; Pro accounts needed for longer meetings.
- *Reputation*: Ubiquitous, everyone knows how to use Zoom.

## Design Doc
- *Trigger*: A customer books a service marked as "Online Meeting".
- *Action*: OHC creates a Zoom meeting via API and attaches the join link to the booking confirmation email and calendar invite.
- *User Interface*: In Services setup, a toggle for "This is an online meeting (Zoom)". The upcoming appointments dashboard shows a "Join Meeting" button.

## Implementation Prompt
Implement Zoom integration so that when a customer books a virtual service, a unique Zoom meeting is automatically generated. The user connects their Zoom account via OAuth. The resulting join link must be displayed in the business owner's dashboard and included in the customer's confirmation email.

## Priority
P2

## Estimated Scope
Medium

## Environment Support
Cloud, Standalone.
