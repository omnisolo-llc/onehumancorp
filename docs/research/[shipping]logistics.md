# [Shipping & Logistics] Automated Logistics

## Title
Implement Automated Shipping Rates and Label Generation

## Problem Statement
Small e-commerce businesses waste hours every week manually copying order details into carrier websites (like USPS, FedEx, or local couriers) to check shipping rates and print labels. This manual process is error-prone, leads to overcharging or undercharging customers for shipping, and slows down fulfillment. They need a system that automatically calculates the correct shipping cost at checkout and lets them print the shipping label with one click.

## Research Report
### Shippo Evaluation
- **Overview:** Shippo (Popout, Inc.) is an American e-commerce software company that provides a multi-carrier shipping API and web application.
- **Key Benefits for SMBs:**
  - **Multi-Carrier Access:** Connects to dozens of global carriers through a single integration.
  - **Discounted Rates:** Often provides pre-negotiated discounted rates for USPS and DHL, saving the business money immediately.
  - **Automation:** Automates rate calculation, label generation, and return processing.
- **Challenges/Risks:**
  - **Physical Hardware:** The business owner needs a reliable label printer setup; printer configuration issues are common non-technical support headaches.
  - **International Complexity:** Handling customs forms and duties via API adds complexity to the integration.
- **Ease of Use for Non-Technical Users:** Very high. The user just clicks "Fulfill Order" and a label pops out. All the complex API negotiation happens in the background.
- **Cloud vs. Standalone:**
  - **Cloud:** Easily managed. OHC server handles API calls to Shippo.
  - **Standalone:** Highly feasible. The local app can make API calls directly to Shippo to generate labels. It might even have an advantage in communicating directly with local USB label printers.
- **Pricing Estimate:** Shippo offers a pay-as-you-go tier (e.g., $0.05 per label) and flat-rate monthly subscriptions starting around $10/month.

## Design Doc
- **Integration Trigger:** A "Fulfillment" settings page to connect a Shippo account and configure default package sizes.
- **Actions Taken:**
  - At checkout, OHC sends cart weight/dimensions to Shippo to retrieve real-time shipping rates to display to the customer.
  - In the order management view, clicking "Fulfill" generates a label PDF via Shippo and updates the order status to "Shipped" with a tracking number.
- **User Experience:**
  - Business Owner: Sees a list of pending orders. Clicks "Buy Label" on an order, confirms package weight, and the label PDF downloads automatically.
  - Customer: Sees accurate shipping costs at checkout and receives an automated tracking link when the order ships.
  - Simple Mode: Standard shipping options only. Advanced Mode: Custom package sizes, international customs form generation.

## Implementation Prompt
Integrate Shippo to automate the fulfillment process for e-commerce orders. Implement two main features: 1) Real-time shipping rate calculation at checkout based on cart contents, and 2) One-click shipping label generation from the OHC order management dashboard. Ensure the integration automatically saves the tracking number to the order record and provides the business owner with a printable PDF label. Hide the API complexity behind a simple setup screen where they authorize their Shippo account.

## Priority
P1

## Estimated Scope
Medium