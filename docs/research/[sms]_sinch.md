# High-Volume SMS Marketing via Sinch

## 1. Problem Statement
Business owners want to run quick 'flash sales' or send urgent updates to their entire customer base, but email open rates are too low and slow. They need a way to broadcast text messages quickly and reliably. However, they lack the tools to segment their audience properly, often leading to blasting irrelevant messages and causing mass unsubscribes.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Sinch is a powerful provider for bulk SMS and conversational messaging. Integrating Sinch allows OHC to offer a 'Broadcast SMS' feature. The primary benefit is instant reach and incredibly high open rates for marketing campaigns. The major risk is spam compliance and the cost of bulk SMS, which must be transparently communicated to the user before they hit send. Cloud and Standalone compatible. A robust opt-out parsing mechanism is absolutely mandatory.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Pay-as-you-go per message segment sent/received. Approximately $0.0075/msg.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
A new 'New SMS Broadcast' button will be added to the Marketing section. The user can select an audience segment, draft a short text message, and see an immediate cost estimate before sending. A preview of how the message looks on a standard mobile screen will be displayed alongside the draft. Delivery reports and click-through rates (if links are included) will be visible post-send.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Design the SMS Broadcast creation tool, ensuring the mobile preview updates in real-time as the user types. Implement a clear, upfront cost estimation component that calculates the price of the broadcast based on the segment size before the user confirms the send. Build a post-campaign analytics view.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P2
- **Estimated Scope**: Large
