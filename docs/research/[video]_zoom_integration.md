# Auto-Generate Zoom Meeting Links

## 1. Problem Statement
Zoom is the most common video conferencing tool. Business owners have to manually create meetings and email links to clients when a consultation is booked.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Zoom API allows automatic meeting creation. A seamless integration ensures meeting details are attached to the OHC booking and sent in the confirmation email.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Requires the business owner to hold a paid subscription with the provider (e.g. $15/mo).
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
Select 'Zoom' as location during service creation. Automatically generate links. Show 'Join Meeting' button on booking details.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Implement Zoom OAuth and meeting generation API. Update the service location dropdown and customer booking views.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P0
- **Estimated Scope**: Medium
