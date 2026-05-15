# [Shipping & Logistics] Shippo Integration

**Title**: Shippo for Automated Label Generation
**Problem Statement**: Manually calculating shipping rates and buying labels at the post office is incredibly time-consuming for e-commerce business owners.
**Research Report**:
- **Target Persona**: E-commerce sellers shipping physical products domestically and internationally.
- **Evaluation**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) and provides discounted rates. The UI is straightforward.
- **Ease of Use**: High. Requires basic package dimensions.
- **Pricing**: Pay-as-you-go pricing (cents per label) is perfect for small volume.
- **Key Risks**: Carrier API outages, accurate weight/dimension input is required from the user to avoid surcharge adjustments.
- **Compatibility**: Works flawlessly in Cloud. Standalone requires secure API key storage.
**Design Doc**: Within an OHC order, users click "Generate Label". OHC fetches the cheapest rate from Shippo and provides a printable PDF label.
**Implementation Prompt**: Add a "Buy Label" button on the order details page that fetches rates and generates a shipping label. Acceptance criteria: user can view rates and download a label PDF.
**Priority**: P1
**Estimated Scope**: Large
