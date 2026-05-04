# [shipping] Integrated Shipping Rates & Label Generation

## Title
Implement Integrated Shipping Rates & Label Generation

## Problem Statement
For users selling physical products, like Maya (The Home Baker) or Priya (The Boutique Owner), fulfilling orders is a major friction point. Manually calculating shipping costs based on weight and distance leads to abandoned carts or lost margins. Going to the post office to buy labels manually is time-consuming. They need a system that calculates real-time shipping rates at checkout and allows them to purchase and print shipping labels directly from the OHC app.

## Research Report
### Market Evaluation
- **EasyPost**: A leading shipping API aggregator.
    - *Ease of use (for OHC)*: Excellent developer experience, normalizes APIs across hundreds of carriers.
    - *Ease of use (for user)*: Users don't need their own carrier accounts to get started; they can use default rates.
    - *Pricing*: Free tier available, pay-per-label model works well.
    - *Cloud vs. Standalone*: Ideal for Cloud where OHC manages the API keys. In Standalone, users would need to create and fund their own EasyPost accounts.
- **Shippo**: Direct competitor to EasyPost.
    - *Ease of use (for OHC)*: Very similar to EasyPost.
    - *Ease of use (for user)*: Good UI dashboard if users want to manage things outside OHC.
    - *Cloud vs. Standalone*: Similar constraints to EasyPost; Standalone mode shifts account management burden to the end user.
- **Direct Carrier Integration (USPS, FedEx, UPS)**:
    - *Pros*: No middleman fees.
    - *Cons*: Extremely complex APIs. Each carrier requires a separate, fragile integration. Unrealistic for OHC's scale and multi-tenant needs.

### Integration Risks & Considerations
- **Data Quality**: Real-time rates require accurate product weights and package dimensions. Non-technical users often skip entering this data, leading to inaccurate rates. OHC needs smart defaults or AI estimation.
- **Label Printing**: Generating a PDF label that prints correctly on different sizes (e.g., standard 8.5x11 vs. 4x6 thermal printers) from a mobile device or web app is challenging.
- **International Shipping**: Customs forms and duties add significant complexity to the label generation process.

## Design Doc
### User Experience
1. **Setup**: In the "Operations" tab, the user enables "Shipping." They define default package sizes (e.g., "Small Box," "Poly Mailer").
2. **Checkout**: When a customer buys a physical product, the checkout flow calculates real-time shipping options (e.g., Standard, Expedited) based on the destination address and estimated weight.
3. **Fulfillment**: The user receives an order notification. In the OHC app, they click "Fulfill Order." They confirm the package size and weight.
4. **Label Generation**: With one tap, OHC buys the label using the funds from the order. A PDF label is generated, which the user can print from their phone or computer. The "Customer Success" agent automatically emails the tracking number to the customer.

### System Flow
- OHC integrates an aggregator API (like EasyPost).
- Checkout flow calls the API to get rates, adding an optional user-defined markup.
- Upon order confirmation, the OHC backend stores the selected shipping method.
- When the user clicks "Fulfill", OHC calls the API to purchase the label (deducting the cost from a pre-funded balance or charging a card on file).
- The label URL/PDF is stored in OHC. Tracking webhooks from the aggregator update the order status (e.g., "Shipped," "Delivered").

## Implementation Prompt
Implement a shipping rate and label generation integration using an aggregator like EasyPost. Create a smooth UX for users to set product weights, calculate rates dynamically at checkout, and generate printable shipping labels directly within the OHC "Operations" dashboard. Ensure tracking updates trigger notifications via the "Customer Success" agent. The system should handle missing weight data gracefully (e.g., via defaults). Do not prescribe specific database schemas or API endpoints; focus on the fulfillment flow and making the label printing experience foolproof on mobile devices.

## Priority
P1

## Estimated Scope
Medium