# Unified Inventory and Booking Sync

## Problem Statement
Hybrid businesses (like Priya, 35, boutique owner) struggle to keep online and in-store inventory synchronized, or manage physical product sales alongside service bookings (e.g., Leo, 22, music tutor).

## Research Report
*   **Competitor Analysis:** Existing tools often segment product sales and service bookings into completely different modules or require expensive third-party apps.
*   **Opportunity:** A unified backend where a "SKU" can be a physical item, a digital download, or a time slot.

## Design Doc
*   **Data Model:** Abstract `SellableItem` entity that handles both physical stock counts and calendar availability.
*   **UI:** A single dashboard to manage all offerings, regardless of type.

## Implementation Prompt
Develop a unified catalog system that supports both physical products and service bookings without requiring separate add-ons. The system must seamlessly sync inventory levels and calendar availability to prevent double-booking or overselling.

## Priority
P1

## Estimated Scope
Large
