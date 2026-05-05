# Title
Shipping & Logistics: EasyPost for Automated Label Generation

# Problem Statement
For businesses selling physical goods (like Maya's custom cakes or Priya's boutique), manually going to the post office or typing addresses into USPS/FedEx websites is incredibly time-consuming. They need a way to automatically calculate shipping rates at checkout and print labels directly from the OHC app.

# Research Report
**Tool Analyzed:** EasyPost
EasyPost provides a single API to access dozens of carriers (USPS, FedEx, UPS, DHL, etc.) for rating, shipping, and tracking.
- **Ease of Use (for non-technical users):** EasyPost is an API, but it allows OHC to build a one-click "Print Label" button in the order dashboard.
- **Pricing:** Very cheap per label (pennies), making it feasible for OHC to absorb the cost or pass it through invisibly.
- **Reputation:** Highly reliable, industry standard for modern e-commerce logistics.
- **Integration Risk:** Low. The API is clean. The main complexity is handling physical package dimensions and weights accurately.
- **Cloud/Standalone:** Cloud API, fits perfectly.

# Design Doc
- **Trigger:** A customer adds a physical product to their cart.
- **Actions:**
  1. OHC pings EasyPost with the cart contents (weight/dimensions) and destination address to get real-time shipping rates.
  2. Customer selects a rate and pays.
  3. When the merchant is ready to fulfill, they click "Generate Label" in the OHC Operations dashboard.
  4. OHC calls EasyPost to purchase the label and returns a printable PDF to the user.
  5. EasyPost webhooks update the order status as it moves through the mail network, and the Customer Success AI emails the customer.
- **User Experience:** The merchant never leaves OHC. They click one button, a label prints from their phone or computer, and the tracking emails are sent automatically.

# Implementation Prompt
Integrate EasyPost to provide real-time shipping rates at checkout and one-click label purchasing in the merchant dashboard. Acceptance criteria include successful rate calculation based on mock package dimensions, generation of a valid test PDF shipping label via the API, and automated tracking status updates processed via webhooks.

# Priority
P1

# Estimated Scope
Medium
