# Automate Fulfillment with ShipStation

## 1. Problem Statement
E-commerce and physical product sellers spend hours copying and pasting customer addresses into shipping carrier websites to print labels. It's tedious, error-prone, and scales terribly when they have a busy day. Shipping a single package manually takes minutes; shipping fifty takes hours. Typographical errors in addresses lead to lost packages and angry customers.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
ShipStation is the industry standard for multi-carrier shipping for SMBs. Integrating ShipStation means whenever an order is marked 'Paid' in OHC, it instantly appears in ShipStation ready for a label to be printed. ShipStation handles the carrier rate shopping. This saves business owners massive amounts of time. The integration is well-documented and highly reliable. Applicable to Cloud and Standalone modes. Webhooks notify OHC when the label is printed so tracking info can be passed to the customer.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Freemium tier available; standard API usage rates apply thereafter.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
Users connect ShipStation via API keys in the Shipping settings. Once connected, the OHC order dashboard will have a new status indicator showing if an order has been 'Sent to Fulfillment'. When ShipStation generates a tracking number, it will seamlessly flow back into OHC, updating the order status to 'Shipped' and automatically notifying the customer via email.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Build the integration settings for ShipStation. Enhance the Order Management dashboard to display fulfillment statuses clearly. Implement the UI logic so that when a tracking number is received back from ShipStation, it is prominently displayed on the order details page and triggers the customer notification flow seamlessly.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P1
- **Estimated Scope**: Medium
