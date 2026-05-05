# EasyPost Integration for OHC

## Problem Statement
Small business owners selling physical products (like Priya or Maya) need a simple way to calculate shipping costs, print labels, and track packages. Managing shipping manually across multiple carriers (USPS, FedEx, UPS) is confusing and error-prone. They need an automated way to handle fulfillment without leaving the OHC platform.

## Research Report
- **Features & API Suitability**: EasyPost offers a unified API for 100+ carriers. Features include rating (price calculation), label purchasing/generation, tracking, and address verification.
- **Pricing**: Free tier (Developer Plan) covers up to 120,000 shipments/year. Labels are paid directly to carriers.
- **Ease of Use for Non-Technical Users**: High once configured. The business owner simply clicks "Buy Label" in OHC.
- **Cloud vs. Standalone**: Works perfectly in both via API keys.
- **Advantages**: Abstracted carrier complexity; built-in address verification reduces shipping errors.
- **Risks**: Carrier API downtimes.

## Design Doc
- **Integration Point**: "The Manager" (Operations).
- **Trigger**: Customer places an order requiring physical shipping.
- **Action**: OHC calls EasyPost to fetch live shipping rates during checkout. Upon order confirmation, the business owner clicks "Fulfill", which uses EasyPost to purchase and generate a PDF label. Tracking numbers are auto-synced.
- **User View**: A "Shipping & Fulfillment" section on the order detail page. A single button to generate and print a shipping label. Auto-updating tracking timeline visible to both owner and customer.

## Implementation Prompt
Integrate EasyPost to provide automated shipping fulfillment. During checkout for physical products, use the EasyPost API to display real-time shipping rates based on the merchant's configured origin address and the customer's destination. Add a "Generate Shipping Label" action on the order details view that purchases the label and provides a printable PDF. Automatically save the resulting tracking number to the order and trigger a "Shipped" notification to the customer.

## Priority
P1

## Estimated Scope
Medium
