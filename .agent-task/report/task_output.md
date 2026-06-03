issue_title: "Implement Unified Payment Gateway Integration Architecture"
issue_description: |
  # Payment Gateway Integration Architecture Deep Dive

  ## Problem Statement
  The current platform requires an extensive update to support diverse payment modalities across our core user personas. As small business operators like Maya (baker), Carlos (handyman), and Priya (boutique owner) require flexible, instant, and localized payment options, a singular payment architecture might not adequately serve global and omnichannel needs. The lack of a robust, multi-tenant-aware, secure payment capability limits our ability to scale globally and provide zero-friction experiences.

  ## Research Report
  ### Current State & Competitor Analysis
  - **Shopify:** Utilizes Shopify Payments (powered by Stripe) as well as hundreds of third-party gateways. Highly flexible but configuration can be complex for non-technical users.
  - **Wix:** Wix Payments offers an integrated experience, abstracting much of the gateway complexity, yet limits advanced customization.
  - **Squarespace:** Seamless Stripe and PayPal integration, though primarily focused on digital/e-commerce rather than hybrid in-person and online.
  - **OHC Vision:** Requires a unified payment architecture that seamlessly blends online checkout (Stripe Checkout/Payment Intents), in-person POS (Stripe Terminal), subscriptions (Stripe Billing), and localized alternative payment methods (APMs).

  ### Identification of Gaps
  1. **Omnichannel Payment Synchronization:** Gap in syncing online deposits with in-person final payments.
  2. **Multi-Tenant Payment Routing:** Ensuring secure, isolated routing of funds to the correct tenant (business owner) without cross-contamination.
  3. **AI Agent Integration for Finance:** Missing an intelligent layer where the "Finance & Payments" agent can autonomously handle disputes, refunds, and dynamically adjust pricing based on market data.

  ## Design Doc

  ### Architecture Overview
  This architecture introduces a unified Payment Gateway Service (PGS) that bridges OHC’s multi-tenant backend with external providers (primarily Stripe, with extensibility for others).

  #### Diagram: Payment Gateway Integration (Mermaid.js)
  ```mermaid
  graph TD
      A[Client UI - Mobile/Web] -->|API Request| B[API Gateway]
      B --> C[Payment Gateway Service]
      C -->|gRPC/REST| D[Stripe API]
      D -->|Webhook| E[Webhook Handler]
      E --> F[AI Finance Agent]
      E --> G[PostgreSQL - Ledger]
      C --> H[Redis - Idempotency Cache]
      F -->|Action| C
  ```

  ### UI & UX (Mobile-First)
  - **375px Flow:**
    1. User (customer) accesses the checkout page.
    2. Large, native-style buttons for Apple Pay/Google Pay.
    3. Seamless transition to native keyboard for numeric inputs.
    4. Glassmorphism success screen upon payment completion.

  ### Multi-Tenant Data Model
  - **Table `payments`:**
    - `tenant_id` (UUID, mandatory for RLS)
    - `payment_id` (UUID, Primary Key)
    - `amount` (Decimal)
    - `currency` (String)
    - `status` (Enum)
    - `provider_ref` (String, e.g., Stripe PaymentIntent ID)
    - `metadata` (JSONB)

  ### Security & Zero Trust
  - All external API keys managed securely (e.g., HashiCorp Vault or AWS Secrets Manager).
  - Strict Row-Level Security (RLS) on the `payments` table to enforce tenant isolation.
  - Webhook signature verification mandatory for all incoming provider events.

  ## Implementation Prompt
  **Prompt for Implementer Agent:**
  "Implement a unified Payment Gateway Service in Go that integrates Stripe Payment Intents and Webhook handling. The service must ensure strict multi-tenant isolation using the `tenant_id` column and Row-Level Security (RLS). Ensure high-fidelity metrics are emitted via OpenTelemetry. The UI implementation must prioritize a mobile-first checkout flow with native payment integrations (Apple/Google Pay) and adhere to the OHC Premium Token design system. Ensure comprehensive test coverage (unit and E2E) simulating an end-to-end checkout flow for a non-technical persona."

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
