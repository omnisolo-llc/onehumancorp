# Title: Video Conferencing via Daily.co

## Problem Statement
Leo, the music tutor, teaches online. Currently, he has to manually create a Zoom link, copy it, and email it to his student before every lesson. If he forgets, the lesson is delayed. He needs a seamless way for video rooms to be created automatically when a lesson is booked, without requiring his students to download any software.

## Research Report
Daily.co provides developer-friendly APIs for embedding WebRTC video calls directly into applications.
- **Ease of Use for Non-Technical Users**: Neither Leo nor his students need to download an app or create an account. They just click a "Join Lesson" link that opens a high-quality video call right in their browser.
- **Pricing**: Generous free tier (up to 10,000 minutes/month), making it effectively free for a single tutor like Leo.
## Risks
- **Risks**: Dependency on the user('s) internet connection quality for WebRTC to function well.

## Reliability & Reputation**: Excellent WebRTC performance, low latency, and highly customizable UI.
- **Environment Support**: Works in browsers via web components, perfect for Cloud and Standalone desktop web views.

## Design Doc
The "Operations" agent handles the virtual classroom.
1. **Trigger**: A student books an online lesson.
2. **Action**: A Daily.co video room is dynamically provisioned. The room link is embedded into the calendar invite and the OHC student portal.
3. **User View**: At the time of the lesson, Leo and the student click "Join Video" in the OHC app. A beautiful, branded video interface opens within the app itself.

## Implementation Prompt
Integrate Daily.co's REST API to dynamically generate temporary video rooms. Embed the Daily Prebuilt UI component into the OHC frontend so that video calls happen within the platform rather than kicking users out to an external app. Link the room creation logic to the booking flow so that every remote service booking automatically generates a unique, secure video link.

## Priority
P2

## Estimated Scope
Medium
