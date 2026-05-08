## [Shipping & Logistics] Issue Brief: EasyPost Integration for Unified Carrier Management

**Title**: Scout 🔍: Integrate EasyPost for Seamless Label Generation and Tracking
**Problem Statement**:
E-commerce businesses spend too much time manually calculating shipping rates across different carriers (USPS, UPS, FedEx) and copying addresses to print labels. They need an automated way to get the best rates and print labels directly from their orders.
**Research Report**:
- **Tool**: EasyPost
- **Evaluation**: A modern API that aggregates hundreds of carriers. It handles rate calculation, label purchasing, and tracking via a single interface.
- **Ease of Use**: Very easy. The user connects their carrier accounts once (or uses EasyPost's default rates).
- **Pricing**: Pay-as-you-go per package, or custom enterprise pricing.
- **Cloud vs. Standalone**: Works in both via standard API calls.
**Design Doc**:
- User configures package dimensions and weights in their product catalog.
- At checkout, OHC calls EasyPost to fetch real-time shipping rates.
- In the order management UI, the user clicks "Generate Label", which purchases the label via EasyPost.
- EasyPost webhooks update the order status with tracking information.
**Implementation Prompt**:
Integrate the EasyPost API. Build a UI for users to manage shipping settings (package sizes, origin address, connected carriers). Implement real-time rate calculation at checkout. Create an interface to purchase and print labels from the order details page, and handle tracking webhooks.
**Priority**: P1
**Estimated Scope**: Medium
