## 5. Shipping & Logistics: ShipStation

**Title:** Integrate ShipStation for Streamlined Order Fulfillment
**Problem Statement:** Calculating shipping rates, printing labels, and sending tracking numbers manually is tedious and error-prone for e-commerce sellers.
**Research Report:**
- **Tool evaluated:** ShipStation
- **What problem it solves for which persona:** Automates shipping tasks for product-based businesses shipping nationwide.
- **Ease of Use:** Powerful, but UI can be dense.
- **Pricing:** Starts at $9.99/month for 50 shipments.
- **Reputation:** The go-to solution for multi-carrier shipping.
- **Advantages & Risks:**
  - *Advantages:* Huge list of carrier integrations, discounted rates.
  - *Risks:* The setup process is involved; might be too complex for a seller who only ships 5 items a week.
- **Cloud/Standalone Mode:** Excellent API for Cloud. Standalone might struggle with local printer integrations without native desktop apps.
**Design Doc:**
- **Trigger:** An order is marked as 'Ready to Ship' in OHC.
- **Action:** OHC requests a shipping label from ShipStation and saves the tracking link.
- **User View:** An 'Orders' view with a 'Print Label' button, and automated emails sent to the customer with tracking info.
**Implementation Prompt:**
Build an integration that fetches live shipping rates during checkout based on weight/dimensions. Add a button in the order management view to generate and download a PDF shipping label.
**Priority:** P2
**Estimated Scope:** Large
