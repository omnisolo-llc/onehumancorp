## [Shipping] ShipEngine Integration
**Title**: Integrate ShipEngine for Multi-Carrier Label Generation
**Problem Statement**: Small e-commerce sellers spend too much time manually copying addresses into carrier websites to print shipping labels. They need an automated way to get rates and print labels from within OHC.
**Research Report**:
- **Tool**: ShipEngine
- **Target Persona**: Maya (Home Baker shipping nationwide), Boutique Owners
- **Advantages**: Connects to dozens of carriers (USPS, UPS, FedEx) through a single API.
- **Risks**: API can be complex due to the sheer number of shipping options.
- **Pricing**: Pay-as-you-go based on labels printed (e.g., $0.05 per label).
- **Compatibility**: Cloud (API Keys managed per tenant). Standalone (API Keys).
**Design Doc**:
- User enters their carrier accounts (or uses default ShipEngine rates) in OHC.
- When an order is ready, user clicks "Generate Label".
- ShipEngine API returns the label PDF and tracking number.
- OHC saves the tracking number and notifies the customer.
**Implementation Prompt**: Build the ShipEngine integration. Implement endpoints to fetch shipping rates based on package dimensions/weight, generate shipping labels, and track package statuses via webhooks.
**Priority**: P1
**Estimated Scope**: Large
