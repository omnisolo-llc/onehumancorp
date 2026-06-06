# Localized Shipping & Fulfillment Integration Research Report

## Executive Summary
This report analyzes the best integration candidates to handle localized shipping, last-mile delivery, and global fulfillment for OneHumanCorp (OHC) users. A core pain point for physical goods sellers (like Priya the Boutique Owner or Maya the Home Baker) is navigating complex shipping rates, printing labels, and offering local pickup/delivery. We must provide "invisible AI" logistics that just work.

---

## 1. EasyPost (Global Aggregator)
**Problem Statement:** SMBs shipping physical goods need real-time shipping rates during checkout and the ability to easily print shipping labels without integrating multiple carrier APIs (USPS, FedEx, UPS) manually.
**Research Report:**
*   **Tool:** EasyPost API
*   **Ease of Use (for SMB):** Very high. They connect their carrier accounts or use EasyPost default rates. OHC handles the complexity.
*   **Pricing:** 120,000 shipments free per year, then a few cents per label.
*   **Reputation/Reliability:** Industry standard for unified logistics API.
*   **Cloud/Standalone:** Fully supported.

**Design Doc:**
*   **Integration Point:** "Shipping Settings" and the order fulfillment flow.
*   **Trigger:** Customer enters address at checkout (fetches rates); business owner clicks "Fulfill Order" (buys label).
*   **Action:** OHC queries EasyPost for rates, presents them to the customer, and later requests a printable PDF label from EasyPost.
*   **User View:** Business owner sees a "Print Label" button on the order detail page.

**Implementation Prompt:**
Integrate the EasyPost API to fetch live shipping rates at checkout and generate printable shipping labels. The business owner must be able to click "Fulfill" on an order and download a PDF shipping label.
**Priority:** P0
**Estimated Scope:** Large

---

## 2. Shippo (SMB-Friendly Aggregator)
**Problem Statement:** Simplifies label generation and manual shipping rate calculations for physical product merchants.
**Research Report:**
*   **Tool:** Shippo
*   **Ease of Use (for SMB):** Excellent dashboard, extremely SMB focused.
*   **Pricing:** Free tier for low volume (only pay for postage + 5¢ per label).
*   **Reputation/Reliability:** High reliability, extensive carrier support.
*   **Cloud/Standalone:** Cloud and Standalone compatible via API.

**Design Doc:**
*   **Integration Point:** "Orders" tab in the OHC app.
*   **Trigger:** Business owner needs to fulfill an order.
*   **Action:** Directly generate the label via Shippo and update tracking automatically in OHC.
*   **User View:** Simplified "Buy Label" flow with default box sizes suggested by AI based on past orders.

**Implementation Prompt:**
Implement Shippo integration to provide an alternative label generation backend, specifically optimized for users without their own carrier accounts.
**Priority:** P1
**Estimated Scope:** Medium

---

## 3. ShipStation (Advanced Multi-Channel)
**Problem Statement:** Sellers who manage inventory across multiple channels (e.g., in-store, OHC storefront, and maybe an old Etsy shop) need a central place to print labels.
**Research Report:**
*   **Tool:** ShipStation API
*   **Ease of Use (for SMB):** High, but more complex than EasyPost/Shippo.
*   **Pricing:** Monthly subscription ($9.99/mo base).
*   **Reputation/Reliability:** Massive market share for mid-market SMBs.
*   **Cloud/Standalone:** Cloud.

**Design Doc:**
*   **Integration Point:** "Fulfillment" settings.
*   **Trigger:** Order is placed on OHC.
*   **Action:** OHC pushes the order to ShipStation. Once fulfilled in ShipStation, ShipStation sends a webhook back to OHC with the tracking number.
*   **User View:** User continues to use ShipStation for fulfillment, but OHC stays perfectly synced.

**Implementation Prompt:**
Build a two-way sync with ShipStation. Push new OHC orders to ShipStation and listen for fulfillment webhooks to update the order status and notify the customer.
**Priority:** P2
**Estimated Scope:** Large

---

## 4. Sendle (Eco-Friendly & Localized)
**Problem Statement:** Boutique and environmentally conscious brands want carbon-neutral shipping options that are simple and flat-rate.
**Research Report:**
*   **Tool:** Sendle API
*   **Ease of Use (for SMB):** Very high. Flat rates based on simple volume categories.
*   **Pricing:** Free plan available, pay per label.
*   **Reputation/Reliability:** Popular in US and Australia for small, sustainable businesses.
*   **Cloud/Standalone:** Cloud.

**Design Doc:**
*   **Integration Point:** Checkout and Fulfillment.
*   **Trigger:** Customer selects "Eco-Friendly Shipping".
*   **Action:** OHC buys the Sendle label and schedules a pickup (Sendle's core feature).
*   **User View:** The owner doesn't even need to go to the post office; Sendle picks it up from their door.

**Implementation Prompt:**
Integrate Sendle to offer carbon-neutral shipping and automated door pickups for the business owner.
**Priority:** P2
**Estimated Scope:** Medium

---

## 5. Local Delivery & Pickup (In-House OHC Feature)
**Problem Statement:** Businesses like Fatima's Food Cart or Maya's Home Bakery do not use mail carriers. They need radius-based local delivery or scheduled local pickup.
**Research Report:**
*   **Tool:** OHC Core (Geocoding API via Google Maps / Mapbox)
*   **Ease of Use (for SMB):** Perfect. Handled entirely natively.
*   **Pricing:** Free for the SMB (OHC absorbs the minimal geocoding cost).
*   **Reputation/Reliability:** 100% reliable as it relies on core maps infrastructure.
*   **Cloud/Standalone:** Both.

**Design Doc:**
*   **Integration Point:** "Shipping & Delivery" settings.
*   **Trigger:** Owner sets a "10-mile delivery radius" or "Pickup at Storefront".
*   **Action:** At checkout, OHC validates the customer's distance. If within radius, it allows checkout.
*   **User View:** Owner sees a map where they can draw their delivery zone. Customers see "Local Delivery ($5)" or "Free Pickup".

**Implementation Prompt:**
Implement a native Local Delivery and Pickup module. Allow the owner to set a geofenced delivery radius and a flat delivery fee, alongside available pickup time slots.
**Priority:** P0
**Estimated Scope:** Medium
