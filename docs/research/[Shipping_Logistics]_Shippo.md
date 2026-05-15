**Title**: Shippo Integration for Automated Label Generation
**Problem Statement**: Fulfilling physical orders is a massive time sink. Business owners have to manually copy customer addresses into carrier websites, calculate rates, and paste tracking numbers back into emails.
**Research Report**: Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) and offers discounted rates. Its API is highly reliable, and its dashboard is friendly for small businesses. It has a pay-as-you-go model which is perfect for low-volume shippers.
**Design Doc**:
- **Trigger**: An order is marked as "Ready to Ship" in OHC.
- **Action**: OHC requests a shipping label from Shippo using the customer's address and package dimensions.
- **User Experience**: The business owner clicks "Print Label" on an order. OHC provides a PDF of the label and automatically emails the tracking number to the customer.
**Implementation Prompt**: Create a shipping fulfillment flow for orders. Allow the business owner to connect Shippo, input default box sizes, and generate a shipping label PDF directly from the order details page. Automatically extract the tracking link and display it to the user.
**Priority**: P2
**Estimated Scope**: Large
**Environment**: Works in both Cloud and Standalone modes.
