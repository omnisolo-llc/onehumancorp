# Title: Shippo Shipping & Logistics Integration

## Problem Statement
For small business owners selling physical goods, shipping is often the most complex and time-consuming part of their operation. Calculating rates manually, waiting in line at the post office, and managing tracking numbers across multiple carriers is inefficient. They need a unified system that automatically calculates the cheapest rates, prints shipping labels instantly, and provides tracking updates, all from one dashboard.

## Research Report
*   **Overview**: Shippo is a multi-carrier shipping software designed for e-commerce. It connects businesses to dozens of carriers (USPS, UPS, FedEx, DHL, etc.) to compare rates and print labels.
*   **Ease of Use**: Very intuitive for non-technical users. Owners simply enter package dimensions and weight, compare the discounted rates presented, and click "print label."
*   **Reputation**: Highly trusted by over 300,000 businesses, processing millions of shipments. Known for deep discounted rates (especially USPS).
*   **Pricing**:
    *   **App Pricing (Starter)**: Free. Pay only $0.05 per label if using your own carrier accounts (or free if using Shippo's default discounted accounts). Good for up to 30 labels/month.
    *   **API Pricing (Starter)**: Free for the first 30 labels/month, then 7¢ per label. No monthly subscription fee. Address validation incurs small fees (2¢ US, 8¢ International).
*   **Environment (Cloud vs Standalone)**: Shippo is an API-first platform. It integrates perfectly into a Cloud environment. For Standalone environments, it functions smoothly as long as the local server has internet connectivity to reach Shippo's REST APIs for rating and label generation.
*   **AI Integration**: Shippo utilizes AI for "Estimated delivery dates powered by AI," enhancing customer experience by providing accurate delivery windows.

## Design Doc
*   **Trigger**: A customer places an order requiring physical shipping, or the business owner selects "Fulfill Order" in the OHC dashboard.
*   **Action**: OHC queries the Shippo Rating API with package details to retrieve available rates. The owner selects a rate, and OHC calls the Shippo Transaction API to purchase and generate the shipping label (PDF).
*   **User Interface**: An order fulfillment modal where the owner inputs box dimensions/weight, views a list of carrier options with prices, and clicks a button to generate and download the shipping label. The order status then automatically updates with the tracking number.

## Implementation Prompt
Integrate the Shippo API to provide seamless label generation and rate calculation. The user-facing outcome must allow a business owner to fulfill an order by viewing live, discounted shipping rates, purchasing a label, and downloading the printable PDF directly from the OHC order dashboard. The system should automatically attach the generated tracking number to the order and support address validation to prevent delivery failures.

## Priority
P1

## Estimated Scope
Medium
