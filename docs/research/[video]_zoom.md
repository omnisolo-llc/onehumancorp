# Zoom Video Conferencing Integration

## Problem Statement
Small business owners who offer online services (like tutors, therapists, or consultants) struggle with manually creating video links for every booked appointment. Copying and pasting links into emails is prone to error and looks unprofessional. They need meeting links to be generated automatically when a client books a session.

## Research Report
Zoom is the ubiquitous video conferencing tool used globally.
- **Ease of Use**: Everyone knows how to join a Zoom meeting. The merchant setup via OAuth is straightforward.
- **Capabilities**: Auto-generates unique meeting links, handles waiting rooms, and integrates deeply with calendar invites.
- **Competitors**: Google Meet, Microsoft Teams, Jitsi. Zoom has the highest brand recognition and standalone utility for clients outside of corporate ecosystems.
- **Reputation**: The industry standard for reliable video calls.
- **Pricing**: Free tier allows up to 40-minute meetings. Pro plan is $14.99/month, which most serious remote businesses already pay for.
- **Deployment**: Exposes robust REST APIs for meeting creation. Fully compatible with Cloud and Standalone environments.

## Design Doc
OHC will integrate with Zoom to automatically generate meetings for virtual services. When a business owner defines a service in OHC as "Online", OHC will prompt them to connect their Zoom account. Upon a customer booking, OHC will call the Zoom API to create a meeting for that specific date and time. The generated join URL will be saved to the appointment record and automatically sent to the customer in their confirmation email.

## Implementation Prompt
In the Services setup area, add a "Location" dropdown with an option for "Zoom Meeting". If selected, prompt the user to link their Zoom account. When an appointment is booked for this service, automatically generate a Zoom link and display it on the merchant's "Upcoming Appointments" dashboard as a clickable "Start Meeting" button. Display the same link on the customer's portal as "Join Meeting".

## Priority
P1

## Estimated Scope
Medium
