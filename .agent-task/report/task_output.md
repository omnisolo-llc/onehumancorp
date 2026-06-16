issue_title: "Implement Instant Localized Invoicing Architecture"
issue_description: |
  # Research Report: Instant Localized Invoicing Architecture

  ## Executive Summary
  This report details an architectural blueprint for a distributed, instant invoicing system designed for OHC merchants. The system aims to solve the gap where SMBs operate across multiple regions, needing immediate invoice generation that is localized (currency, taxes, languages) while maintaining a globally consistent financial ledger.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **Shopify & Wix**: Provide invoicing, but it is often deeply coupled to the storefront logic and struggles with rapid multi-region generation without complex plugins.
  - **Stripe Invoicing**: Highly robust but presents a learning curve and configuration overhead that intimidates micro-merchants.
  - **OHC Gap**: OHC currently lacks a dedicated, instant invoicing generation and edge-caching layer capable of serving fully localized, visually appealing invoices instantly to customers worldwide while keeping the core PostgreSQL ledger strongly consistent.

  ## 2. OHC Gap & Pain Point Identification
  - **Persona Focus**: Nora (agency principal) and Priya (boutique operator) who need to send invoices instantly to international clients.
  - **The Gap**: Waiting for centralized server rendering for invoices introduces latency and potential bottlenecks during peak sales. Furthermore, generating PDFs/HTML on the fly for every request impacts server resources.

  ## 3. Deep Dive Architecture Design

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL)**: Stores the canonical state of invoices, line items, and tax configurations.
  - **Edge Document Cache (Valkey/Redis)**: Pre-rendered or rapidly-rendered HTML/JSON invoice representations are cached at the edge.
  - **Localization Engine**: An agent-driven service that translates line items and calculates localized taxes before caching.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Operations Agent] -->|Generates Draft| B(Invoice Engine)
      B --> C[PostgreSQL Ledger]
      B --> D[Localization & Tax Service]
      D --> E{Edge Cache - Valkey}
      E -->|Serves Instant Invoice Link| F[Customer Browser/Mobile]
      F -->|Pays via Stripe| G[Payment Webhook]
      G --> C
      G --> H[Finance Agent]
  ```

  ### AI Agent Coordination
  - **Finance Agent ("The Accountant")**: Monitors incoming payments, reconciles them with the PostgreSQL ledger, and marks invoices as paid.
  - **Operations Agent ("The Manager")**: Automatically triggers invoice generation based on approved proposals or completed tasks.

  ### Mobile-First Implementation
  - Invoices must be beautifully rendered in HTML, fully responsive, and optimized for 375px viewports.
  - "Pay Now" buttons must be sticky and highly visible on mobile.

  ## 4. Implementation Prompt
  **User-Facing Outcome**: As Nora, when I complete a project phase, the Operations Agent drafts an invoice. Once approved, the client receives a link that instantly loads a localized, mobile-optimized invoice page, regardless of their global location.

  **CUJ & Acceptance Criteria**:
  1. A background job (simulated Operations Agent) creates an invoice record in the database.
  2. The system triggers the Localization Engine to prepare the invoice data.
  3. The rendered invoice data is pushed to the Edge Cache (Redis/Valkey).
  4. A customer accesses the invoice via a unique public link, and the response is served instantly (<50ms) from the cache.
  5. Provide Playwright E2E tests validating the public invoice view on a 375px viewport and the caching mechanism.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
