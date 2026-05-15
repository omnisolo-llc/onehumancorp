# Deep Integration with ActiveCampaign Automations

## 1. Problem Statement
Advanced small businesses use ActiveCampaign for complex marketing automations (like abandoned cart emails or onboarding sequences). If OHC doesn't tell ActiveCampaign when a customer makes a purchase or books a service, those expensive automations are useless. Without this real-time event data, the business cannot personalize their marketing, sending irrelevant generic emails instead of highly targeted offers based on recent activity.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
ActiveCampaign is a powerful CRM and marketing automation platform. While more expensive, it caters to growing SMBs who need more than just simple newsletters. Integrating OHC to send not just contacts, but specific 'events' (e.g., 'Appointment Booked', 'Invoice Paid') unlocks immense value for these users. The integration risk is the complexity of mapping these events clearly without confusing the user. Applicable to Cloud and Standalone modes. Custom event creation via their API is very flexible and robust.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Monthly subscription starting at $15/mo based on subscriber count.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
The integration page will allow connecting ActiveCampaign via their connection credentials. Once linked, the user will see a list of OHC triggers (e.g., 'New Customer Added', 'Purchase Completed'). They can toggle which of these events they want to silently pass over to ActiveCampaign, empowering them to kick off advanced external marketing flows. A 'Test Connection' button will allow them to fire a dummy event to verify their ActiveCampaign setup.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Design an intuitive 'Event Sync' dashboard where users can easily toggle which business activities in OHC should be sent to ActiveCampaign. The UI should explain in plain language what sending these events allows them to do in their marketing platform. Implement a testing utility so users can verify their automation triggers work before going live.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P2
- **Estimated Scope**: Medium
