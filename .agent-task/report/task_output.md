# [shipping] Multi-Carrier Shipping Logistics via Shippo

## Title
Implement Shippo for Multi-Carrier Shipping & Automated Label Generation

## Problem Statement
Small business owners selling physical goods (like Priya or Carlos) spend a significant amount of time copying and pasting order details into different carrier websites (USPS, FedEx, UPS) to find the cheapest shipping rates. They also struggle to print professional shipping labels efficiently and keep their customers updated with accurate tracking information. This manual process is error-prone, eats into their profit margins, and doesn't scale as their order volume grows. Traditional website builders often require expensive premium tiers or clunky workarounds to get access to real-time discounted shipping rates.

## Research Report
**Market Need:** Shipping logistics is consistently ranked as one of the top pain points for e-commerce SMBs on forums like r/smallbusiness and ecommerce communities. A centralized shipping solution is a critical table-stakes feature to compete with Shopify and Wix.
**Tool Evaluated:** Shippo (goshippo.com)
**Ease of Use:** High for end-users. Shippo aggregates multiple carriers into a single unified interface. For developers, the API is modern, well-documented, and RESTful.
**Pricing:** Favorable for SMBs. They offer a "Starter" tier with no monthly fees, charging only a few cents per label printed plus the actual postage cost. They also negotiate deep discounts with carriers (e.g., USPS Commercial Plus Pricing), passing savings directly to the merchant.
**Reputation:** Shippo is a highly reputable API-first shipping service trusted by major platforms (like eBay and Vinted) and thousands of independent merchants.
**Modes:** Can operate in Cloud (multi-tenant) via API keys and OAuth, as well as Standalone (local) using individual merchant API keys.

## Design Doc
**Trigger:** An order is placed or marked "Ready to Ship" in the OHC platform.
**Integration Actions:**
1. The OHC platform securely sends package dimensions, weight, and delivery address to Shippo.
2. Shippo returns real-time rate quotes across multiple configured carriers.
3. Once the user selects a rate, OHC requests label generation from Shippo.
4. OHC stores the tracking number returned by Shippo and updates the order status.
**User Experience:** The business owner sees a clean "Shipping Rates" panel directly within the OHC order dashboard. They can select the cheapest or fastest option, click "Print Label," and the shipping label is generated as a PDF without leaving the OHC interface. Tracking links are automatically emailed to the buyer.

## Implementation Prompt
**User-Facing Outcome:** Merchants can view discounted shipping rates from multiple carriers, purchase postage, and print shipping labels directly from their OHC order management screen. Customers automatically receive tracking notifications when a label is created.
**Acceptance Criteria:**
- The system can retrieve and display comparative shipping rates for a given order address and package dimensions.
- The merchant can purchase a selected rate and generate a downloadable PDF shipping label.
- A tracking number is successfully attached to the order record.
- The UI gracefully handles errors (e.g., invalid addresses or unsupported package dimensions).

## Priority
P1

## Estimated Scope
Medium
