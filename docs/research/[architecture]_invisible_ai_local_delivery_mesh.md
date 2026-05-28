# [Architecture] Invisible AI Local Delivery & Logistics Mesh

## 1. Title
Invisible AI Local Delivery & Logistics Mesh

## 2. Problem Statement
**The Small Business Reality:**
Maya (custom cakes) and Fatima (food cart) have huge demand for local delivery, but managing it is a nightmare.
Currently, their options are:
1. **Third-party marketplaces (UberEats, DoorDash, Grubhub)**: These platforms charge 15-30% commissions, destroying their thin margins, and they steal the customer relationship.
2. **Managing their own drivers**: Too expensive, complex routing, insurance liabilities, and unreliable.
3. **Manual courier dispatching**: Using services like Uber Direct or DoorDash Drive manually requires jumping between apps, copy-pasting addresses, manually calculating delivery fees, and fielding phone calls from lost drivers.

**The Non-Technical User Pain Point:**
Maya just wants to toggle a switch that says "Offer Local Delivery." When a customer buys a cake, she wants a courier to magically appear at the right time to pick it up, and the customer to receive tracking links via SMS, without Maya ever leaving her OHC dashboard or typing an address into a courier app.

## 3. Research Report
**Market Context & Competitor Gaps:**
- **Shopify / Square**: They offer local delivery settings (radius, zip codes) but mostly leave fulfillment to the merchant, or require third-party apps (like Zapiet or ShipBob) which are too complex for our micro-merchant personas to configure.
- **Wix**: Basic local delivery zones, but no native, invisible third-party courier dispatching without apps.
- **Delivery as a Service (DaaS)**: Uber Direct, DoorDash Drive, Relay, Nash, and Stuart offer API-driven white-label delivery. They provide the drivers, the merchant keeps the customer and pays a flat fee per delivery (e.g., $6-$9).

**Opportunity for OHC:**
By integrating DaaS providers invisibly into the OHC order flow, we can provide immediate, white-labeled local delivery to every OHC merchant out-of-the-box. We can use AI to dynamically quote delivery fees at checkout, automatically dispatch the best/cheapest courier based on size/time, and use our AI Inbox to intercept and resolve courier issues (e.g., driver texting "gate code?") without waking up the merchant.

## 4. Design Doc
### Architecture Diagram
```mermaid
erDiagram
    MERCHANT ||--o{ DELIVERY_ZONE : defines
    DELIVERY_ZONE ||--o{ ORDER : matches
    ORDER ||--|| DELIVERY_QUOTATION : generates
    ORDER ||--|| DISPATCH_JOB : triggers

    DISPATCH_JOB ||--o| COURIER_NETWORK (Uber_Direct) : routes_to
    DISPATCH_JOB ||--o| COURIER_NETWORK (DoorDash_Drive) : routes_to

    DISPATCH_JOB }|--|| AI_LOGISTICS_AGENT : monitored_by
    AI_LOGISTICS_AGENT ||--o{ AI_INBOX : resolves_driver_messages
    AI_LOGISTICS_AGENT ||--o{ SMS_NOTIFICATIONS : updates_customer

    CUSTOMER ||--o{ ORDER : places
```

### UI Wireframes & Mobile UX Flow (375px First)

**Screen 1: Delivery Setup (Merchant App - Settings)**
- macOS-style Translucent Glass card layout.
- **Toggle:** "Offer Local Delivery" (Enabled).
- **Map View:** A clean map showing the store location.
- **Radius Slider:** A simple slider to set delivery radius (e.g., "5 miles").
- **Fee Setting:**
  - Option A: "Pass delivery cost to customer" (AI calculates live at checkout).
  - Option B: "Flat fee" (e.g., $5).
- **No API keys or courier selection.** OHC handles the routing behind the scenes.

**Screen 2: Checkout (Customer View)**
- Customer enters their address.
- Seamless, Apple Pay-style bottom sheet checkout.
- If within radius, "Local Delivery (Powered by StoreName)" appears as a shipping option.
- Dynamic fee displayed.

**Screen 3: Order Management (Merchant App - Active Orders)**
- Maya taps an active order: "Vegan Chocolate Cake - Delivery at 3 PM".
- **Status:** "Courier arriving in 15 mins (Uber Direct)".
- Action buttons: "Print Receipt" (thermal), "Mark Ready for Pickup".

### AI Agent Integration Points
- **AI Quoting Engine (Finance Dept)**: Calculates real-time courier quotes across multiple DaaS APIs during checkout to present the best price.
- **AI Logistics Dispatcher (Operations Dept)**: Automatically calls the courier API (Uber/DoorDash) when the merchant marks the item "Ready" or based on predictive prep times.
- **AI Inbox (CS Dept)**: If the courier texts "I'm outside but don't see the house," the AI intercepts the SMS, looks at customer notes, and replies "It's the blue house with the red door, leave on porch" without bothering Maya.

### Key Design Decisions
- **Zero Configuration**: We abstract away the specific courier networks. Merchants do not create Uber Direct or DoorDash Drive accounts. OHC holds the master accounts and acts as the marketplace router.
- **Predictive Dispatching**: For food (like Fatima's cart), dispatching a driver too early results in cold food; too late results in angry customers. The system will use ML on average prep times to schedule the driver pickup precisely.
- **Unified Liability**: OHC handles driver disputes invisibly.

## 5. Implementation Prompt
**Context:** Implement the Invisible AI Local Delivery Mesh.
**User Journey:** Maya toggles "Local Delivery" and sets a 10-mile radius. A customer buys a cake and selects delivery. The system calculates a live quote using DaaS APIs. When Maya marks the cake "Ready," the system automatically dispatches a white-label courier to her store, tracks the driver, and texts the customer the tracking link.
**Acceptance Criteria:**
1. Create a `DeliveryZone` and `DeliveryQuotation` data model with strict multi-tenant isolation.
2. Implement a unified interface/adapter for routing to multiple DaaS providers (e.g., Uber Direct, DoorDash Drive).
3. Build the backend logic to generate real-time delivery quotes during checkout based on distance and payload size.
4. Implement the AI Operations Agent trigger to dispatch a courier when an order status changes to "Ready for Pickup".
5. Ensure mobile-first API responses for the Merchant settings UI (radius slider, fee preference).
6. Do NOT prescribe specific DB schemas in this prompt, just ensure the models support multi-tenancy and the described UX.

## 6. Metadata
- **Priority**: P1 (High)
- **Estimated Scope**: Large
