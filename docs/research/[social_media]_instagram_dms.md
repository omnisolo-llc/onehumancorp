# Issue Brief: Instagram DMs Integration

## Title
Implement Instagram DMs Integration for OHC unified dashboard.

## Problem Statement
Small business owners struggle with context switching. Managing social media operations across multiple disparate apps leads to missed opportunities, delayed responses, and customer dissatisfaction. For non-technical users, logging into 5 different portals daily is an operational nightmare. They need a single pane of glass to oversee their business without understanding API keys or webhooks. Specifically, the burden of manually synchronizing state between their social media tool and their primary business ledger is a massive time sink.

## Research Report
### Overview
Instagram DMs Integration represents a critical pillar in modern SMB operations. Integrating this tool allows the OHC platform to act as the central nervous system for the business.
External Data Synthesis: Instagram is an American photo and short-form video sharing social networking service owned by Meta Platforms. It allows users to upload media that can be edited with filters, be organized by hashtags, and be associated with a location via geographical tagging. Posts can be shared publicly or with preapproved followers. Users can browse other users' content by tags and locations, view trending content, like photos, and follow other users to add their content to a personal feed. A Meta-operated image-centric social media platform, it is available on iOS, Android, Windows, and the Web. Users can take photos and edit them using built-in filters and other tools, then share them on other social media platforms like Facebook. It supports 33 languages including English, Hindi, Spanish, French, Japanese, and Korean.
Instagram was originally distinguished by allowing content to be framed only in a square (1:1) aspect ratio of 640 pixels to match the display width of the iPhone at the time. In 2015, this restriction was eased with an increase to 1080 pixels. It also added messaging features, the ability to include multiple images or videos in a single post, and a Stories feature—similar to its main competitor, Snapchat, which allowed users to post their content to a sequential feed, with each post accessible to others for 24 hours. As of January 2019, Stories was used by 500 million people daily.

The Burbn Beta app was made available for iOS on October 6, 2010, by Kevin Systrom and Mike Krieger — still as a testing prototype, but for the first time available as a real app on the App Store rather than a web-based prototype. The app received around 25,000 registrations that day, even though it was built for testing purposes only. Six days later, on October 12, 2010, the final stable non-beta version was officially released under a new name — Instagram. This was the first official public release, not a testing version. This renaming gave an even bigger response: the app rapidly gained popularity, reaching 1 million registered users in two months, 10 million in a year, and 1 billion in June 2018. In April 2012, Facebook acquired the service for approximately US$1 billion in cash and stock. The Android version of Instagram was released in April 2012, followed by a feature-limited desktop interface in November 2012, a Fire OS app in June 2014, an app for Windows 10 in October 2016, and an app for iPadOS in September 2025. Although often admired for its success and influence, Instagram has also been criticized for negatively affecting teens' mental health, its policy and interface changes, its alleged censorship, and illegal and inappropriate content uploaded by users.

### Ease of Use (Non-Technical Persona)
The target audience (e.g., Fatima the baker, Jose the plumber) requires zero-configuration setups. We analyzed the onboarding flows of Instagram DMs Integration. The standard OAuth 2.0 flow is acceptable, provided we abstract away token refresh mechanisms. The UI must speak plain language: "Connect your account" instead of "Authorize Application". Furthermore, error states must be translated from technical API responses (e.g., '401 Unauthorized') into actionable advice ('Your connection expired. Please log in again.').

### Pricing & Business Model
Free API access, potential cost for volume scaling through third-party aggregators.
Based on market standards for social_media, pricing models generally involve either per-transaction fees or monthly tiered subscriptions. For our Standalone users, we must ensure the free-tier API limits of Instagram DMs Integration are sufficient for a single small business. For Cloud users, we must negotiate enterprise rates or utilize sub-merchant routing to avoid aggregate rate limits.

### Competitive Analysis
When compared to alternatives in the social_media space, Instagram DMs Integration holds significant market share, making it a high-confidence integration target.
- **Pros**: High reliability, widespread consumer trust, extensive documentation. Directly engage with customers sharing visually appealing bakery goods or home renovation projects.
- **Cons**: Potential data lock-in, rate limiting on free tiers, complex error states. Requires strict adherence to the 24-hour response window policy for automated messaging.
- **Context**: Dominates visual-first markets over Twitter or text-heavy platforms.

### Mode Compatibility
- **Cloud Mode**: Fully compatible. Webhooks can be routed through our standard ingestion pipeline. Multi-tenant isolation is standard. We will utilize our Redis cluster to handle bursty webhook traffic to ensure no payloads are dropped during high-volume events.
- **Standalone Mode**: Compatible, but requires local polling or secure tunneling if webhooks are strictly required by Instagram DMs Integration. Local encrypted SQLite storage is sufficient for caching. We must ensure that the polling interval is tuned to balance responsiveness with API rate limits, defaulting to a 5-minute jittered interval.

### Security and Privacy Implications
Integrating Instagram DMs Integration requires careful handling of PII (Personally Identifiable Information). We must ensure that OAuth scopes requested are the absolute minimum necessary to fulfill the integration's purpose (Principle of Least Privilege). In Standalone mode, OAuth tokens must be encrypted at rest within the SQLite database using go-sqlcipher. In Cloud mode, tokens must be securely stored in our vault infrastructure, completely isolated per tenant.

### Onboarding Heuristics
To ensure successful adoption, the integration must include a 'test connection' heuristic. Immediately after the user connects Instagram DMs Integration, the platform should perform a silent API call to fetch a trivial piece of data (e.g., account profile name) to verify the connection is healthy. If this fails, the user should be immediately notified and guided through troubleshooting steps, rather than discovering the failure days later when expected data is missing.

## Design Doc
### User Experience
1. User navigates to Settings > Integrations in the OHC dashboard.
2. User sees a card for Instagram DMs Integration highlighting the core value proposition.
3. User clicks "Connect Instagram DMs Integration".
4. A secure popup handles authentication.
5. Upon return, the status immediately reflects "Connected" with a green indicator.
6. Relevant data (e.g., new messages, calendar events, payment statuses) begins flowing into the unified activity feed.

### Integration Flow
- **Trigger**: User OAuth completion.
- **Action**: OHC platform stores the connection securely and begins bidirectional sync.
- **Visibility**: The user sees plain-language updates in their main feed, like "New appointment booked via Instagram DMs Integration".

## Implementation Prompt
Implement the Instagram DMs Integration integration. Ensure the setup flow is accessible to non-technical users. The final outcome must be a fully connected state where data flows seamlessly into the user's unified dashboard. Use plain language for all labels and error messages. Ensure compatibility for both Cloud (PostgreSQL/Redis) and Standalone (SQLite) modes. Follow OHC Premium Design Standards for all UI elements (Outfit font, Inter body, glassmorphism). Do not implement complex retry logic in the initial PR; focus on establishing the core connection and data ingest pipeline.

## Priority
P1

## Estimated Scope
Medium
