# Scout Tool Integration Research: Shipday for Local Delivery

## Title
Integrate Shipday for Autonomous Local Delivery Management

## Problem Statement
Small businesses offering local delivery (like Fatima's food cart, Maya's custom cakes, or Carlos the Handyman's parts runs) struggle with managing delivery routes, tracking drivers, and keeping customers updated. Competitor platforms rely on 3rd-party marketplaces (like UberEats or DoorDash) that charge massive margins (15-30%) and own the customer relationship, or they require the business owner to manually coordinate "Uber Direct" or "DoorDash Drive" APIs. Business owners need a simple, zero-configuration way to dispatch their own staff or local couriers efficiently.

## Research Report
Shipday is a delivery management software explicitly built for restaurants, meal prep services, grocery stores, and other small businesses offering local delivery.

- **Relevance:** It solves the exact pain point identified in our `[architecture]_autonomous_multi_modal_local_delivery_mesh.md` and `[architecture]_invisible_ai_local_delivery_mesh.md` design docs by providing a unified dispatch system.
- **Capabilities:** It offers route optimization, real-time driver tracking, SMS notifications to customers, and proof of delivery. Crucially, it supports assigning orders to internal staff (the OHC Staff Mesh) OR bridging to 3rd-party services (like Uber/DoorDash) when staff is unavailable.
- **SMB Fit:** Shipday is designed for local operations. A non-technical owner like Fatima doesn't need to understand routing algorithms; the system just tells her driver where to go, and the customer gets an SMS tracking link.

## Design Doc: OHC + Shipday Integration

The Shipday API will act as the fulfillment engine for the "Local Delivery" option in OHC.

1.  **Trigger:** An order is placed with "Local Delivery" selected, or the Operations AI Agent determines a local dispatch is required.
2.  **Action:** The OHC Operations Agent creates an order in Shipday via API.
3.  **Routing (Invisible to Owner):** Shipday optimizes the route and assigns it to the available local staff member (via the Shipday Drive App or an OHC integrated view).
4.  **Customer Experience:** Shipday automatically sends a branded SMS to the customer with a live tracking link.
5.  **Completion:** Upon delivery, proof (photo/signature) is captured, and the OHC Global Inventory Ledger and Order Status are updated via Shipday webhook.

## Implementation Prompt
Implement the Shipday integration within the OHC platform.

**Acceptance Criteria:**
- Add a "Connect Shipday" option in the integrations dashboard.
- When an order is created in OHC with "Local Delivery", it should automatically create a delivery task in Shipday.
- The business owner should see the live delivery status (e.g., "Assigned to Driver", "In Transit", "Delivered") updated on the OHC order detail view, powered by Shipday webhooks.
- The integration must handle the OAuth/API Key connection securely for the specific tenant.

## Priority
P1

## Estimated Scope
Medium
