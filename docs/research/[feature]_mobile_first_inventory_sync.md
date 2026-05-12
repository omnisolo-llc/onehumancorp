# Mobile-First Inventory Sync

## Title
Mobile-First Cross-Channel Inventory Sync & Reordering

## Problem Statement
For non-technical small business owners like Priya (boutique owner), managing inventory across both in-store physical locations and an online storefront is overwhelmingly complex. Current platforms require manual sync, leading to overselling or missed sales. When stock is low, the process of reordering from suppliers is entirely manual and often forgotten until stock runs out completely, severely impacting revenue.

## Research Report
Based on an audit of Shopify, Wix, and Squarespace, all major platforms treat mobile inventory management as a secondary experience (often requiring a separate app, like Shopify POS). App Store reviews for Shopify (1-2 stars) frequently cite: "I sold an item in my store and the online shop didn't update, now I have angry customers." OHC lacks a unified, mobile-first inventory entity that syncs in real-time between physical and digital storefronts with an integrated AI agent to automate reordering.

## Design Doc
**High-Level Architecture:**
- **Entity Types:** `Product`, `InventoryLevel`, `Supplier`, `ReorderRule`.
- **Key Relationships:** A `Product` has multiple `InventoryLevel` records (per location: 'Store', 'Online').
- **Integration Points:** POS webhook ingest, Online checkout listener.
- **Mobile UX Flow (375px first):**
  1. User opens the OHC app.
  2. Dashboard highlights "Low Stock Alerts" prominently at the top.
  3. Tapping an alert shows the product and a 1-tap "Restock from Supplier" button.
  4. User confirms quantity, and the system automatically emails the supplier.
- **AI Agent Integration:** A background agent monitors `InventoryLevel`. When a threshold is breached, it drafts a supplier reorder email and surfaces the 1-tap approval in the activity feed.

## Implementation Prompt
Implement a unified inventory synchronization engine that updates stock levels across both 'Online' and 'In-Store' locations in real-time. Ensure the UI for managing these levels is optimized for a 375px viewport, presenting stock updates via a simple activity feed. Create a background worker (the AI Reorder Agent) that detects low-stock events and generates a 1-tap approval notification for the business owner to reorder stock from predefined suppliers.
Acceptance Criteria:
- Inventory updates from either channel instantly reflect in the other.
- The low-stock threshold triggers an actionable notification.
- The user can approve a supplier reorder email with a single tap from a mobile device.

## Priority
P1

## Estimated Scope
Medium
