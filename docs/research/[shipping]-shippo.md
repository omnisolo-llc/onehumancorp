# Title: Painless Shipping Labels and Tracking via Shippo

## Problem Statement
Small e-commerce businesses waste hours copying addresses from their inbox into carrier websites (USPS, UPS, FedEx) to buy shipping labels. They need a way to instantly buy and print a shipping label the moment an order is placed, without leaving their dashboard.

## Research Report
- **Tool Evaluated**: Shippo API
- **Benefit to Users**: Automates shipping label generation and provides discounted carrier rates.
- **Ease of Use**: When an order comes in, the owner clicks "Buy Label", confirms the box weight, and the label PDF is instantly downloaded.
- **Pricing**: Pay-as-you-go model (cents per label) or free tier for default carrier accounts. Offers deep discounts on USPS/UPS rates which directly saves the business owner money.
- **Integration Risks**: Handling international customs forms (commercial invoices) programmatically is complex. Edge cases with address validation can block label creation.
- **Environment**: Cloud and Standalone compatible.

## Design Doc
- **Trigger**: User views a new "Order" in the OHC dashboard.
- **Action**: User clicks "Create Shipping Label". OHC fetches rates from Shippo based on saved package dimensions and the customer's address.
- **User Interface**: A simple modal shows the cheapest rate and the fastest rate. Upon selection, the tracking number is automatically emailed to the customer, and the label PDF opens for printing.

## Implementation Prompt
Integrate the Shippo API to enable shipping label purchases directly from order records. Automatically validate the destination address. Present the user with rate options, allow them to purchase the label, and store the resulting tracking number. Automatically notify the customer of the tracking details once the label is generated.

## Priority
P2

## Estimated Scope
Medium