issue_title: "Architectural Design: Autonomous Localized Shipping & Fulfillment Engine"
issue_description: |
  # Research Report: Autonomous Localized Shipping and Fulfillment Engine

  ## Discovery
  An audit of competitor systems (Shopify, Wix) reveals that fulfillment setup (shipping zones, calculating rates, generating labels) is one of the highest friction points for SMBs, often leading to onboarding abandonment. OHC currently lacks an autonomous, zero-config engine that dynamically calculates the best fulfillment method (local delivery, shipping, pickup) at checkout and automatically generates the necessary artifacts (labels) post-purchase.

  ## Proposed Solution
  Implement the Autonomous Localized Shipping and Fulfillment Engine. This system will:
  1. Dynamically calculate optimal fulfillment options at checkout based on distance and tenant profile.
  2. Automatically interface with carrier APIs to purchase and generate shipping labels post-payment.
  3. Integrate seamlessly with the Universal Offline Thermal Print Mesh to print labels instantly without merchant intervention.

  ## Design Doc
  ### Architecture Diagram (ER)
  ```mermaid
  erDiagram
      TENANT ||--o{ FULFILLMENT_PROFILE : configures
      FULFILLMENT_PROFILE ||--o{ SHIPPING_ZONE : contains
      ORDER ||--|| FULFILLMENT_METHOD : requires
      ORDER ||--o{ SHIPPING_LABEL : generates
      TENANT {
          string id PK
          string business_address
      }
      FULFILLMENT_PROFILE {
          string id PK
          string tenant_id FK
          boolean enable_local_delivery
          boolean enable_pickup
          boolean enable_shipping
      }
      ORDER {
          string id PK
          string tenant_id FK
          string customer_address
          string status
      }
      FULFILLMENT_METHOD {
          string id PK
          string order_id FK
          string type
          float cost
          string provider
      }
      SHIPPING_LABEL {
          string id PK
          string order_id FK
          string tracking_number
          string label_url
      }
  ```

  ### Key Design Decisions
  1. **Dynamic Checkout Calculation:** At checkout, the engine automatically calculates the distance between the `tenant_address` and `customer_address`. It presents local delivery if within radius, otherwise presents standard shipping rates.
  2. **AI Operations Agent Integration:** When an order is placed, the Operations Agent automatically interfaces with carrier APIs (via a broker) to purchase and generate the shipping label using the pre-negotiated OHC rates.
  3. **Thermal Print Mesh:** The generated label is automatically pushed to the `Universal Offline Thermal Print Mesh` to print instantly in the merchant's store/kitchen.

  ### Mobile-First UX Flow (375px)
  - **Merchant View:** Zero setup. The merchant simply receives an order card: *"Priya bought 3 items. Shipping label printed. Stick it on the box."*
  - **Buyer View:** Clean, translucent glass checkout. *"You are 3 miles away! We can deliver locally for $5, or you can pick it up for free."*

  ### Security & Multi-Tenancy
  - **Strict Isolation:** Label generation and address data must be strictly isolated by `tenant_id`.
  - **Zero Trust Routing:** Carrier API keys (if integrated directly by advanced merchants) must be securely stored in the secrets manager and never exposed to the frontend.

  ## Implementation Prompt
  **To the Implementer:**
  Build the Autonomous Localized Shipping and Fulfillment Engine.
  1. Implement the dynamic rate calculation service that determines the best fulfillment options (pickup, local delivery, shipping) based on the tenant's profile and the buyer's address.
  2. Create the backend service to automatically generate shipping labels via carrier APIs (using a unified interface) upon order payment.
  3. Integrate the label generation event with the Event Mesh so that it can trigger the Thermal Print Mesh for automatic printing.
  Ensure all data structures (`FulfillmentProfile`, `ShippingLabel`) strictly enforce multi-tenant isolation rules. Do not expose complex configuration UIs to the merchant; rely on sensible defaults and AI agent management.

  ## Estimated Scope
  **Large**

  ## Next Steps
  Implementer agents should review the architectural design and begin constructing the dynamic rate calculation service and the label generation pipeline.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
