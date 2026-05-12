# Daily.co Video Integration

## Problem Statement
Small business owners offering online lessons, coaching, or consultations struggle with generating and managing Zoom/Meet links. Clients get confused with passwords, app downloads, or lost links. They need frictionless, embedded video calls directly on their website.

## Research Report
Daily.co provides real-time audio and video APIs to embed video calls directly into web applications.
- **Ease of Use**: Incredible for the end-user. Clients just click a link on the business owner's site and join the call in their browser—no downloads or accounts required.
- **Pricing**: Very generous free tier (10,000 participant minutes/month). Pay-as-you-go after that.
- **Reputation**: Highly regarded by developers for reliability and ease of embedding.
- **Environment**: Cloud API. Works in Cloud and Standalone modes.

## Design Doc
**Trigger**: It's time for a scheduled consultation or online lesson.
**Action**: Both the business owner and the client click a "Join Call" button within their respective OHC portals/pages. Daily.co handles the WebRTC session.
**User Experience**: A video call interface opens directly inside the browser window. No external apps launch. The experience is branded and seamless.

## Implementation Prompt
Integrate Daily.co to provide embedded video conferencing. When an online appointment is booked, automatically provision a Daily.co room URL. Display a "Join Room" button in the dashboard for the business owner and in the portal for the client. The video call should happen within an iframe or a new tab, requiring no software installation.

## Priority
P2

## Estimated Scope
Large
