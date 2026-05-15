# Sync Sales with Square Point of Sale

## 1. Problem Statement
A huge portion of retail and food small businesses use Square for their in-store registers. If OHC handles their online bookings or digital invoices, but the data doesn't sync with Square, their financial reporting is split in half and impossible to manage. Come tax season, the owner has to export CSVs from two different systems and merge them manually, which is a nightmare.

This issue represents a significant friction point for our core demographic. The inability to seamlessly manage this aspect of their business leads to tangible revenue loss and operational inefficiency. Small business owners are not IT administrators; they expect their tools to communicate flawlessly without requiring manual intervention or complex configuration screens.

## 2. Research Report
Square is incredibly dominant in the SMB POS space. A deep integration where OHC can either push invoices to Square or pull sales data from Square is highly requested. The API is robust and developer-friendly. The value is unifying financial reporting for the business owner. The risk is handling duplicate customer records gracefully. Works in Cloud and Standalone environments. Syncing inventory quantities alongside sales is a potential 'phase 2' that should be considered in the database design.

### Market Validation
Our market analysis confirms that competitors either lack this integration entirely or place it behind expensive enterprise tiers. By offering this seamlessly within OHC, we create a strong competitive moat. It directly appeals to businesses scaling past their first phase of growth who are beginning to feel the pain of fragmented systems.

### Technical Feasibility Assessment
The third-party APIs required to support this are generally stable and well-documented. We anticipate standard challenges regarding rate limiting and token expiration, which must be handled gracefully by our backend worker queues. The implementation will rely on our standard asynchronous event processing model to ensure the core application remains highly responsive.


### Pricing & Deployment
- **Pricing Estimate:** Standard transaction fee basis (e.g., 2.9% + 30¢). No monthly fixed cost.
- **Deployment Compatibility:** Fully functional in both Cloud (multi-tenant) and Standalone (local instance) modes.

## 3. Design Document
The user will connect their Square account via a simple login button. They will then choose their sync preference: 'Import my Square sales into OHC' or 'Send my OHC invoices to Square'. The dashboard will then reflect a combined view of their revenue, clearly marking which transactions originated from the physical Square register. A sync conflict resolution UI will gently prompt the user if duplicate customers are detected.

### User Experience Considerations
The 'Grandmother Test' is critical here. The connection flow must avoid technical jargon. We cannot ask users to configure 'webhooks' or 'callback URLs'. The entire process must be a simple OAuth click-through or a very well-guided wizard with clear screenshots and tooltips.

## 4. Implementation Prompt
Design the Square connection flow, specifically focusing on the user decision of how data should flow (import vs. export) using plain language examples. Build the unified revenue dashboard view that clearly distinguishes between online and physical sales. Design a simple conflict resolution modal for merged customer profiles.

### Acceptance Criteria
1. The user can initiate and complete the connection flow in under 3 minutes.
2. The unified dashboard accurately reflects the new data without requiring a manual page refresh.
3. If the connection fails or the token expires, a clear, actionable alert is displayed to the user.
4. The feature passes all Playwright E2E tests for the core happy path.

## 5. Metadata
- **Priority**: P1
- **Estimated Scope**: Large
