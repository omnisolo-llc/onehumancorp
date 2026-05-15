# Accept Payments via Alipay

## 1. Problem Statement
Businesses operating internationally or serving Chinese tourists and expatriates lose significant sales if they cannot accept Alipay, as it is the preferred, and sometimes only, digital wallet for hundreds of millions of consumers. A physical retail store in a tourist-heavy area or an e-commerce site shipping globally will face massive cart abandonment if they only offer Western credit card options.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Alipay is a dominant payment method globally, not just in China. Integrating Alipay allows merchants to present a QR code or checkout link that customers can scan with their Alipay app. The fees are generally competitive. The primary advantage is capturing a demographic that might otherwise abandon their purchase. The integration requires specific merchant onboarding, which can be rigorous. Works in both Cloud and Standalone environments. Refunds and dispute handling must be carefully mapped to the OHC billing dashboard.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Standard transaction fee basis (e.g., 2.9% + 30¢). No monthly fixed cost.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
In the Payment Settings, a new 'Enable Alipay' toggle will be available. If the business is eligible, they can walk through the onboarding. When creating an invoice or checkout link in OHC, the business owner can choose to include Alipay as an option. The customer will see a clean Alipay QR code or redirection button when they go to pay the bill. The OHC dashboard will clearly label Alipay transactions with their respective currency conversions.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Implement the Alipay onboarding UI in the payment settings, ensuring the eligibility requirements are clearly stated upfront. Add the Alipay visual option to the customer-facing invoice checkout screen, ensuring the QR code generation process is smooth and visually clear. Build a clean transaction history view for these specific alternative payments.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P2
- **Estimated Scope**: Medium
