# Integrate Instagram Direct Messages

## 1. Problem Statement
Many businesses operate entirely out of their Instagram DMs, receiving orders, support questions, and collaboration requests all in one place. Managing this on a tiny mobile screen leads to missed messages and disorganized customer relationships. Instagram does not natively provide a good CRM experience for tracking which customer has paid, which order is pending, or who needs a follow-up. A business owner often has to scroll through hundreds of messages to find an address sent days ago.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Instagram Direct (via Messenger API) is a critical channel. For many modern brands, it is the primary support and sales channel. Integrating this allows business owners to have a larger, more organized view of their DMs, potentially assigning them to different team members if they have staff. The OAuth process requires an Instagram Professional account linked to a Facebook Page, which can be a friction point, so clear, step-by-step guidance is necessary. The integration provides massive value by saving time and reducing errors. This works well in both Cloud (webhook driven) and Standalone modes. Meta's API limits must be carefully respected, particularly around the 24-hour response window constraint.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Freemium tier available; standard API usage rates apply thereafter.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
A 'Connect Instagram' option will be added to the Communications dashboard. A wizard will guide the user to convert their account to professional if they haven't already, then link their Facebook page. DMs will flow into the OHC Inbox with a distinct Instagram icon. Users can send text, emojis, and images directly from OHC back to the customer's Instagram app. The UI must clearly indicate if the 24-hour standard messaging window is closing, ensuring the business owner knows they need to reply urgently.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Implement the user onboarding wizard for connecting Instagram, specifically handling the edge cases where a user might not have a connected Facebook Page yet, using friendly, educational UI steps. Add message rendering for Instagram DMs in the Inbox, supporting text and image attachments. Build a visual countdown or alert system for the 24-hour reply window to prevent user frustration.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P0
- **Estimated Scope**: Medium
