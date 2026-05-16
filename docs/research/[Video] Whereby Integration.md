# Title: Whereby Video Conferencing Integration

## Problem Statement
Service-based small business owners (tutors, consultants, therapists) increasingly offer virtual sessions. Expecting clients to download software, create accounts, or navigate complex calendar invites with third-party Zoom links causes friction and missed appointments. They need a simple, 1-click video call solution embedded directly into their workflow.

## Research Report
*   **Overview**: Whereby provides simple, browser-based video meetings with no downloads or logins required for guests. They offer an "Embedded" product specifically for integrating video directly into platforms like OHC.
*   **Ease of Use**: Unmatched for end-users. Both the business owner and the client simply click a link and the video call opens instantly in their browser window.
*   **Reputation**: Known for its beautiful, minimalist UI, ease of use, and strong privacy focus (European alternative).
*   **Pricing**:
    *   **Meetings (Standalone)**: Pro plans for individuals/teams.
    *   **Embedded (API)**: "Build" plan is $9.99/month (includes 2,000 participant minutes). Additional minutes are $0.004/minute.
*   **Environment (Cloud vs Standalone)**: API-based. Works perfectly in Cloud. Works in Standalone as long as outbound internet access is available to generate room URLs via the API.
*   **AI Integration**: Whereby offers session transcriptions and live captions (via add-ons), which could feed directly into OHC AI agents for meeting summaries.

## Design Doc
*   **Trigger**: A client books a virtual appointment, or a business owner clicks "Start Video Call" from an active client chat.
*   **Action**: OHC calls the Whereby API to generate a unique, temporary room URL. This URL is embedded in an iframe within the OHC interface or sent to the client.
*   **User Interface**: A seamless video window embedded directly inside the OHC dashboard. The owner doesn't need to open a separate app; they manage the call directly alongside their customer notes.

## Implementation Prompt
Integrate the Whereby Embedded API to provide native video conferencing. The user-facing outcome should allow a business owner to generate a 1-click video meeting link for virtual appointments. The video interface should be embedded directly within the OHC platform using Whereby's SDK/iframe, ensuring neither the owner nor the client has to download external software or leave the OHC ecosystem.

## Priority
P2

## Estimated Scope
Medium
