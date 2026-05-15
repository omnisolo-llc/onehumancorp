# Integrate TikTok Comments into Unified Inbox

## 1. Problem Statement
Small business owners often go viral on TikTok but struggle to keep up with hundreds of comments asking 'how much?' or 'do you ship to XYZ?'. Missing these comments means losing direct sales because checking the app constantly is exhausting and inefficient. When an entrepreneur posts a video showcasing a new product, the algorithm can suddenly push it to thousands of viewers. The creator receives a flood of notifications, making it impossible to separate genuine purchase inquiries from general engagement. Without a way to manage this efficiently, the business owner leaves money on the table and frustrates potential buyers who expect rapid responses.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
TikTok is currently the fastest-growing customer acquisition channel for many SMBs, particularly in retail and beauty. Engaging quickly with commenters can significantly increase conversion rates. TikTok's APIs allow fetching and replying to comments. While the OAuth flow can be slightly daunting for a non-technical user, if guided properly, it's highly beneficial. The primary advantage is centralizing the chaotic TikTok notifications into a clean, manageable inbox where the owner can reply alongside their other social channels. Pricing for the integration itself would be part of OHC's standard offering, adding immense value for free or a small premium. It operates seamlessly in both Cloud and Standalone environments, as it primarily relies on API polling/webhooks. Furthermore, TikTok Shop is becoming a massive driver of revenue, and having the initial conversational touchpoint handled inside the OHC ecosystem keeps the user from churning to other platforms.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Freemium tier available; standard API usage rates apply thereafter.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
The user will see a new 'Connect TikTok' button in their Social Settings page. Clicking it will walk them through a secure login screen provided by TikTok. Once connected, all new comments on their videos will appear as individual threads in their main OHC Inbox. The user can type a reply in the OHC Inbox, and it will be posted directly under the customer's comment on the TikTok video. They will also be able to see the video thumbnail next to the comment for context. The interface should group comments by video to provide context, and highlight comments that contain intent-based keywords like 'buy', 'price', or 'shipping'.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Create a seamless connection flow for TikTok in the Settings menu. Build the UI components to display short-form video comments within the main Inbox, including visual context (like the video thumbnail). Ensure that when a user types and sends a reply, the UI updates instantly and handles any connectivity errors gracefully with a friendly retry prompt. Implement a visual indicator for high-intent comments to help the business owner prioritize their time.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P1
- **Estimated Scope**: Medium
