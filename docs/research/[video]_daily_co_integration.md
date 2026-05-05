# Title
Video Conferencing: Daily.co Integration for Virtual Services

# Problem Statement
For online service providers like Leo (The Music Tutor), managing Zoom links manually is tedious. They often forget to send the link, or customers lose it. They need a system that automatically creates a secure video room for every booked session and emails it to the client.

# Research Report
**Tool Analyzed:** Daily.co
Daily provides WebRTC video APIs to build video calls directly into apps or generate quick meeting links.
- **Ease of Use (for non-technical users):** Excellent. Daily offers prebuilt UIs that can be embedded directly into OHC or shared as standalone links.
- **Pricing:** 10,000 free minutes per month. Very generous for early-stage SaaS platforms.
- **Reputation:** Highly regarded among developers for simplicity and video quality compared to raw WebRTC or Zoom's heavy API.
- **Integration Risk:** Low. The API is designed for exactly this use case (creating temporary rooms via REST).
- **Cloud/Standalone:** Cloud API.

# Design Doc
- **Trigger:** A customer successfully books a virtual service slot (e.g., via the Cal.com integration or native OHC booking).
- **Actions:**
  1. OHC backend calls the Daily.co REST API to create a new, private video room with an expiration time matching the booking.
  2. The generated room URL is stored in the OHC database linked to the appointment.
  3. The Operations agent emails the URL to both the merchant and the customer.
  4. Optionally, OHC can embed the Daily Prebuilt UI directly into a "Virtual Session" tab in the dashboard.
- **User Experience:** Leo just sees his calendar fill up. When it's time for the lesson, he clicks "Join Room" in his OHC app. The student clicks the link in their email. No software downloads required.

# Implementation Prompt
Integrate Daily.co to automate the creation of video conferencing rooms for virtual bookings. When a virtual service is booked, the system must automatically generate a Daily room link and attach it to the booking record. Acceptance criteria include the successful API creation of a room, automated distribution of the link via email, and an option for the merchant to join the room directly from the OHC mobile/web app.

# Priority
P2

# Estimated Scope
Small
