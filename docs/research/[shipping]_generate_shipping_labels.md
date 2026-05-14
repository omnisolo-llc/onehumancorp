# Generate Shipping Labels Without Leaving the App

**Problem Statement:** David (an online crafts seller) hates copying and pasting customer addresses into his local post office website to buy shipping labels. He wants to click one button to buy and print a label right after making a sale.

**Research Report:** Shippo and EasyPost aggregate multiple carriers (USPS, FedEx, UPS, local carriers) into a single API. Shippo is very small-business friendly with pay-as-you-go pricing and discounted rates. Non-technical users understand "buy a label".

**Design Doc:** On an Order page, the user sees a "Buy Shipping Label" button. OHC calls the Shippo API to get rates based on the package weight. The user selects a rate, and OHC generates a printable PDF label and automatically emails the tracking number to the customer.

**Implementation Prompt:** Integrate a shipping provider to fetch live rates for an order. Add a UI to select a rate, purchase the label, and display the resulting PDF for printing. Auto-update the order status to shipped and store the tracking number.

**Priority:** P1

**Estimated Scope:** Medium
