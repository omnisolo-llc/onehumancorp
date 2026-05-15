# Sync with Apple Calendar (iCloud)

## 1. Problem Statement
Many solopreneurs and creatives use iPhones and Macs, relying entirely on Apple Calendar. If their business app doesn't sync with their personal Apple Calendar, they have to manually copy appointments over, which is tedious and error-prone. This demographic is highly mobile, often checking their schedule directly from their Apple Watch or iPhone lock screen. Without sync, the OHC platform feels disconnected from their daily reality.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Apple Calendar integration is often requested by the creative and mobile-first demographic. The integration typically relies on CalDAV or app-specific passwords, as Apple doesn't offer a modern OAuth API for calendar access in the same way Google or Microsoft do. This makes the setup slightly more technical for the user. However, the value of having their business schedule on their iPhone automatically is a massive quality-of-life improvement. It's viable for both Cloud and Standalone setups. The lack of webhooks means robust polling and diffing logic is required.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Free feature built into standard iCloud accounts.
- **Deployment Compatibility:** Functional in both environments, though Standalone may require local polling rather than webhooks.

## 3. Design Document
The setup process will involve a guided tutorial teaching the user how to generate an 'App-Specific Password' from their Apple ID account. The UI will provide clear, step-by-step screenshots. Once authenticated, OHC will sync appointments, displaying iCloud events as busy times and pushing new bookings to their Apple devices. A manual 'Sync Now' button should be provided in case the background polling is delayed.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Design a highly visual, step-by-step onboarding flow that guides non-technical users through the process of generating an Apple app-specific password, as this is a major friction point. Implement the calendar view integration to display synced iCloud events. Provide a manual sync trigger for immediate updates.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P2
- **Estimated Scope**: Medium
