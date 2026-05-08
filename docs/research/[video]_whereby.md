# Scout: Tool Integration Research [Q2]

## [Video Conferencing] Issue Brief: Whereby Integration

**Title**: One-Click Branded Video Consultations via Whereby

**Problem Statement**:
Leo (Music Tutor) and other service-based business owners need a way to host online sessions that feels professional and branded. Currently, they have to send students to external apps like Zoom or Google Meet, which requires students to download apps or create accounts, creating friction and looking less professional than a native solution.

**Research Report**:
- **Tool**: Whereby Embedded.
- **Evaluation**: Whereby allows browser-based video calls with zero downloads or logins required for the participants. It is highly brandable.
- **Ease of Use**: Exceptional. Students just click a link in their email or dashboard and the video call opens in their browser.
- **Pricing**: "Build" plan starting at $9.99/mo for small businesses, making it very accessible.
- **Reputation**: Known for the best user experience in the "no-download" video category.
- **Cloud vs. Standalone**: Works in both. Requires a Whereby API key.

**Design Doc**:
- When a user creates a service, they can select "Whereby Video Room" as the location.
- Upon booking, OHC calls the Whereby API to generate a unique, temporary room URL.
- This URL is embedded in the customer's "My Appointments" page and the merchant's dashboard.
- The video call can be embedded directly within the OHC interface using an `<iframe>` for a completely native feel.

**Implementation Prompt**:
Integrate Whereby Embedded for online service bookings. Automatically generate unique room URLs for appointments and provide an embedded video interface within the OHC platform for both the merchant and the customer.
- **Acceptance Criteria**: Unique Whereby room is generated for every online booking. Both parties can join the call directly from the OHC dashboard without downloads.
- **Priority**: P2
- **Estimated Scope**: Small
