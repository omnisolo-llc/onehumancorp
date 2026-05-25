# Title: Automated Shipping Rates and Label Generation

## Problem Statement
Business owners selling physical goods waste hours manually entering addresses into carrier websites to buy shipping labels. They need real-time shipping rates at checkout and one-click label printing.

## Research Report
*   **Tool Candidates**: Shippo API, EasyPost API.
*   **Evaluation**: Both Shippo and EasyPost aggregate dozens of carriers (USPS, UPS, FedEx, DHL). Shippo has slightly better out-of-the-box rates for small users.
*   **Ease of Use**: Business owner sets up box sizes. The system automatically buys and downloads the PDF label when an order is packed.
*   **Pricing**: Usually a few cents per label plus the actual postage cost.
*   **Modes**: Cloud (API keys managed by OHC). Standalone (user needs their own Shippo/EasyPost account).

## Design Doc
*   **Integration Trigger**: An order containing physical products is marked as "Packed".
*   **Action**: OHC requests a shipping label from the API using the customer's address and the predefined box size, then saves the tracking number and PDF.
*   **User Interface**: A "Print Label" button on the order details page and automatic tracking emails sent to the customer.

## Implementation Prompt
Integrate a shipping API to automatically calculate shipping rates during checkout and generate shipping labels for physical orders. Acceptance criteria: user can configure default package dimensions, checkout calculates accurate rates, and the user can generate and download a PDF shipping label for an order.

## Priority
P2

## Estimated Scope
Large
