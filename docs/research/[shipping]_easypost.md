# Title: Seamless Shipping Label Generation & Tracking

## Problem Statement
Boutique owners like Priya hate manually copying addresses to USPS/FedEx to buy shipping labels. They need a simple, one-click solution to print labels and auto-email tracking numbers to customers.

## Research Report
- **Tool Evaluated**: EasyPost
- **Persona Value**: High time-saver for physical product merchants.
- **Advantages**: Single unified API for 100+ carriers (USPS, FedEx, UPS, DHL). Abstracts complex carrier APIs. Handles tracking webhooks.
- **Risks**: Reliance on carrier APIs which can have downtime.
- **Pricing**: Free tier for low volume, pennies per label after.
- **Cloud vs Standalone**: Cloud and Standalone compatible via API.

## Design Doc
- **Integration Trigger**: Operations agent calculates shipping rate at checkout; user clicks "Print Label" on the order details.
- **Action**: EasyPost generates a PDF label. Tracking webhooks trigger the Ambassador agent to email the customer.
- **User Interface**: Order details view with a "Print Label" button.

## Implementation Prompt
Connect EasyPost to the order fulfillment flow so users can generate shipping labels with one click and automatically send tracking updates to customers via webhooks.
- **Acceptance Criteria**: User can click "Print Label" on an order to download a PDF label. Customer receives an email with the tracking number.

## Priority
P1

## Estimated Scope
Medium
