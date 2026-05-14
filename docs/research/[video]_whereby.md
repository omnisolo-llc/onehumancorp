# [Video] Whereby Integration

## Title
Zero-Friction Video Consultations with Whereby

## Problem Statement
Leo (Music Tutor) is tired of students struggling to download Zoom or Meet. He needs a "one-click" video room that opens directly in the browser for his lessons, without any software installation or account creation for him or the student. This removes the primary barrier to starting an online session.

## Research Report
- **Strategy**: Embed Whereby video rooms via their API/SDK.
- **Target Persona**: Leo (Music Tutor), Consultants, Online Teachers.
- **Advantages**: Purely browser-based (WebRTC) — no downloads required. Minimalist, high-quality UI that feels like part of the OHC platform. Extremely easy for non-technical users.
- **Risks**: Lower brand recognition than Zoom, but higher "ease of use" for first-time students.
- **Pricing**: Generous free tier for 1:1 rooms. Embedded/API plans available for scaling.
- **Ease of Use**: Highest in class. Click a link and you are in.
- **Compatibility**: Cloud & Standalone (Browser-based).

## Design Doc
- **Integration with OHC**:
    - When a service marked as "Online" is booked, OHC calls the Whereby API to create a unique, temporary room.
    - The room link is automatically sent to the customer and displayed in the merchant's "Meetings" dashboard.
    - Clicking the link opens the video call directly within a browser tab or an iframe in the OHC app.
- **User View**: A "Join Lesson" button in the dashboard that opens the video call instantly.

## Implementation Prompt
Integrate Whereby for native, browser-based video conferencing. Implement the logic to automatically generate unique room URLs for scheduled appointments. Display these links in the OHC "Meetings" dashboard and include them in customer confirmation and reminder notifications.

## Priority
P2

## Estimated Scope
Medium
