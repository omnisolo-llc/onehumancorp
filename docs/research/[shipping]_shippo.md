## 4. Shipping & Logistics: Shippo
**Problem Statement:** E-commerce or craft business owners waste significant time manually calculating shipping rates, writing labels, and waiting in line at the post office.
**Research Report:**
- **Tool:** Shippo
- **Persona Benefit:** Streamlines the entire shipping process, from rate calculation to label printing and tracking, saving time and money.
- **Key Advantages:** Connects with dozens of carriers (USPS, UPS, FedEx, DHL), offers discounted rates, and has a simple pay-as-you-go or low-cost monthly tier.
- **Risks:** Hardware integration (label printers) can sometimes be tricky for non-technical users.
- **Pricing:** Starter plan is free (pay only postage + 5¢ per label if using own accounts, or free if using Shippo's discounted rates). Pro plan starts at $17/month.
- **Environment:** Cloud.
**Design Doc:**
- **Trigger:** A customer places a physical order requiring shipping.
- **Action:** Automatically fetches the best shipping rates based on package dimensions/weight and generates a printable label.
- **User View:** The business owner sees a "Fulfill Order" button that presents shipping options, allows them to print a label, and automatically emails the tracking number to the customer.
**Implementation Prompt:** Integrate Shippo to provide shipping rate calculation and label generation. The owner should be able to input package details, select a carrier rate, and print the shipping label from their order management dashboard.
**Priority:** P2
**Estimated Scope:** Medium
