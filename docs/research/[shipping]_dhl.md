# Direct DHL Express International Shipping Rates

## 1. Problem Statement
When small businesses try to sell internationally, calculating the correct shipping cost at checkout is incredibly difficult. If they charge too little, they lose money on shipping; if they charge too much, the customer abandons the cart. Many SMBs avoid international sales entirely out of fear of getting the shipping calculations wrong.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
DHL Express is the premier carrier for international SMB shipping. Integrating their rating API allows OHC to display real-time, accurate shipping costs to international customers during checkout based on the package weight and destination. This removes the guesswork for the business owner and builds trust with the buyer. The integration requires careful handling of dimensional weight and customs documentation. Works in Cloud and Standalone modes. Fallback rates must be established if the DHL API times out.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Freemium tier available; standard API usage rates apply thereafter.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
In the Store setup, users can enable 'Real-time DHL Rates' and input their DHL account credentials. When a customer from another country adds items to their cart and enters their address, the checkout page will dynamically show the exact DHL shipping cost. The business owner will see these details on the order receipt, along with any necessary customs forms generated dynamically.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Design the configuration panel for enabling real-time DHL rates, including fields for default package dimensions. Implement the dynamic checkout UI that clearly presents the fetched shipping options and costs to the customer in a seamless, fast-loading manner. Provide a fallback message if rates cannot be fetched.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P2
- **Estimated Scope**: Medium
