# Sync Customer Lists with MailerLite

## 1. Problem Statement
Business owners collect customer emails in OHC but want to send beautiful, automated newsletters using a dedicated tool like MailerLite. Exporting and importing CSV files manually every week is a huge waste of time and often gets forgotten. When a customer signs up for a service but never receives the welcome email series because the CSV wasn't uploaded, the business loses a vital engagement opportunity.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
MailerLite is a very popular email marketing tool for SMBs due to its generous free tier and ease of use. Integrating it allows OHC to automatically push new customer contacts directly into specific MailerLite groups. The API is straightforward and reliable. The primary benefit is automating audience growth. It's low risk and high reward, especially for retail and content-driven businesses. Works perfectly in Cloud and Standalone environments. The integration should also handle unsubscribe events so OHC stops sending marketing emails if the user opts out via MailerLite.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Monthly subscription starting at $15/mo based on subscriber count.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
In the Marketing integrations page, users can click 'Connect MailerLite'. They will be prompted to paste their connection key (API key). The UI will explain exactly where to find this key with a small helper image. Once connected, users can map OHC customer segments (e.g., 'All Customers', 'VIPs') to specific MailerLite groups, ensuring every new customer is automatically subscribed to the right newsletter.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Build the connection settings UI for MailerLite, including the helper tooltips that show the user exactly where to find their connection key. Create the mapping interface that lets users visually link their OHC customer lists to their MailerLite groups. Ensure the UI clearly shows when the last successful sync occurred.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P1
- **Estimated Scope**: Small
