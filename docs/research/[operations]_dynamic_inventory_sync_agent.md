# [operations] Dynamic Inventory Sync Agent

## Problem Statement
Boutique owners and food cart operators often struggle with keeping their physical inventory in sync with their online storefront. When Priya sells the last red dress in-store, her online store might still show it as available, leading to canceled orders and unhappy customers.

## Research Report
According to reviews on G2 Crowd and TrustRadius, Shopify and Wix provide inventory tools, but they require manual updates or expensive third-party POS integrations. SMB owners complain about the "two system problem." Our research indicates a need for a unified agent that listens to both online checkouts and physical tap-to-pay events.

## Design Doc
- **Architecture**: The KAIROS Orchestrator receives events from both the online `Checkout` component and the `Terminal` component. The `Operations Manager` agent processes these events.
- **Data Model**: `ProductVariant` entity with a `stock_count` field, protected by row-level locks during concurrent checkouts.
- **UI/UX**:
  - Mobile dashboard shows a "Low Stock" warning card.
  - 1-tap "Mark Sold Out" button on the mobile app home screen for fast-moving items like Fatima's food cart.

## Implementation Prompt
Create a unified inventory synchronization workflow. Integrate a webhook from the POS terminal to trigger a stock decrement. Add a real-time UI update to the mobile dashboard using WebSockets. When a product hits zero stock, the `Operations Manager` should automatically hide it from the public storefront and send a push notification to the owner.
The design should use the OHC Glassmorphism design tokens.

## Priority
P1

## Estimated Scope
Medium
