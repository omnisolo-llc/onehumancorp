# Scout: Shipping & Logistics (AfterShip)

## Title
Multi-Carrier Label Generation & Tracking 📦 (AfterShip Shipping API Integration)

## Problem Statement
Small business owners selling physical goods, like Maya (the baker) and Priya (the boutique owner), spend too much time manually copying addresses into carrier websites (USPS, UPS, FedEx) to buy shipping labels. They also struggle to provide accurate tracking information to their customers. A unified shipping API is needed to calculate rates, buy labels, and automate tracking updates from within the OHC platform.

## Research Report
- **Goal**: Evaluate AfterShip Shipping API (formerly Postmen) as the multi-carrier shipping engine for OHC's Operations Department.
- **Features evaluated**:
  - **Label Generation**: API to create shipping labels for 100+ carriers globally.
  - **Rate Comparison**: Real-time rate calculation to find the cheapest shipping option.
  - **Tracking Integration**: Automatically sync tracking numbers to AfterShip Tracking for customer notifications.
  - **Address Validation**: Reduce delivery failures by validating shipping addresses at checkout.
- **Benefits for OHC users (Non-technical)**:
  - No more manual entry; labels are generated with one click from the order screen.
  - Professional "Track Your Order" pages for their customers.
  - Access to discounted shipping rates.
- **Integration Risks**:
  - AfterShip Shipping API (v3) requires users to connect their own carrier accounts or use AfterShip's discounted rates.
  - International shipping (customs forms) adds complexity to the API mapping.
- **Pricing**: "Pay-as-you-go" or tiered monthly plans based on label volume.
- **Cloud vs Standalone**: Native support for Cloud mode. For Standalone, the local OHC backend can trigger label creation via the AfterShip API, allowing the user to print directly from their desktop.

### Persona Pain Point Summary
| Persona | Pain Point | Solution via AfterShip Integration |
|---------|------------|-----------------------------------|
| **Maya (Baker)** | Worried about her custom cake decorations being damaged or lost during delivery. | Automated tracking notifications keep her customers updated, reducing "where is my order?" inquiries. |
| **Priya (Boutique)**| Spends hours every Sunday printing labels for her weekend orders. | "Batch Labeling" feature allows her to generate all 50 labels for her weekend sales in seconds. |

## Design Doc
- **Component**: `UnifiedShippingService`
- **Responsibilities**:
  - Map OHC "Order" data to AfterShip Shipping API requests.
  - Store and manage carrier credentials for each OHC tenant.
  - Provide a "Print Label" UI that displays the PDF/ZPL label from the API.
  - Register tracking numbers with the AfterShip Tracking API.
- **User Experience**:
  - An "Orders" dashboard where owners see "Ship this order."
  - A selection of shipping speeds/prices from different carriers.

## Implementation Prompt
"Integrate the AfterShip Shipping API in `src/server/integrations/aftership/`. Implement a service that can calculate shipping rates for a given weight and destination, and generate a printable shipping label. Ensure the service handles webhook notifications for tracking updates. Acceptance criteria: A merchant can view an order, see various shipping rates, select one, and generate a valid PDF shipping label."

## Priority
P1

## Estimated Scope
Large
