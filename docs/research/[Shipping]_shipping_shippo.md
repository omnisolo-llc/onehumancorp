**Title**: Shipping & Logistics Integration via Shippo

**Problem Statement**:
Small e-commerce businesses (like Maya's bakery shipping cookies) struggle with calculating accurate shipping rates, buying postage, and generating tracking numbers. Managing this manually across different carrier websites (USPS, FedEx, DHL) is error-prone and time-consuming. They need an automated way to get rates and print labels directly from their order dashboard.

**Research Report**:
Shippo is a multi-carrier shipping API that simplifies logistics.
- **Ease of Use for Non-Technical Users**: Excellent. Shippo offers "Shipping Elements," which are embeddable UI widgets that allow users to purchase labels without leaving OHC.
- **Features**: Connects to 40+ carriers globally. Handles address validation, rate comparison, label generation, multi-parcel shipments, and tracking webhooks.
- **Reputation & Reliability**: Very reliable, widely used by major e-commerce platforms.
- **Pricing**: Pay-as-you-go model. $0.05 per label plus the cost of postage for the starter tier, making it very accessible for small businesses.
- **Cloud vs Standalone**: API-based, works well in both. Webhook delivery for tracking updates will require a public endpoint, necessitating a relay for Standalone users behind strict firewalls.

**Design Doc**:
- **Trigger**: An order is placed and marked as "Ready to Ship" in OHC.
- **Action**: OHC passes the origin and destination addresses, plus parcel dimensions, to the Shippo API to fetch rates.
- **User View**: The business owner clicks "Create Label" on an order. They see a popup (Shippo Shipping Element) comparing rates from different carriers. They select one, purchase it, and a printable PDF label is generated. The customer automatically receives a tracking link.
- **Architecture**: OHC integrates the Shippo API and UI Elements. We will need to store package dimensions/weights. A webhook listener must be implemented to receive tracking status updates (`TRANSIT`, `DELIVERED`) and update the OHC order status accordingly.

**Implementation Prompt**:
Integrate the Shippo API to enable in-platform shipping label generation. Implement address validation for customer shipping addresses. Add a "Fulfill Order" button to the order details page that opens the Shippo UI to compare rates and purchase a label. Automatically save the generated tracking number to the order and notify the customer.

**Priority**: P1 (high)
**Estimated Scope**: Medium
