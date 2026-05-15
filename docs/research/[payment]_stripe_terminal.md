# In-Person Payments with Stripe Terminal

## 1. Problem Statement
Many OHC users run hybrid businesses (e.g., a salon or a pop-up shop) where they need to take physical credit cards. Currently, they have to use a separate clunky card reader system and manually type the sales back into OHC at the end of the day. This manual entry leads to accounting mistakes, missed revenue tracking, and a disjointed customer experience where the receipt doesn't match the booking software.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Stripe Terminal provides physical card readers that can be controlled via web or mobile apps. Integrating this means a user can click 'Charge' in OHC, and their physical Stripe reader lights up asking the customer to tap their card. This unifies their online and offline sales data flawlessly. The hardware cost is reasonable for SMBs. The complexity lies in managing the connection to the physical device. Works well in Cloud mode; Standalone may require local network reader configurations. It relies heavily on WebSockets or local polling for device status.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Standard transaction fee basis (e.g., 2.9% + 30¢). No monthly fixed cost.
- **Deployment Compatibility:** Functional in Cloud mode. Standalone mode requires advanced local network configuration for the hardware reader.

## 3. Design Document
Users will have a 'Hardware' section in their settings to pair a new Stripe Terminal reader. When creating a charge or closing an invoice, a new 'Tap, Insert, or Swipe' button will appear. Clicking it will show a waiting animation in OHC while the physical reader prompts the customer. Success or failure will instantly reflect in the OHC interface, and a unified digital receipt can be emailed.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Create the hardware pairing interface in settings, providing clear, visual instructions on how to put the physical reader into pairing mode. Design the checkout overlay that appears when waiting for the customer to tap their card on the reader, ensuring it handles timeouts or declined cards gracefully with clear retry instructions.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P1
- **Estimated Scope**: Large
