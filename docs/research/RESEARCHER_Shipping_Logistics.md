# Shipping & Logistics Brief

## Problem Statement
For e-commerce sellers, manually calculating shipping rates and purchasing labels at the post office is a major bottleneck that prevents scaling.

## Research Report
**Tool Evaluated:** Shippo
**Findings:** Shippo aggregates rates across major carriers (USPS, UPS, FedEx, DHL) and allows for one-click label generation. It significantly streamlines the fulfillment process.
**Pricing:** Free to use (pay for labels) or a small per-label fee; pro plans around $10/month.
**Ease of Use:** Simplifies a complex process, though understanding different carrier options still requires a learning curve.
**Risks:** International shipping involves complex rules (customs, duties) that can still confuse users.

## Design Doc
**Trigger:** A new physical order is placed.
**Action:** OHC fetches shipping rates based on package dimensions and destination. The owner can purchase a label, which generates a tracking number that is sent to the customer.
**User Experience:** An "Orders" dashboard where the owner sees pending shipments, compares carrier rates, buys labels, and marks orders as fulfilled in a few clicks.

## Implementation Prompt
**Outcome:** An integrated shipping management system where business owners can easily compare rates and purchase shipping labels for their orders.
**Acceptance Criteria:**
- System calculates accurate shipping rates based on order details.
- Owner can purchase and print a shipping label directly from OHC.
- Tracking information is automatically provided to the customer.

## Priority
P2

## Estimated Scope
Large
