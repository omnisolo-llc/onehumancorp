issue_title: "Architectural Resolution for F13: BYOK and Native-Client Subscriptions"
issue_description: |
  # Title
  Architectural Resolution for F13: BYOK and Native-Client Subscriptions

  # Problem Statement
  Currently, API key usage, consumer plans, and provider-permitted native-client subscriptions are conflated as per F13 in the remediation ledger. From the perspective of a small business owner, distinguishing between Bring Your Own Key (BYOK) for general API use versus linking a permitted consumer subscription (where permitted) is critical. We must ensure our application strictly separates these modes, avoiding token pooling, session-token relay, or silently billing our own account for customer-directed inference, while preserving strict tenant isolation and honoring provider terms of service.

  # Research Report
  The `docs/research/business_capability_and_usage_economics_audit.md` correctly notes that Anthropic and Google offer specific guidelines around application hosting and subscription modes. Anthropic permits hosting unmodified binaries where users authenticate directly without intermediary relay. Google differentiates API-key usage (via Google Cloud) from Gemini AI access. Our solution must strictly delineate:
  1. OHC-funded inference (managed API) - where we charge the customer based on tracked compute/resources.
  2. Customer-funded API key - BYOK mode where the customer provides their own provider API keys and we do not double-bill for inference.
  3. Provider-native client - where the customer authenticates directly with the provider using supported SDKs.

  These modes must be distinct and fail-closed if unsupported combinations are attempted.

  Source dates: 2026-09-18
  Uncertainties: Exact willingness to pay, usage tolerance, privacy preference and desired autonomy remain unknown.

  # Design Doc

  ## Architecture
  ```mermaid
  graph TD
      A[Small Business Owner] -->|Selects Billing Mode| B(Configuration UI)
      B --> C{Billing Strategy Selector}
      C -->|Mode: OHC Managed API| D[OHC Core Proxy]
      C -->|Mode: BYOK API Key| E[Customer Key Vault]
      C -->|Mode: Native Client Subscription| F[Native SDK Integrator]

      D --> G[Provider API]
      E --> G
      F --> H[Provider Native Authentication]
  ```

  ## UI Wireframes
  - **Billing Mode Setup:** A setting panel where the owner selects "Use OHC Managed Service", "Provide My Own API Key", or "Link My Existing AI Subscription".
  - **BYOK Configuration:** A secure form to input API keys.
  - **Native Client Setup:** A direct authentication flow (OAuth or SDK login) specifically designated for the supported native client integrations, clearly displaying "You are using your own subscription."

  ## Mobile UX Flow
  1. Owner opens Settings -> AI Providers.
  2. Owner selects a provider (e.g., Anthropic, Google).
  3. App prompts for the connection method (Managed, BYOK, Subscription).
  4. App directs to secure key entry or native OAuth login.
  5. App returns to Settings with connection status verified.

  ## AI Agent Integration Points
  - The AI Agent runner will inspect the configured Billing Strategy prior to dispatching inference requests.
  - Cost tracking agents will disregard token counts for BYOK or Native-Client modes when calculating OHC revenue/invoices, logging them only for owner visibility.
  - Security agents will enforce that BYOK keys are bound to the tenant and never pooled.

  # Implementation Prompt
  Implement the tri-modal provider connection strategy in the settings configuration. Ensure that the inference dispatch layer reads the selected mode, routes requests appropriately without mixing OHC keys with customer keys, and completely isolates token cost accounting to prevent double charging. Create the UI for owners to clearly select and manage these three distinct modes.

  # Priority
  High

  # Estimated Scope
  Medium

  # Final Evidence and Skill Provenance
  Loaded Superpowers revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)

  Executed test commands:
  - `cargo check --locked --workspace --exclude app --all-targets` (Timed out)
  - `make test-backend` (Timed out)
  - `make lint` (Failed)
issue_priority: "High"
issue_category: "Architecture"
issue_type: "Epic"
issue_label: "F13"
assignees: []
