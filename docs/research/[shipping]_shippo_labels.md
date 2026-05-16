# Automated Shipping Label Generation with Shippo

**Title**: Automated Shipping Label Generation with Shippo
**Problem Statement**: E-commerce sellers waste hours manually calculating shipping rates at the post office and writing labels by hand. They need an automated way to generate shipping labels directly from their orders.

**Research Report**:
- Shippo provides a unified API connecting to dozens of global carriers (USPS, UPS, FedEx, DHL, etc.).
- **Ease of Use**: OHC can provide a "Buy Label" button directly on the order details page, abstracting the carrier integrations.
- **Pricing**: Pay-as-you-go model (per label fee), which is very SMB-friendly without monthly commitments.
- **Reputation**: Reliable and widely used in the e-commerce space.
- **Cloud vs Standalone**: Compatible with both modes.
- **Key Advantages**: Broad carrier coverage, negotiated discount rates available out-of-the-box.
- **Key Risks**: International customs forms can still be complex for users to fill out accurately.

**Design Doc**:
- On the "Orders" page, an unfulfilled order displays a "Create Shipping Label" button.
- The system automatically pulls the package dimensions and weight (if saved) and displays real-time carrier rates.
- The user selects a rate, purchases the label, and the system generates a printable PDF and automatically emails the tracking link to the customer.

**Implementation Prompt**: Integrate Shippo to allow users to compare real-time shipping rates, purchase labels directly from their order dashboard, and automatically update customers with tracking information.

**Priority**: P1
**Estimated Scope**: Large
