# OHC Scout: Tool Integration Research Report - Shippo

## Phase 1: Dynamic Integration & Market Need Discovery
**Ecosystem Scraping & Community Mining**
Through analysis of eCommerce platform app stores (Shopify, Wix, WooCommerce) and small business communities, a recurring, high-friction pain point emerged: shipping logistics. Small business owners (like Leo, selling physical goods) frequently struggle with:
1. Negotiating competitive shipping rates across multiple carriers (USPS, FedEx, UPS).
2. Manually entering tracking numbers and managing returns.
3. Managing disjointed multi-platform fulfillment (printing a label on USPS.com, copying tracking to their website, emailing the customer).

**Identified Target:** **Shippo (goshippo.com)**
Shippo stood out as a leader in democratizing enterprise-grade logistics for SMBs. It offers a unified API and web app to compare rates across 40+ carriers and print labels at highly discounted rates, abstracting away the complex carrier negotiation process.

---

## Phase 2: Deep Dive Evaluation (Shippo)
**User-First Value Mapping**
For a non-technical user, Shippo acts as an automated post office. Instead of driving to USPS or maintaining a separate FedEx account, the merchant connects Shippo to their OHC dashboard. When an order arrives, they click "Print Label" and immediately get the cheapest rate possible, with tracking instantly synced. This saves hours of manual data entry per week and reduces shipping costs by up to 90% off retail rates.

**Capabilities & Limits**
- **Core APIs:** Address validation, rate shopping, label generation, package tracking, and automated return label generation.
- **Reliability:** Processes over 200M+ shipments annually for 4.6M+ customers. API is RESTful and well-documented with high uptime.
- **OAuth & Webhooks:** Supports OAuth 2.0 for secure account connection and webhooks for real-time tracking updates (e.g., package scanned, out for delivery).

**SaaS Viability & Pricing**
- **SMB Friendly:** Shippo offers a "Pay As You Go" tier that is perfect for small businesses. It charges no monthly subscription fees; users only pay $0.05 per label plus postage.
- **Platform/API Pricing:** For platform providers (like OHC acting as a hub), API access for label generation can be integrated efficiently.
- **Hybrid Support:**
  - *Cloud (Multi-tenant):* Can utilize Shippo’s platform/OAuth APIs to manage multiple tenant connections securely.
  - *Standalone (Local/Private):* The API architecture supports standard API key authentication, allowing a local desktop client to securely broker its own connection directly with Shippo.

---

## Phase 3: Synthesis & Brief

# Title: Multi-carrier Shipping and Label Printing Integration via Shippo

## Problem Statement
Small business owners selling physical goods struggle with the complexity, cost, and time sink of managing shipping. Going to the post office, manually entering tracking numbers, comparing rates across carriers (USPS, UPS, FedEx, DHL, etc.), and managing returns is a massive pain point. Traditional website builders often have clunky or expensive shipping add-ons. Non-technical business owners need a simple, centralized way to print shipping labels at discounted rates, track packages, and automate fulfillment updates directly from their One Human Corp (OHC) dashboard without managing multiple carrier accounts.

## Research Report
**Tool Analyzed:** Shippo (https://goshippo.com/)
**Category:** Multi-carrier Shipping & Logistics

**Findings:**
- **Value Proposition:** Shippo consolidates 40+ global shipping carriers into a single API and app. It provides deep discounts (up to 90% off retail rates) on shipping labels without requiring merchants to negotiate their own carrier contracts.
- **Ease of Use for Non-Technical Users:** Excellent. For a small business owner, Shippo removes the need to understand complex shipping APIs. They simply connect their store, select an order, review pre-calculated rates, and click "Print Label."
- **Pricing:** Very SMB-friendly. Shippo offers a "Pay As You Go" tier that is free to use (no monthly fee), charging only a nominal per-label fee ($0.05) plus the cost of postage. They also have a Professional tier for growing businesses with custom branding needs. The API pricing is also free for volume-based label generation for platforms.
- **Capabilities:** Address validation, rate shopping, label generation, package tracking, and automated return labels.
- **Hybrid Viability:**
  - *Cloud (Multi-tenant):* Shippo’s robust OAuth and multi-tenant platform APIs make it ideal for cloud integration, allowing OHC to act as the central hub for our users' shipping needs.
  - *Standalone (Local/Private):* The API can be securely called from standalone desktop clients using individual user API keys or a securely brokered OAuth connection.
- **Reputation:** Highly trusted. Used by 4.6M+ customers and integrates seamlessly with platforms like Shopify, Square, Wix, and WooCommerce.

## Design Doc
**Integration Flow (High-Level):**
1. **Onboarding/Trigger:** A user (e.g., Leo, who runs a local craft store) navigates to the "Shipping & Fulfillment" tab in OHC. They click "Connect Shippo" to authenticate via OAuth or enter their API key.
2. **Order Sync:** When an order is placed through an OHC-managed channel (or external connected store), OHC retrieves the order details (destination address, items, weight).
3. **Rate Shopping:** OHC pings the Shippo API in the background to fetch real-time shipping rates from various carriers.
4. **User Action (Label Generation):** The user views the order in OHC, sees the available shipping options sorted by price/speed, selects one, and clicks "Purchase & Print Label."
5. **Post-Purchase:** OHC securely stores the tracking number returned by Shippo, marks the order as "Shipped," and triggers an automated email/SMS to the customer with the tracking link.
6. **Returns (Optional):** Users can click "Generate Return Label" in OHC, which requests a scan-based return label from Shippo to email to the customer.

## Implementation Prompt
**User-Facing Outcome:**
Implement a seamless shipping fulfillment workflow within OHC that allows small business owners to buy and print discounted shipping labels without leaving the platform.
**Acceptance Criteria:**
- The OHC dashboard includes a "Fulfillment" or "Shipping" settings section where a user can connect a Shippo account.
- When viewing an unfulfilled order, the user can input/verify package dimensions and weight.
- The UI displays a list of real-time shipping rates from different carriers (e.g., USPS Priority, UPS Ground).
- The user can click a button to purchase the selected label, deducting funds from their configured payment method in Shippo.
- The purchased label is presented to the user as a downloadable/printable PDF.
- The order status automatically updates to "Shipped," and the tracking number is saved and visible on the order details page.

## Priority
P1 (High) - Shipping is a critical, revenue-blocking operation for e-commerce and local retail SMBs. Solving this builds massive trust.

## Estimated Scope
Medium
