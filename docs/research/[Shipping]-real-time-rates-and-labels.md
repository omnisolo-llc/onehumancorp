# Shipping & Logistics: Real-time Rates and Labels

## Title
Auto-Generate Shipping Labels and Track Packages

## Problem Statement
E-commerce and physical product sellers waste immense time manually typing addresses into courier websites to buy shipping labels. They also struggle to give customers accurate shipping costs at checkout.

## Research Report
- **Tools Evaluated:** Shippo, EasyPost, ShipStation API.
- **Ease of Use:** EasyPost and Shippo abstract hundreds of carriers into one API.
- **Pricing:** Shippo/EasyPost usually charge a few cents per label + postage costs.
- **Reputation:** Both are highly reliable and widely used by modern e-commerce platforms.
- **Cloud vs Standalone:** Works seamlessly in both, as it relies on outbound API calls and standard webhook/polling for tracking updates.

## Design Doc
- **Trigger:** User views a paid order and clicks "Generate Shipping Label".
- **Action:** OHC sends package dimensions and addresses to the shipping API, gets rates, buys the label, and returns a PDF.
- **User View:** A simple list of available shipping rates. One click to buy, and a PDF label pops up to print. A tracking link is automatically emailed to the customer.

## Implementation Prompt
Integrate a shipping API (like EasyPost or Shippo) to allow users to purchase and print shipping labels directly from an OHC order page. The system must automatically fetch shipping rates based on standard box sizes, allow the user to select a rate, generate the PDF label, and save the tracking number to the order details.

## Priority
P2

## Estimated Scope
Medium
