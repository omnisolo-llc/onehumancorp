# Automated Shipping & Label Generation

**Problem Statement:**
For small businesses selling physical goods, calculating shipping rates manually and copy-pasting addresses into carrier websites to buy labels is a massive time sink. Mistakes lead to lost packages or angry customers. They need a system that automatically calculates shipping at checkout and lets them print a label with one click.

**Research Report:**
- **Evaluated Tools:** Shippo API, EasyPost API.
- **Ease of Use:** Exceptional. OHC abstracts the carrier complexity. The user just clicks "Buy Label" and prints a PDF.
- **Pricing:** Usually pennies per label (e.g., $0.05) plus the cost of postage.
- **Reputation:** Both EasyPost and Shippo are highly reliable and support dozens of carriers globally (USPS, UPS, FedEx, DHL, etc.).
- **Cloud vs Standalone:** Works perfectly in both environments via standard API calls.

**Design Doc:**
- **Trigger:** A customer places an order.
- **Action:** The business owner views the order details and clicks "Generate Shipping Label." The integration hits the Shippo/EasyPost API to buy the postage and returns a printable PDF.
- **User Interface:** A simple "Print Label" button on the order detail page. A settings page where the owner inputs their box sizes and origin address.

**Implementation Prompt:**
Integrate a shipping API (like Shippo or EasyPost) to allow business owners to generate and print shipping labels directly from an order's detail page. The system must automatically pull the customer's shipping address, purchase the correct postage based on predefined box sizes, and provide a downloadable PDF label. It must also automatically add the tracking number to the order and notify the customer.

**Priority:** P2
**Estimated Scope:** Medium
