# Integration Research Report: DoorDash Drive API

## 1. Discovery and Market Need
Small business owners—such as local bakers, florists, independent grocers, and specialized retailers—consistently cite "competing with Amazon Prime and UberEats" as a top pain point. They lose significant sales to massive aggregators because they cannot offer same-day, on-demand local delivery without taking on the logistical nightmare and cost of managing their own fleet of drivers.

Currently, many SMBs are forced to list their businesses on consumer-facing marketplaces (like UberEats or the main DoorDash app). While this provides delivery infrastructure, these marketplaces charge exorbitant commissions (often 15-30% of the gross order value), effectively wiping out the merchant's profit margins.

SMBs desperately need a way to offer a "buy directly from us and get it delivered today" experience on their *own* direct-to-consumer OHC storefronts, without sacrificing their revenue to marketplace commissions or losing control of the customer relationship.

## 2. Tool Deep-Dive Evaluation: DoorDash Drive API
I selected **DoorDash Drive API** as the primary integration candidate to solve this problem.

**What is DoorDash Drive?**
Unlike the consumer DoorDash marketplace app, DoorDash Drive is a white-label fulfillment API. It allows businesses to request a "Dasher" to deliver an order that was placed directly on the merchant's own website or app. The customer never has to download or use the DoorDash app.

**User-First Value Mapping:**
For our personas, like Carlos (a local baker) or Maya (an independent florist):
- **Problem Solved:** They can offer same-day delivery without hiring drivers or giving up 30% of their revenue.
- **Presentation to Non-Technical Owner:** A simple "Enable Local Delivery" toggle in the OHC dashboard. When an order comes in, they click a button (or it happens automatically) and a driver simply shows up to pick it up, exactly as if it were a standard online order.

**Capabilities & Technical Assessment:**
- **API Quality & Documentation:** DoorDash provides a robust REST API with clear documentation, sandbox environments, and webhooks for real-time status updates.
- **Workflow:**
  1. OHC queries the Drive API to generate a delivery quote (cost and feasibility based on distance).
  2. The customer accepts and pays at checkout on the OHC storefront.
  3. OHC sends an API request to create the delivery.
  4. Webhooks update OHC on the Dasher's status (assigned, arrived, picked up, delivered).
- **SLA & Reliability:** Backed by the largest on-demand delivery network in the US, ensuring high availability of drivers.

**SaaS Viability & Pricing Models:**
- **Pricing Model:** DoorDash Drive charges a flat fee per delivery (typically ranging from $7.00 to $9.00 depending on distance and market), rather than a percentage commission. This is highly favorable for SMBs, as they can either absorb the flat fee or pass it directly to the consumer at checkout, maintaining their full product margins.
- **Cloud (Multi-tenant) Mode:** OHC can manage a master DoorDash Drive account. We can abstract all API keys from the SMB. The SMB enables the feature, and OHC automatically adds the flat delivery fee to the consumer's cart at checkout.
- **Standalone (Local/Private) Mode:** The OHC platform can allow advanced users to input their own DoorDash Drive Developer API keys and webhook URLs to manage billing and operations directly.

## 3. Conclusion and Strategic Dispatch
Integrating the DoorDash Drive API is a high-impact, high-priority (P1) opportunity that directly solves a critical margin and logistics problem for local SMBs. It allows OHC to offer a tier-1, enterprise-grade local fulfillment experience out of the box, significantly differentiating the platform from basic website builders.

A detailed Issue Brief (`docs/research/[delivery]_doordash_drive.md`) has been created for the engineering swarm following the Mission Queue Protocol.