# [operations] AI Automated Inventory Synchronization Agent

## Title
**Autonomous Inventory Sync & Proactive Stock Agent**

## Problem Statement
Small business owners selling both in-person (POS) and online (like Priya the boutique owner) suffer from constant inventory desynchronization. Managing dual systems requires manual data entry, which is time-consuming and leads to out-of-stock items being sold online, or vice-versa. They do not want to manage "spreadsheets" or configure mapping rules between POS and web.

## Research Report
Based on reviews of Shopify (which requires technical configuration or third-party apps for robust multi-location/POS sync for micro-merchants) and Wix (which can be clunky when dealing with large physical-to-digital catalogs), true beginners want a "zero-touch" approach.
- **Evidence:** "Managing two systems for online and in-person is a nightmare." (Common pain point observed from competitor reviews).
- **Competitor Flaw:** Competitors require users to manually link products, manage SKUs across platforms, and click through multiple screens to update stock.
- **OHC Opportunity:** Instead of a complex dashboard, OHC should use an invisible agent to monitor POS sales and automatically deduct online stock, and when new stock arrives, automatically identify it via a mobile camera scan and update both channels.

## Design Doc
- **Key Entities:** `Product`, `InventoryTransaction`, `Location` (Physical/Online).
- **Agent Integration:** `Operations Agent` handles inventory tracking via an event-driven queue.
- **UI Flow:**
  - Owner receives new stock.
  - Owner opens OHC mobile app, selects "Scan Inventory".
  - Camera identifies the product (using an AI vision model) or scans barcode.
  - Agent confirms: "Added 10x Floral Dresses. Updated online and in-store stock."
  - When a dress is sold in-store via OHC POS, the agent immediately updates the online stock to 9 and can optionally trigger an SMS notification if stock is low.
- **Architecture:** The agent operates behind the scenes, processing events (like a sale or a scan) and updating the unified capacity/inventory ledger.

## Implementation Prompt
**User-Facing Outcome:** The user never has to manually edit a stock number in a form field. They scan items to add them, and the system automatically deducts them when sold anywhere.
**Critical User Journey:**
1. User receives a shipment of goods.
2. User uses the OHC mobile app to scan the items.
3. The Operations Agent automatically recognizes the items, updates the inventory count, and ensures the changes are reflected on the storefront instantly.
**Acceptance Criteria:**
- The system must provide a mobile interface for scanning items to increment stock.
- Sales on any channel (online checkout or mobile POS) must instantly decrement stock.
- The Operations Agent must handle the backend deduction and notify the user via a plain-language summary if stock reaches a critical threshold.

## Priority
P1

## Estimated Scope
Medium
