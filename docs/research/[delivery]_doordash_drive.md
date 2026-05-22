## [Delivery] DoorDash Drive API Integration

**Title**: Enable White-Label Local Delivery via DoorDash Drive API

**Problem Statement**:
Small business owners—such as local bakers, florists, and independent retailers—lose sales to massive aggregators because they cannot offer same-day, on-demand delivery without managing their own fleet of drivers. Setting up an in-house delivery team is expensive, logistically complex, and highly unreliable for a small operation. They need a way to offer "buy directly from us and get it delivered today" without sacrificing 30% of their revenue to marketplace commissions.

**Research Report**:
- **Market Demand:** Consumers increasingly expect same-day and on-demand delivery for local goods, not just food but retail as well.
- **Competitor Landscape:** Shopify and Wix rely on third-party app extensions for local delivery (like Zapiet), which can be complex to set up. A native, zero-configuration local delivery option is a major competitive advantage.
- **Tool Capabilities (DoorDash Drive):** DoorDash Drive is a white-label fulfillment API. It allows businesses to request a "Dasher" to deliver an order placed on their own website. The customer never has to use the DoorDash app.
- **SaaS Viability:**
  - *Cloud (Multi-tenant):* Can be seamlessly integrated via an OHC master DoorDash Drive account, abstracting setup complexity from the user. OHC can pass the flat delivery fee directly to the consumer at checkout.
  - *Standalone (Local/Private):* Users can supply their own DoorDash Drive API Developer credentials.
- **Pricing:** DoorDash Drive charges a flat fee per delivery (typically $7-9 depending on distance), rather than a percentage commission. This is vastly superior for SMB margins compared to marketplace listings.
- **Ease of Use:** From the SMB's perspective, this should act like a simple toggle: "Enable Local Delivery."

**Design Doc**:
- **Trigger/Setup:** In the "Fulfillment" settings of the OHC dashboard, users toggle "Local Delivery" on. In Cloud mode, they simply set their delivery radius and whether they want to subsidize the fee. In Standalone mode, they input their API credentials.
- **User Experience (SMB Owner):**
  - The SMB owner sees new orders in their OHC dashboard marked for "Delivery."
  - They can click "Ready for Pickup" to dispatch a driver, or have it automated based on item prep times.
  - The dashboard shows real-time Dasher tracking status.
- **Customer Experience:** At checkout on the SMB's OHC storefront, the customer sees "Same-Day Local Delivery" as an option and pays the flat delivery fee. After ordering, they receive SMS updates with a tracking link for their Dasher.
- **Actions:**
  - Automated dispatch of drivers via the Drive API.
  - Webhook listener for real-time driver status updates to reflect in the OHC dashboard.

**Implementation Prompt**:
Integrate the DoorDash Drive API to power white-label local delivery. Build the checkout flow extension to calculate delivery feasibility based on store radius and dynamically add the flat-rate delivery fee. Ensure the OHC order management dashboard surfaces Dasher tracking status (e.g., Driver Assigned, At Store, Out for Delivery, Delivered). Create the setup flow where users can enable Local Delivery (handling both Cloud managed accounts and Standalone custom API key configurations). Implement automated triggers that dispatch the delivery request to DoorDash based on the business's configured prep time or a manual "Request Driver" button click.

**Priority**: P1

**Estimated Scope**: Medium
