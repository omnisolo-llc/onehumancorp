## [Shipping] Issue Brief

**Title**: Scout 🔍: Integrate EasyPost for Streamlined Shipping
**Problem Statement**:
E-commerce businesses spend too much time calculating shipping rates, printing labels, and tracking packages manually across different carriers.
**Research Report**:
- **Tool**: EasyPost API
- **Evaluation**: EasyPost provides a unified API for multiple carriers (USPS, UPS, FedEx, etc.). Integrating it allows OHC to offer real-time rates and automated label generation.
- **Ease of Use**: Users enter their EasyPost API key or connect their carrier accounts through the EasyPost dashboard.
- **Pricing**: Priced per label generated. Some carriers have negotiated rates available.
- **Cloud vs. Standalone**: Works in both modes.
**Design Doc**:
- User sets up EasyPost API keys.
- During checkout, EasyPost calculates real-time shipping rates based on package dimensions.
- The 'Operations' agent can automatically generate labels when orders are fulfilled.
**Implementation Prompt**:
Integrate the EasyPost API. Implement real-time rate calculation during checkout. Provide a UI for business owners to generate and print shipping labels directly from the order details page.
**Priority**: P1
**Estimated Scope**: Large
