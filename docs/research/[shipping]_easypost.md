# Shipping & Logistics: Multi-Carrier Shipping via EasyPost

## Title
Automate Shipping Rates and Label Generation

## Problem Statement
E-commerce small business owners spend hours manually typing addresses into carrier websites (USPS, FedEx, local posts) to compare rates and print shipping labels.

## Research Report
- **Tool Evaluated:** EasyPost
- **Ease of Use:** High. Excellent developer documentation translates to a smooth user experience.
- **Pricing:** Pay-as-you-go (e.g., 1¢ per label), no monthly fees.
- **Reputation:** Extremely reliable, high uptime, broad international carrier support.
- **Cloud/Standalone Compatibility:** API-only (Cloud). Requires internet access for Standalone mode, but can cache some static rates locally if needed.

## Design Doc
- **Integration Point:** "Orders" management screen.
- **User Experience:** When an order is ready to ship, the owner enters package dimensions/weight. OHC instantly displays a list of carrier options sorted by price/speed. The owner clicks "Buy Label," and a printable PDF is generated. Tracking info is automatically emailed to the customer.
- **System Behavior:** OHC queries EasyPost API for rates and label generation. Tracking webhooks from EasyPost update OHC order status.

## Implementation Prompt
Integrate a shipping rate calculator and label printing workflow into the Order Details page. Provide an interface to input package weight and dimensions, fetch rates, and generate a printable shipping label. Automatically update the order status to 'Shipped' and provide the tracking number in the UI.

## Priority
P2

## Estimated Scope
Medium
