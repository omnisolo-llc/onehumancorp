# Shipping & Logistics Integration

## Title
Integrate Shippo for Shipping & Logistics

## Problem Statement
Calculating shipping rates manually and buying labels at the post office is incredibly tedious. Business owners need automated rate calculation and label printing.

## Research Report
**Tool Evaluated:** Shippo
**Pricing:** $0.05 per label or $10/mo
**Cloud/Standalone Support:** Cloud: Yes. Standalone: Yes (API driven).

**Findings:**
Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) and provides discounted rates. The API is developer-friendly. Non-technical users benefit from a unified interface to buy and print labels. Pricing is pay-as-you-go ($0.05 per label) or $10/mo.

## Design Doc
When an order is placed, OHC queries Shippo for shipping rates. In the OHC order management view, the owner sees a 'Buy Shipping Label' button. Clicking it purchases the label via Shippo and provides a printable PDF.

## Implementation Prompt
Add a 'Fulfillment' module that connects to Shippo. Allow business owners to view live shipping rates for an order and purchase/print a shipping label directly from the OHC order details page.

## Priority
P1

## Estimated Scope
Large
