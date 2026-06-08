# OHC Autonomous Unified Booking & Revenue Engine

## 1. Sequence Diagram

```mermaid
sequenceDiagram
    participant Customer
    participant OwnerApp as Owner (Mobile App)
    participant OHC as OHC Booking Engine
    participant OpsAgent as Operations Agent
    participant CSAgent as Customer Success Agent

    Customer->>OHC: Books Service & Pays Deposit
    OHC->>OpsAgent: Update Calendar & State
    OpsAgent->>OpsAgent: Run Nightly Dormant Analysis
    OpsAgent->>CSAgent: Trigger: "Sarah missed regular slot"
    CSAgent-->>OHC: Draft check-in message & magic link
    OHC->>OwnerApp: Push Notification: "Approve check-in for Sarah?"
    OwnerApp->>OHC: Tap "Approve"
    OHC->>Customer: Send SMS/Email
```

## 2. Entity-Relationship (ER) Diagram

```mermaid
erDiagram
    TENANTS ||--o{ CUSTOMERS : "owns"
    TENANTS ||--o{ SERVICES : "offers"
    TENANTS ||--o{ AVAILABILITY_BLOCKS : "defines"
    TENANTS ||--o{ BOOKINGS : "manages"
    TENANTS ||--o{ OHC_UNIVERSAL_LEDGER : "records"

    CUSTOMERS ||--o{ BOOKINGS : "makes"
    SERVICES ||--o{ AVAILABILITY_BLOCKS : "has"
    PRODUCTS ||--o{ BOOKINGS : "reserved_via"

    BOOKINGS {
        string id PK
        string tenant_id FK
        string customer_id FK
        string product_id FK
        timestamp start_time
        timestamp end_time
        string status "pending, pending_payment, confirmed, completed, cancelled"
        string payment_intent_id
    }

    SERVICES {
        string id PK
        string tenant_id FK
        string title
        string description
        bigint price_cents
    }

    AVAILABILITY_BLOCKS {
        string id PK
        string tenant_id FK
        string product_id FK
        timestamp start_time
        timestamp end_time
        boolean is_available
    }
```
