# Video Conferencing

## Title
[Video] Automated Zoom/Meet Link Generation for Services

## Problem Statement
Service providers like Leo (The Music Tutor) conduct their business online. Manually creating a Zoom link for every booked lesson and emailing it to the student is tedious and prone to human error.

## Research Report
- **Evaluated Tools**: Zoom API, Google Meet API (via Google Calendar), Whereby.
- **Ease of Use**: Whereby embedded links are easiest to generate via API. Zoom requires OAuth setup. Google Meet is seamless if the user already connected Google Calendar.
- **Pricing**: Zoom requires a paid plan for API access. Google Meet is free with Calendar. Whereby has API pricing.
- **Join Experience**: Critical that students can join without installing new software if possible.
- **Cloud vs Standalone**: Fully supported in both modes.

## Design Doc
- **Triggers**: A customer books an online service.
- **Actions**: System creates a meeting via the provider's API and attaches the join URL to the booking confirmation and calendar event.
- **User View**: A setting to "Enable Video Meeting for this Service". The booking confirmation automatically includes a big "Join Meeting" button.

## Implementation Prompt
Build an automated video meeting integration. For services marked as "Online", the system should automatically generate a unique meeting link (e.g., using Google Meet or Zoom) upon booking. This link must be included in the customer's confirmation email, the calendar invite, and displayed prominently in the business owner's upcoming appointments dashboard.
- **Acceptance Criteria**: Business owner can mark a service as an "Online" service. When a customer books an "Online" service, the system automatically generates a unique video meeting link (e.g., Zoom or Google Meet). The generated link is automatically included in the booking confirmation email sent to the customer, the calendar invite for both parties, and displayed prominently on the business owner's upcoming appointments dashboard.

## Priority
P2

## Estimated Scope
Small
