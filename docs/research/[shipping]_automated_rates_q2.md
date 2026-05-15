# [shipping] Issue Brief: Automated Shipping Rates & Labels

**Title**: EasyPost Integration for Real-Time Shipping & Labels
**Problem Statement**: As an artist selling physical prints, I hate trying to guess how much shipping will cost to different states. I either overcharge the customer or lose money. I need the checkout page to automatically calculate the exact shipping cost, and I need a simple button to print the shipping label from my phone.
**Research Report**:
- Evaluated Tools: EasyPost, Shippo, ShipEngine.
- Ease of Use: EasyPost offers a very clean REST API that normalizes dozens of carriers (USPS, UPS, FedEx, DHL).
- Pricing: EasyPost charges pennies per label and gives access to discounted USPS rates (Commercial Plus), which is a huge benefit for small businesses.
- Reputation: Reliable, high uptime, developer-friendly.
- Environment: Cloud and Standalone.
- Recommendation: Integrate EasyPost to handle rate calculation at checkout and label generation in the Operations dashboard.
**Design Doc**:
- **Integration Flow**: User inputs their package dimensions and weight in the Product settings.
- **Actions**:
  - *Checkout*: OHC sends the cart items and destination address to EasyPost, retrieves real-time rates, and adds them to the customer's total.
  - *Fulfillment*: In the Operations tab, the user clicks "Buy Label". OHC buys the label via EasyPost and returns a PDF.
- **User Interface**: At checkout, a dynamic "Shipping" line item. In the manager app, an "Orders" screen with a prominent "Print Shipping Label" button for unfulfilled physical orders.
**Implementation Prompt**: Implement shipping rate calculation and label generation using EasyPost. Products must have weight/dimension fields. The checkout flow must dynamically fetch shipping rates based on the buyer's address. The Operations dashboard must allow the business owner to generate and download a printable PDF shipping label for paid orders. Acceptance criteria: Accurate rate calculation at checkout, successful generation of a test label, and marking the order as "Shipped" with a tracking number.
**Priority**: P2
**Estimated Scope**: Medium
