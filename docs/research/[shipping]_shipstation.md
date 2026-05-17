# ShipStation Integration Issue Brief

## Title
Integrate ShipStation for Automated Shipping and Label Generation

## Problem Statement
E-commerce small businesses spend countless hours manually entering addresses into carrier websites to print shipping labels. They need a unified dashboard to compare rates, print labels, and track packages across multiple carriers (USPS, UPS, FedEx).

## Research Report
- ShipStation is a leading multi-carrier shipping software designed specifically for e-commerce.
- It connects to dozens of carriers globally and offers deeply discounted rates (especially for USPS).
- Pricing: Monthly subscription plans based on shipment volume.
- Competitors: Shippo (more API-focused, pay-as-you-go), EasyPost (developer-centric). ShipStation offers the best balance of features for non-technical merchants.
- Integration: Robust REST API for creating orders, generating labels, and receiving tracking updates.
- Cloud/Standalone: Works seamlessly in Cloud mode. Standalone mode might require webhooks to be proxied.

## Design Doc
- Users connect their ShipStation account in the "Fulfillment" dashboard.
- When an order is placed in OHC, it is automatically pushed to ShipStation.
- Users can view and print shipping labels directly from the OHC order detail page, leveraging the ShipStation API.
- Tracking numbers are automatically synced back to OHC and the "Concierge" AI sends a notification to the customer.

## Implementation Prompt
Implement a ShipStation integration. Create a background worker that pushes new "Paid" OHC orders to ShipStation. Add a UI button on the order page to "Generate Label" which calls the ShipStation API and returns a printable PDF. Implement a webhook listener to receive tracking numbers when ShipStation marks an order as shipped, and update the OHC order status.

## Priority
P1

## Estimated Scope
Large
