# Integrate EasyPost for Automated Shipping & Labels

## Problem Statement
For small e-commerce business owners, fulfilling physical orders is incredibly tedious. Manually copying addresses into carrier websites (USPS, FedEx, UPS) to buy and print shipping labels takes up a large chunk of their day and introduces human error. They need a way to purchase and print shipping labels directly from their order management dashboard.

## Research Report
**Tool**: EasyPost
EasyPost is a multi-carrier shipping API that aggregates over 100 carriers into a single integration.
- **Ease of use**: Very developer-friendly. For the business owner, they only need an EasyPost account, which connects automatically to default carrier accounts (like USPS).
- **Pricing**: Developer plan is free for up to 120,000 shipments/year, making it effectively free (excluding actual postage costs) for most small businesses.
- **Reputation**: Industry standard for modern e-commerce shipping platforms. Very high uptime.
- **Environment**: Cloud-based REST API. Works perfectly in both Cloud and Standalone OHC instances since it just requires outbound API calls.

## Design Doc
The integration will allow the business owner to generate shipping labels from the OHC order detail page.
- **Trigger**: User opens an unfulfilled physical order in OHC and clicks "Buy Shipping Label".
- **Actions**: OHC prompts for package dimensions/weight, calls EasyPost to fetch rates, allows the user to select a rate, and purchases the label. OHC then downloads the label PDF and saves the tracking number to the order.
- **User View**: A "Fulfillment" section on the order page showing available shipping rates. Once bought, a "Print Label" button appears, and tracking info is automatically emailed to the customer.

## Implementation Prompt
On the Order Details page, add a "Create Label" workflow for physical products. The user should be able to input package weight and dimensions. Use the EasyPost API to create a shipment and display the returned shipping rates (e.g., USPS Priority, FedEx Ground). When the user selects a rate and confirms, purchase the label via EasyPost. Save the resulting tracking URL to the order record and display a prominent "Download PDF Label" button for the user to print.

## Priority
P1

## Estimated Scope
Medium
