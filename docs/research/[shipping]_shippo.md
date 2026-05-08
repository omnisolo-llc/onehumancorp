**Title**: Integrate Shippo for OHC

## Problem Statement
Calculating shipping costs and printing labels for my handmade products takes hours every week. I have to guess the shipping cost during checkout.

## Research Report
**Tool Evaluated:** Shippo

**Findings:** Shippo provides a single API to access rates and print labels for 85+ global carriers (USPS, UPS, FedEx, DHL). It offers discounted rates and is tailored for SMBs and e-commerce platforms. The API handles address validation, rating, and tracking.

**Pricing:** Free tier available (pay for postage only); Pro tier starts at $19/mo.

**Cloud vs Standalone Mode:** Works seamlessly in both environments via standard REST API.

## Design Doc
When an order is placed in OHC, the order details are sent to Shippo to generate shipping rates. The owner can select a rate, generate a label in OHC, and OHC will email the tracking number to the customer.

## Implementation Prompt
Integrate Shippo to allow business owners to generate and print shipping labels directly from the OHC order management screen. Automatically calculate live shipping rates at customer checkout based on the store's origin address.

## Priority
P1

## Estimated Scope
Large
