# Automated Shipping Labels via Shippo

## Problem Statement
Calculating shipping rates manually and visiting the post office to purchase labels wastes significant time for small business owners shipping physical goods.

## Research Report
Shippo provides an excellent API for multi-carrier shipping rate calculation and label generation. It abstracts the complexity of dealing with individual carrier APIs (USPS, UPS, FedEx, DHL, etc.).
*   **Ease of use (end user):** High. The user enters dimensions, and Shippo handles the rate comparisons.
*   **Pricing:** Pay As You Go tier ($0.05 per label) is ideal for small businesses.
*   **Reputation:** Well-regarded developer API and reliable service.

## Design Doc
OHC will integrate Shippo for order fulfillment.
1.  **Trigger:** Business owner views a pending order and clicks "Generate Shipping Label".
2.  **Action:** OHC prompts for package dimensions and weight, calls Shippo API to get rates, and allows the user to purchase and print the label.
3.  **User Sees:** A simple interface to select the best shipping rate, followed by a printable PDF label and automatic tracking number assignment.

## Implementation Prompt
Implement automated shipping label generation using the Shippo API.
*   Add a "Fulfillment" section to the order details page.
*   Create a form for the business owner to input package dimensions and weight.
*   Display real-time shipping rates from Shippo for the owner to choose from.
*   Generate the selected shipping label (PDF) and update the order with the tracking number.
*   Acceptance Criteria: An owner can input package details on an order, view rates, purchase a label (simulated), and download the PDF.

## Priority
P2

## Estimated Scope
Medium
