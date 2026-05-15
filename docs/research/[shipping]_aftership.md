# [Shipping] AfterShip for Proactive Support

## Title
🔍 Scout: Integrate AfterShip for Branded Tracking and Proactive Alerts

## Problem Statement
Maya (Home Baker) spends too much time answering "Where is my order?" messages. She needs a way to give her customers a professional tracking page and send them automatic updates so they feel informed and she can focus on baking.

## Research Report
- **Tool**: AfterShip
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner), physical goods sellers.
- **Value Proposition**: AfterShip provides a premium tracking experience for even the smallest business.
- **Key Advantages**:
  - **Branded Tracking Pages**: Customers see the owner's logo, not a carrier site.
  - **Automatic Milestone Alerts**: Sends alerts when status changes.
  - **1,200+ Carrier Support**: Works with almost all global carriers.
  - **Delivery Predictions**: Predicts arrival time accurately.
- **Risks**: Relies on carrier data accuracy.
- **Pricing**: Includes a strong Free tier for low volumes.
- **Compatibility**: Fully compatible in both Cloud and Standalone modes.

## Design Doc
- **User Experience**:
  - In the "Orders" tab, the merchant marks an order as shipped and enters the tracking number.
  - OHC handles the registration automatically.
  - The customer receives a beautiful tracking email.
  - If a package is delayed, OHC alerts the merchant to take action.
- **Visuals**: A simple map showing the package journey.

## Implementation Prompt
Integrate AfterShip tracking capabilities into the OHC fulfillment flow. When an order is marked as shipped, automatically register the tracking and generate a branded tracking URL. Implement a system to receive tracking updates and trigger customer notifications for major milestones. Create a merchant-facing view highlighting any shipping issues.

## Priority
P2

## Estimated Scope
Medium
