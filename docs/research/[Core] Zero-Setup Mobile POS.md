# [Core] Zero-Setup Mobile POS

## Problem Statement
In-person sellers (like Fatima the food cart owner) need a simple way to take orders and payments on their phone without navigating complex back-office software or buying additional hardware.

## Research Report
- **Competitor Landscape**:
  - Square dominates this, but their online store integration can be clunky.
  - Shopify POS is powerful but complex and expensive.
- **Pain Point Validation**: Users complain about the disconnect between their online store inventory and in-person sales.
- **Opportunity**: A mobile-first POS that is instantly available on the user's phone, synced perfectly with their OHC online store, requiring zero setup.

## Design Doc
- **Architecture**:
  - Mobile web app or native app -> Stripe Terminal / Tap to Pay integration -> Unified Order DB.
- **UI Wireframes (375px first)**:
  - Big keypad for quick entry.
  - "Tap to Pay" prominent button.
  - Quick-select grid for top products.
- **AI Integration**: AI categorizes custom amounts for reporting.

## Implementation Prompt
Build a mobile-optimized point-of-sale interface. It must support Tap to Pay via Stripe and sync automatically with the unified order management system. The focus is on speed of checkout and zero configuration.

## Priority
P1

## Estimated Scope
Medium
