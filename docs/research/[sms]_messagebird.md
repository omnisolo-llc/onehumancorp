# Global SMS Notifications with MessageBird

## 1. Problem Statement
For non-technical users and customers in regions where email isn't checked frequently, SMS is the only reliable way to send appointment reminders or order updates. Without it, no-show rates for appointments skyrocket. Email fatigue is real, and a simple text message often guarantees a 95%+ open rate within minutes, which is crucial for time-sensitive business operations.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
MessageBird (now part of Bird) provides excellent global SMS coverage, often better priced for European and Asian markets compared to US-centric providers. Integrating this allows business owners to seamlessly send automated text reminders. The value is a direct reduction in lost revenue from missed appointments. Compliance (opt-outs) and varying global regulations are the main risks to manage for the user. Cloud and Standalone compatible. Need to gracefully handle formatting of international phone numbers.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Pay-as-you-go per message segment sent/received. Approximately $0.0075/msg.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
Users configure their SMS provider in the Communications settings. Once enabled, they can toggle which automated messages should be sent via SMS (e.g., 'Appointment Reminder 24h before', 'Order Ready for Pickup'). The system will automatically append necessary opt-out instructions to stay compliant, keeping the business owner safe. A character counter will help users avoid accidentally sending two-part (double cost) text messages.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Create the SMS configuration dashboard where users can toggle various automated text notifications. Design a clear, visual warning system that informs users if they attempt to send an SMS to a customer who hasn't opted in or who lacks a valid phone number format. Include a character counter in the template editor.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P1
- **Estimated Scope**: Medium
