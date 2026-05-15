# Auto-Generate Microsoft Teams Meeting Links

## 1. Problem Statement
For B2B consultants and service providers, manually creating a Microsoft Teams meeting and pasting the link into every new calendar invite is a tedious, repetitive chore that looks unprofessional if forgotten. Clients expect a frictionless joining experience, and scrambling to send a link two minutes before the meeting starts damages the business's credibility.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Microsoft Teams is heavily used in the corporate and B2B sector. When a customer books an online consultation through OHC, automatically generating a unique Teams meeting link and embedding it in the confirmation email saves the business owner time and provides a frictionless experience for the client. The integration uses the Microsoft Graph API. It is highly valuable for professional services. Works in Cloud and Standalone modes. Token refresh logic is the trickiest part of this integration.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Requires the business owner to hold a paid subscription with the provider (e.g. $15/mo).
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
In the Service creation menu, under 'Location', the user can select 'Online Meeting (Microsoft Teams)'. Once they have linked their Microsoft account, every time this service is booked, a unique Teams link is generated. Both the business owner and the customer will see a prominent 'Join Meeting' button on their respective appointment details pages. The interface will also allow copying the link manually if needed.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Update the Service creation flow to include 'Microsoft Teams' as a dynamic location option. Design the appointment details view to prominently feature a friendly, clickable 'Join Meeting' button that handles the transition to the Teams app smoothly. Add fallback text instructions if the user doesn't have the Teams client installed.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P1
- **Estimated Scope**: Small
