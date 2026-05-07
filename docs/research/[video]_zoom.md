# Scout: Tool Integration Research

## [Video] Issue Brief: Zoom Meeting Generation
**Title**: Automated Zoom Meeting Generation for Online Services
**Problem Statement**:
Leo (Music Tutor) offers online lessons. Every time a student books, Leo has to manually create a Zoom link, copy it, and email it to the student. This is a manual task that can be easily forgotten, leading to a poor customer experience.

**Research Report**:
- **Tool**: Zoom Meeting API.
- **Evaluation**:
  - **Ease of Use**: High. One-time OAuth connection.
  - **Pricing**: Free tier allows 40-minute meetings; paid tiers for longer sessions. API access is included.
  - **Reputation**: The global standard for video conferencing.
  - **Cloud vs. Standalone**: Works in both via OAuth.
- **Key Advantages**: Fully automates the "Delivery" phase for digital service providers.
- **Risks**: OAuth token refresh management.

**Design Doc**:
- **User Flow**: User connects Zoom in "Integrations". In the Service Editor, they check "Generate Zoom link for this service".
- **Integration**: On successful booking, OHC calls Zoom API `POST /users/me/meetings`.
- **User Experience**: The generated link is automatically added to the customer's confirmation email and the merchant's "Upcoming Tasks" list.

**Implementation Prompt**:
Build an integration with the Zoom API to automatically generate unique meeting links for bookings. When a service is flagged as an "Online Meeting", the system should provision a Zoom meeting upon successful payment/booking and store the join URL. This URL should be injected into the automated customer confirmation emails.

**Priority**: P1
**Estimated Scope**: Small
