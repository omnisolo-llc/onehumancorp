# [Shipping] Shippo Integration

## Title
Implement Shippo for Multi-Carrier Shipping & Automated Label Generation

## Problem Statement
Small business owners selling physical goods struggle with managing shipments manually. Maya (Artisan Baker) needs a way to fulfill her local and regional shipments in a few clicks without leaving the OHC platform, eliminating manual shipping rate calculations and label generation.

## Research Report
- **Strategy**: Direct API integration with Shippo
- **Target Persona**: Maya (Artisan Baker), Priya (Boutique Owner)
- **Advantages**: Shippo offers a pay-as-you-go model (no monthly fee). Users can purchase and print labels directly from the dashboard. Wide carrier support.
- **Risks**: Reliance on carrier APIs which can occasionally be slow or down.
- **Pricing**: Free tier for low volume (only pay for postage + 5¢ per label).
- **Compatibility**: Cloud and Standalone compatible via API.

## Design Doc
**Trigger:**
1. A new order is placed with a physical shipping address.
2. The user navigates to the specific order details page within OHC and selects "Fulfill Order".

**Actions:**
1. OHC fetches live shipping rates from Shippo based on order weight, dimensions, and destination.
2. The user selects a shipping rate and purchases the label directly within OHC.
3. OHC retrieves the generated shipping label (PDF/ZPL) and tracking number from Shippo.
4. OHC automatically marks the order as "Shipped" and emails the tracking number to the customer.

## Implementation Prompt
Integrate Shippo to enable users to view live shipping rates, purchase shipping labels directly, and automatically sync tracking information back to OHC orders.

## Priority
P1

## Estimated Scope
Medium
