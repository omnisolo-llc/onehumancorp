# Title: Automated Shipping Rates & Labels via EasyPost

## Problem Statement
Physical product sellers like Maya (The Home Baker) or artists shipping prints struggle with calculating shipping costs at checkout. They often undercharge and lose money, or overcharge and lose sales. Furthermore, manually buying and copying tracking numbers for every order is tedious.

## Research Report
**Findings & Evaluation:**
- **EasyPost:** A unified API for shipping. It aggregates 100+ carriers (USPS, UPS, FedEx, DHL, etc.) behind a single REST API.
- **Alternatives evaluated:** Shippo, Sendle. EasyPost offers the most robust API and highest reliability, especially for real-time rating at checkout.
- **Ease of Use:** Business owners input their package dimensions/weights on the product page. EasyPost handles the rest invisibly.
- **Pricing:** Very developer-friendly; pay-per-label model fits well.

## Design Doc
**Integration with OHC:**
During the checkout flow on the OHC storefront, the backend makes a synchronous call to the EasyPost Rating API using the cart contents (weights/dimensions) and the customer's shipping address. Real-time shipping options (Standard, Expedited) are displayed to the customer.
When the order is paid, the Operations Agent ("The Manager") uses the EasyPost API to purchase the label. The label PDF is stored in GCS and made available in the OHC app for the business owner to print.
EasyPost webhooks update the tracking status (In Transit, Delivered), triggering the Customer Success Agent to automatically email the customer.

## Implementation Prompt
**User-Facing Outcome & Acceptance Criteria:**
- Business owners can input weight and dimensions for physical products.
- Customers see accurate, real-time shipping costs at checkout based on their address.
- The business owner can generate and print a shipping label directly from the OHC app with one tap.
- Customers automatically receive email updates with tracking numbers when the item ships.

## Priority
P1

## Estimated Scope
Large
