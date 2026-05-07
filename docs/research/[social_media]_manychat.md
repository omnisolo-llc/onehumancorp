# Native Integration of Manychat for Social Media DMs

## Title
Native Integration of Manychat for Social Media DMs

## Problem Statement
Small business owners, especially those selling on Instagram and Facebook, struggle to manage the volume of Direct Messages. They often miss sales inquiries or take too long to respond. They need a way to automatically reply to common questions and capture leads directly within their unified OHC inbox without needing to learn a complex chatbot builder.

## Research Report
- **Strategy**: Direct API integration with Manychat to handle Instagram/Facebook DMs.
- **Target Persona**: Boutique owners, online sellers, and service providers heavily relying on Instagram/Facebook for customer acquisition.
- **Advantages**: Manychat is an industry leader in chat automation. Integrating it natively means users get powerful auto-replies without the steep learning curve of setting it up from scratch.
- **Risks**: Requires users to authenticate their social accounts through Manychat, adding an extra step to onboarding. Meta's API changes can affect stability.
- **Pricing**: Manychat offers a free tier (up to 1,000 contacts) and a Pro tier starting at $15/mo. Highly affordable for small businesses.
- **Compatibility**: Works in both Cloud mode (OHC manages a centralized integration or users connect their own) and Standalone mode (users provide their own API keys or OAuth).

## Design Doc
- In the OHC settings, the user navigates to "Social Integrations" and clicks "Connect Instagram/Facebook via Manychat".
- The user goes through the standard Manychat OAuth flow to authorize access.
- OHC automatically provisions a set of default "quick replies" (e.g., store hours, location, link to storefront) based on the user's business profile.
- Incoming DMs and automated replies appear seamlessly in the OHC unified inbox.
- **AI Integration**: The Customer Service Agent can monitor the Manychat conversations and take over when a query falls outside the automated rules.

## Implementation Prompt
Integrate Manychat to handle inbound social media direct messages (Instagram and Facebook). The integration should support OAuth connection from the OHC dashboard. OHC should sync conversations from Manychat to the internal unified inbox and allow the AI agent or the user to respond directly from OHC.
- **Acceptance Criteria**: User can connect Manychat via OAuth. Inbound Instagram/Facebook DMs appear in the OHC inbox. User/Agent can reply from OHC, and the message is sent via Manychat.
- **Priority**: P1
- **Estimated Scope**: Medium
