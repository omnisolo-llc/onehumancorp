issue_title: "Implement Universal Multi-Tenant Dynamic Catalog & Variant Rules Engine"
issue_description: |
  **Problem Statement**
  Small business owners offer extremely diverse products and services, yet most platforms force them into rigid "E-commerce Product" or "Service Booking" data models.
  - **Priya (boutique owner)** needs physical products with size/color variants, tracked inventory, and shipping rules.
  - **Fatima (food cart)** needs food items with simple "sold out" toggles, add-ons (extra sauce), and no shipping.
  - **Carlos (handyman)** needs service listings with variable pricing (e.g., hourly vs. flat rate) and deposit requirements.
  - **Maya (baker)** needs custom photo catalogs where items require custom quote requests and percentage-based deposits rather than instant checkout.

  Currently, OneHumanCorp (OHC) lacks a unified, multi-tenant catalog architecture that can instantly adapt to these fundamentally different business models from a single, simple, mobile-first interface without forcing the user to understand complex configuration options.

  **Research Report**
  *Competitive Analysis*
  *   **Shopify:** Excellent for Priya (variants, physical inventory), but terrible for Carlos (services require complex workarounds or apps). Rigid data model focused purely on physical/digital retail.
  *   **Wix/Squarespace:** Offers different "apps" for Stores vs. Bookings vs. Restaurants, creating fragmented silos. A user can't easily sell a physical t-shirt, a digital tutorial, and an in-person workshop from the same unified catalog manager.
  *   **Square:** Good offline/online parity, but variants can become messy, and quote/deposit flows for custom orders are clunky.

  *OHC Opportunity*
  OHC must provide a **Polymorphic Catalog Entity** that fluidly changes its behavior (and AI operational handling) based on the business type. The business owner simply says, "I want to sell Vegan Chocolate Cake," and the AI dynamically attaches the correct variant structure (size, custom writing), fulfillment type (pickup/delivery), and payment rules (deposit vs. full payment) without exposing a complex database schema to the user.

  **Next Steps (Implementation Prompt)**
  Implement the backend APIs and database schema for the Universal Dynamic Catalog Engine. Your implementation must allow a single Tenant to create items of fundamentally different types (e.g., a physical product with size variants and tracked inventory, alongside a custom service requiring a 50% deposit and no inventory) within the same catalog structure.

  The API must support creating an item, its variant groups, options, fulfillment rules, and payment rules in a single transactional request to support AI-driven generation. Ensure strict multi-tenant isolation (RLS or equivalent) so tenants can only access their own catalog. Do not prescribe the specific UI framework, but ensure the payload structure is clean enough to map directly to mobile-first UI components.

  See full design doc at `docs/research/[architecture]_multi_tenant_dynamic_catalog_variant_engine.md` for Mermaid ER diagrams, UX flow constraints, and AI Integration Points.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
