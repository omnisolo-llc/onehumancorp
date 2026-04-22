# 📦 Shipping & Logistics: Shipping Hub

## Title
Integrated Shipping Logistics and Label Generation

## Problem Statement
For users selling physical products, managing shipping is a major friction point. Calculating shipping costs accurately at checkout and generating shipping labels manually is error-prone and time-consuming. They need an automated way to present live shipping rates to customers and print labels with one click.

## Research Report
- **Goal**: Evaluate logistics APIs for live rates, label generation, and package tracking.
- **Tools Evaluated**:
    - **EasyPost**: Excellent, developer-friendly API. Supports hundreds of carriers globally. Good pricing model.
    - **Shippo**: Strong competitor, very similar feature set. slightly better out-of-the-box UI components if we needed them, but we prefer building our own UI.
    - **ShipEngine**: Robust, powers ShipStation. Very comprehensive but API can be slightly more complex.
- **Recommendation**: Integrate with **EasyPost**. Its API structure aligns well with our backend architecture, and it supports both domestic and international carriers reliably. It is API-driven and functions securely in both Cloud and Standalone modes.
- **User Impact**: A customer buying a candle sees live USPS and UPS rates at checkout. Once the order is placed, the OHC dashboard allows the seller to click "Generate Label," print it, and the system automatically sends a tracking email to the customer.

## Design Doc
- **Component**: `LogisticsAgent`
- **Responsibilities**:
    - Fetch live shipping rates based on cart weight, dimensions, and origin/destination addresses during checkout.
    - Purchase and generate shipping labels via the API.
    - Subscribe to tracking webhooks to update order status (e.g., "Shipped", "Delivered").
- **Integration Point**: Checkout flow queries for rates. Operations dashboard requests labels. Order system receives tracking updates.

## Implementation Prompt
Implement the Shipping Logistics integration using the chosen provider (e.g., EasyPost). Create endpoints to calculate shipping rates based on package dimensions and addresses. Implement the flow to purchase a shipping label and return the PDF/PNG URL to the frontend. Set up webhook handlers to process tracking updates and update the internal order status accordingly.

## Priority
P1

## Estimated Scope
Medium
