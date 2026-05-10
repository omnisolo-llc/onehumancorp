# Title: Simplified Fulfillment via ShipStation
## Problem Statement
E-commerce small business owners waste hours manually calculating shipping rates, buying postage, and pasting tracking numbers into customer emails.

## Research Report
**Tool Evaluated:** ShipStation
- **Ease of Use:** High, very popular among SMBs.
- **Pricing:** Starts at $9.99/month.
- **Reputation:** Industry standard for multi-carrier shipping.
- **Advantages:** Broad carrier coverage (USPS, UPS, FedEx, international), reliable API.
- **Risks:** Sometimes can be slow during peak holiday seasons.
- **Environment:** Cloud and Standalone compatible.

## Design Doc
OHC will push new orders to ShipStation. Business owners can review orders and print labels in batch. ShipStation will webhook back the tracking number to OHC, which will then automatically notify the customer.

## Implementation Prompt
Connect OHC orders to ShipStation so that new orders automatically appear in the merchant's ShipStation account. Once a label is created, ensure the tracking number syncs back to OHC and updates the order status to fulfilled.

## Priority
P1

## Estimated Scope
Medium
