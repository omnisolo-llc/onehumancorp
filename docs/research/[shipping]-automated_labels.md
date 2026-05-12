# Automated Shipping Label Generation

## Problem Statement
Calculating shipping rates manually and buying labels on external sites slows down order fulfillment.

## Research Report
Shippo and EasyPost evaluated. Both offer multi-carrier support. Shippo has slightly better pricing for small volumes. EasyPost has a more robust API. EasyPost recommended.

## Design Doc
When an order is marked as 'Ready to Ship', query EasyPost for rates, allow the owner to select a rate, and generate a printable PDF label. Tracking numbers are auto-saved to the order.

## Implementation Prompt
Add a 'Create Shipping Label' button on order details. It should fetch rates, let the user choose one, purchase the label, and provide a link to download the PDF.

## Priority
P2

## Estimated Scope
Medium
