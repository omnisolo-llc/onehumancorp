# Hyperlocal AI Fleet and Delivery Mesh

## Overview

The Hyperlocal AI Fleet and Delivery Mesh enables business owners to automatically dispatch and manage local deliveries. The AI Dispatcher groups pending orders into optimized routes based on proximity and assigns them to drivers. Drivers have a simple web/mobile app to view their route, navigate, and mark orders as delivered (including proof-of-delivery photos). Customers receive live ETA updates.

## DB Schema

```sql
CREATE TABLE IF NOT EXISTS delivery_tasks (
    task_id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT REFERENCES orders(id) ON DELETE CASCADE,
    driver_id TEXT, -- REFERENCES drivers(id) (or just driver name)
    status TEXT DEFAULT 'pending', -- pending, assigned, in_transit, delivered, failed
    estimated_eta TEXT,
    proof_of_delivery_url TEXT,
    route_poly TEXT,
    stop_order INT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS drivers (
    driver_id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    current_location TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

Add to `orders` table:
`delivery_address` TEXT

## Protobuf Definition
Need to add `DeliveryTask` and `Driver` and `DeliveryMeshService` to `app.proto` or a new `delivery.proto`.

## API Handlers
- `GetDeliveryDashboard`: lists unassigned orders, active routes, available drivers.
- `AutoAssignDeliveries`: AI agent groups orders into routes and assigns drivers.
- `DispatchRoute`: moves assigned tasks to `in_transit`, notifies drivers via SMS.
- `MarkDelivered`: Updates status and adds proof of delivery photo.

## UI Components
- Storefront Dashboard: Delivery Dashboard tab.
- Driver Web App: Simplified UI for drivers (accessible via a tokenized link).

## E2E Tests
1. Seed an org with local delivery orders.
2. Go to Delivery Dashboard.
3. Verify unassigned orders are shown.
4. Auto-assign and dispatch.
5. Emulate Driver UI to mark an order as delivered.
6. Verify Order status changes to delivered on Dashboard.
