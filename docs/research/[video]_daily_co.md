# Scout: Tool Integration Research

## Video Conferencing
**Title**: Integrate Daily.co for Embedded White-Label Video Consultations
**Problem Statement**: Tutors and consultants want to offer premium, branded video sessions directly on their own website domain. Sending clients away to Zoom or Google Meet feels disjointed and unprofessional.
**Research Report**:
- Daily.co provides developer-first WebRTC APIs and pre-built video UI components that can be deeply embedded into any web application.
- It completely white-labels the experience; the user never knows they are using Daily.co. No external apps or logins are required for the client.
- Pricing: Extremely generous free tier (10,000 minutes/month), which easily covers the needs of an average solo consultant.
- Compatibility: Perfect for Cloud mode (centralized token generation). Can work in Standalone mode if the user provides their own API key, though slightly more technical to set up for them.
**Design Doc**:
- When a service is booked as an "Online Consultation", OHC uses the Daily.co API to generate a temporary video room URL.
- When the meeting time arrives, the client visits their OHC customer portal, and the video interface (Daily.co Prebuilt) renders directly inside the OHC webpage frame.
- The business owner has a branded waiting room and full controls, all without leaving their OHC dashboard.
**Implementation Prompt**: Integrate Daily.co's API to dynamically generate meeting rooms for online service bookings. Embed the Daily.co Prebuilt UI into the OHC client portal and business dashboard so that video consultations occur entirely within the OHC ecosystem.
**Priority**: P1
**Estimated Scope**: Large