# [Mobile] Seamless POS & Inventory Sync

## Title
Seamless Mobile Point-of-Sale (POS) and Inventory Sync

## Problem Statement
Boutique owners (like Priya) who sell both in-store and online struggle with inventory synchronization. They often sell an item in-store that was just bought online, leading to cancellations and bad reviews.

## Research Report
- **Competitor Landscape**:
  - Square dominates POS but their online store builder is weak.
  - Shopify POS is powerful but requires expensive hardware and complex setup.
- **User Pain Points**:
  - "I accidentally sold my last dress in-store while someone was checking out online. It was a nightmare." (Trustpilot review).
- **Differentiation**:
  - OHC will use the merchant's mobile phone camera as a POS barcode scanner with instant cloud sync, requiring zero extra hardware.

## Design Doc
- **Architecture**:
  - Entity: `InventoryItem`, `Transaction`.
  - Integration: Mobile camera barcode scanner, real-time WebSocket sync to the OHC backend.
- **UI Wireframes/Flow**:
  - Mobile UX (375px): Large "Scan Item" button on the main tab.
  - Camera opens, scans barcode, shows item details and "Mark Sold / Take Payment" button.
  - Success screen confirms inventory updated across all channels.

## Implementation Prompt
Implement the Seamless POS & Inventory Sync feature. The Critical User Journey involves a merchant using their mobile phone to scan a product barcode in-store, taking a payment (or marking as sold), and having the inventory instantly decrement on their online storefront.
- **Acceptance Criteria**:
  - Mobile UI for barcode scanning.
  - Real-time inventory deduction upon successful scan/sale.
  - Conflict resolution if item sells online concurrently.

## Priority
P1

## Estimated Scope
Medium
