# Automate Shipping Rates and Labels with EasyPost

**Problem Statement**
Calculating shipping costs for different carriers and manually printing labels takes hours every week. I need a way to automatically get the best shipping rates and print labels with one click when an order comes in.

**Research Report**
EasyPost provides a unified API for dozens of carriers (USPS, UPS, FedEx, DHL). It allows users to compare rates and generate tracking numbers seamlessly. For small business owners, this removes the need to negotiate with individual carriers. Pricing is very friendly, often free for up to 120,000 shipments/year, only charging postage. It is highly reliable and fits perfectly into Cloud and Standalone modes.

**Design Doc**
When an order is marked 'Ready to Ship' in OHC, the user will be presented with a list of shipping rates from different carriers. They can select the preferred rate and click 'Buy Label'. OHC will generate a printable PDF of the shipping label and save the tracking number to the order.

**Implementation Prompt**
Implement an EasyPost integration for order fulfillment. When viewing an order, the user should be able to fetch shipping rates based on package weight and destination. The user must be able to purchase a label, which should return a printable PDF and a tracking number. Acceptance criteria: Correct rates are displayed, and a test label can be generated and viewed.

**Priority:** P1
**Estimated Scope:** Large
