# [operations] Local Pickup and Order Printing

## Title
Retail-Ready Local Operations: Pickup & Thermal Printing

## Problem Statement
Fatima (Food Cart, 50) and Priya (Boutique, 35) both deal with high volumes of local customers. Fatima needs to know exactly when a customer is arriving so she can have the food hot. Priya needs a way to print a simple "packing slip" or "order ticket" to a small thermal printer without needing a complex POS system or a laptop.

## Research Report
- **Competitor Audit**:
    - **Square Online**: Best in class for pickup, but expensive hardware requirements.
    - **Shopify**: "Local Pickup" is a secondary feature; printing requires "Shopify POS" which is overkill for a small cart or boutique.
    - **UberEats / DoorDash**: High fees (20-30%) for simple pickup orders.
- **Data**: 64% of consumers prefer to buy online and pick up in-store (BOPIS) to save on shipping.
- **Evidence**: Fatima needs "Loud Notifications" and "Physical Tickets."

## Design Doc
- **Architecture**:
    - Integration with standard thermal print protocols (ESC/POS) over Bluetooth/WebUSB.
    - `PickupOrder` entity with `EstimatedArrivalTime`.
- **Mobile Flow (375px)**:
    - Notification: "New Pickup Order: Burger & Fries (Fatima, 12:30 PM)."
    - Dashboard Button: [Print Ticket] -> Sends to Bluetooth printer.
- **AI Agent Integration**:
    - "The Manager" agent monitors the store's "Busy-ness" and automatically adjusts the "Pickup Lead Time" on the storefront (e.g., changing from 15 min to 30 min during a rush).

## Implementation Prompt
Build a local operations module focused on pickup and physical fulfillment. Implement a native thermal printing bridge (WebUSB/Bluetooth) to allow merchants to print order tickets directly from the mobile dashboard.
- **Critical User Journey**: Customer orders for pickup -> Fatima gets a loud alert on her phone -> She taps "Print" -> Small ticket prints at her cart -> She marks "Ready" -> Customer gets an SMS.
- **Acceptance Criteria**: Pickup time selection at checkout. Native printing support for thermal printers. AI-managed lead times based on order volume.
- **Priority**: P1
- **Estimated Scope**: Medium
