# Shippo Integration for Shipping Logistics

## Title
Automate Shipping Labels and Tracking with Shippo

## Problem Statement
E-commerce small business owners waste significant time manually copying customer addresses into carrier websites to buy shipping labels and then pasting tracking numbers back into emails to send to customers. They need a system that calculates shipping rates, generates labels, and tracks packages automatically from within their main dashboard.

## Research Report
Shippo is an American e-commerce software company that provides an API and web interface for shipping logistics. According to Wikipedia, over 100,000 businesses use their system. Shippo aggregates customers' packages to receive discounted rates from multiple carriers, passing those savings onto the business.

Key features include generating shipping labels, address validation, multi-carrier support, and API tracking. By connecting to Shippo, OHC can instantly offer integration with dozens of carriers globally without building individual integrations. Shippo's pricing is typically per-label plus postage costs, making it highly affordable for small businesses. It is well-suited for both Cloud and Standalone deployments.

## Design Doc
When an order is marked as "Ready to Ship" in OHC, the business owner can click a "Create Label" button. OHC will send the package dimensions, weight, and destination to Shippo to fetch rate quotes. The user selects a rate, and OHC purchases the label via Shippo. The printable label is displayed in OHC, and the resulting tracking number is automatically emailed to the customer. The order's status will visually update in OHC as Shippo tracks the package's journey.

## Implementation Prompt
Integrate Shippo to handle shipping logistics. Create a UI flow for an order that allows the user to input package details, view real-time shipping rates, and purchase a label. Automatically save the generated label PDF for printing and attach the tracking number to the order profile. Set up tracking webhooks to update the order status (e.g., "In Transit", "Delivered") automatically.

## Priority
P2

## Estimated Scope
Large
