# Shipping and Logistics Integration

**Problem Statement:**
E-commerce small businesses struggle with manual fulfillment: copying addresses, logging into carrier websites, guessing package weights, and manually emailing tracking numbers to customers.

**Research Report:**
* **Tool Evaluated:** Shippo API
* **Ease of Use:** Abstracting multiple carriers into one API allows OHC to provide a "1-Click Ship" button.
* **Pricing:** Pay-as-you-go ($0.05 per label) plus actual postage. Very SMB friendly.
* **Reputation:** Highly reliable with broad carrier support globally.
* **Hybrid Context:** Cloud and Standalone modes can utilize the API identically, relying on webhooks for tracking state updates.

**Design Doc:**
* **Trigger:** An order is marked "Ready to Fulfill."
* **Action:** OHC requests shipping rates, allows the owner to select one, generates a printable PDF label, and sends the tracking number to the customer.
* **User Experience:** The owner opens a new order, verifies the package size, clicks "Buy Label," and a PDF pops up to print. The customer automatically receives an email saying "Your order shipped."

**Implementation Prompt:**
Build a fulfillment flow in the Order Management view. Allow the user to input package dimensions/weight, fetch live shipping rates, purchase a shipping label, and generate a printable PDF. Ensure the system automatically updates the order status to "Shipped" and attaches the tracking number.

**Priority:** P2
**Estimated Scope:** Large
