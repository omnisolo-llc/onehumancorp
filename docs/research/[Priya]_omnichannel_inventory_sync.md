# Issue Brief: Omnichannel Inventory Guardian (Priya)

## Problem Statement
Priya (Boutique Owner, 35) sells both in-store and online. Her biggest nightmare is a "Double Sell" — where someone buys a unique dress online at the same time she's selling it to a customer in the shop. She manually updates her inventory every evening, which is error-prone and exhausting.

## Research Report
- **Competitor Audit**: Shopify POS is excellent but expensive ($89/mo for the full retail plan). Square is good but online/offline sync can be laggy.
- **Pain Point**: "Stock Anxiety" - small owners live in fear of having to cancel orders and apologize to customers.
- **Opportunity**: OHC can act as the "Single Source of Truth" that proactively locks stock across all channels.

## Design Doc
### High-Level Architecture
- **The Lock Engine**: When an item is added to an online cart, the Manager agent "Soft-Locks" it for 10 minutes.
- **POS Integration**: Real-time webhook integration with Square/Clover POS.
- **Proactive Alerts**: The Manager agent notifies Priya via mobile if stock levels for a trending item drop below 2.

### Mobile UX Flow (375px)
1. **Alert**: "Priya! You just sold the Blue Silk Dress in-store. I've updated the online store and flagged it as 'Sold Out'."
2. **Dashboard**: A "Real-Time Stock" widget that highlights discrepancies.

### AI Agent Integration
- **The Manager**: Real-time inventory orchestration and channel synchronization.
- **The Advisor**: Forecasting stock-outs based on sales velocity.

## Implementation Prompt
Create an "Omnichannel Inventory Sync" module. This module must support real-time inventory updates from external POS systems (e.g., Square) via webhooks. When a sale occurs offline, the "Manager" agent should immediately update the OHC storefront. Additionally, implement "Soft-Locking" for online carts to prevent over-selling of low-stock items. The owner should receive proactive notifications for any stock-level changes that impact online availability.

## Priority
P1

## Estimated Scope
Large
