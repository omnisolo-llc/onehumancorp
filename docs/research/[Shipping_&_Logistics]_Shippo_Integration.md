# [Shipping & Logistics] Shippo Integration

**Problem Statement**: Online sellers spend hours manually copying addresses to carrier websites to buy shipping labels. They need to generate shipping labels directly from their order dashboard to save time.

**Research Report**:
- **Target Persona**: E-commerce and physical product sellers.
- **Ease of Use**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) into one API.
- **Pricing**: Pay-as-you-go per label fee (e.g., 5 cents) + postage cost. Very friendly for small businesses.
- **Reputation**: Highly regarded for easy integration and good rates.
- **Cloud/Standalone**: API-based, works well in both environments.

**Design Doc**:
- **Trigger**: Business owner marks an order as "Ready to Ship".
- **Action**: OHC requests a shipping label from Shippo and provides it for download.
- **User View**: A "Buy Shipping Label" button appears on order details. Owner confirms package weight/size and clicks to generate a printable PDF label.

**Implementation Prompt**: Integrate Shippo to allow merchants to generate and download shipping labels directly from the order management screen in OHC.

**Priority**: P1
**Estimated Scope**: Large
