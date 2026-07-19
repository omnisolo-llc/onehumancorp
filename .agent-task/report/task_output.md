issue_title: "[Architecture] Distributed Edge-Cached Storefronts & Invoicing Engine"
issue_description: |
  # [Architecture] Distributed Edge-Cached Storefronts & Invoicing Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) need a highly performant, globally available storefront and instantly generated localized invoicing. A single monolithic API or database bottleneck prevents seamless customer checkout and quote generation when mobile reception drops or high traffic spikes happen, hurting the non-technical owner's business. We lack edge-caching for storefront reads and a localized offline-first invoice/quote generation engine integrated via agents.

  ## Research Report
  - **Codebase & Docs Audit**: The codebase utilizes an API Gateway approach, but lacks explicit edge-caching middleware specifically tailored to storefront product variants and CRDT-based offline quote mutations.
  - **Competitor Analysis**: Shopify leverages aggressive CDN caching for product catalogs. Square and Stripe offer robust invoicing, but separate these from the CRM. We need them seamlessly integrated via the KAIROS AI OS.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Devices
          CustomerApp[Customer Storefront Web] --> EdgeCache[Edge CDN Cache];
          OwnerApp[Owner App - Offline First 375px] --> LocalDB[(CRDT Local State)];
      end

      EdgeCache --> ApiGateway[OHC API Gateway];
      LocalDB --> ApiGateway;

      ApiGateway --> InvoiceEngine[Instant Invoicing & Quotes];
      ApiGateway --> CatalogEngine[Product Catalog Service];

      InvoiceEngine --> DistributedDB[(Cloud Tenant-Isolated DB)];
      CatalogEngine --> DistributedDB;

      InvoiceEngine -.-> FinanceAgent[Finance & Billing AI];
  ```

  ### Mobile UX Flow (375px)
  - The customer views a fast-loading edge-cached product page.
  - The owner drafts an invoice seamlessly on their phone even in low connectivity. Changes sync optimisticially using CRDTs to the master database upon reconnection.

  ### AI Agent Integration
  - **Finance Agent**: Observes invoice creation via the unified event queue and autonomously calculates localized tax rules and sends SMS payment reminders.

  ### Security and Isolation
  - Zero-Trust Multi-Tenancy: SPIFFE/SPIRE SVIDs enforce tenant boundaries for every catalog and invoice read/write.

  ## Implementation Prompt
  Implement the distributed edge-caching layer for the public storefronts and the local-first CRDT-backed Invoicing Engine.
  - **User-Facing Outcome**: Customers load product catalogs instantly. Owners can generate and send localized invoices instantly, even with spotty connectivity.
  - **CUJ**: An owner drafts a quote offline. Once connected, it syncs to the central database, triggering the Finance agent to calculate tax and prepare the final localized invoice.
  - **Acceptance Criteria**: The catalog must hit the edge cache. Invoices must sync from a local state. Ensure strict row-level multitenancy, and include Playwright 375px tests. Do not prescribe specific schema details—allow the implementer to finalize the CRDT model.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
