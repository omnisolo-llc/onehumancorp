issue_title: "🔎 Scout: Tool Integration Research - Google Workspace vs Stripe (Integration Gap Analysis)"
issue_description: |
  **Overview**
  This report investigates the current integration boundaries and charging mechanisms for compute/AI API usage (Managed API vs Customer API Key / BYOK vs Provider-Native Client), as well as evaluating the current integration state for Stripe and Google Workspace to prove a reusable operating loop.

  **AI Usage & Charging Modes**
  - **Managed API**: OHC acts as a reseller. Current findings (F03, F05 in remediation ledger) indicate that while basic budgeting exists, a true invoice-grade meter is blocked. F05 remains blocked due to missing durable idempotent usage, payer/auth attribution, and reconciliation.
  - **Customer API Key / BYOK**: Direct payment to AI provider by the customer. OHC would only charge a hosting fee. While technically feasible, the current implementation lacks strict boundaries and attribution separating BYOK from OHC-managed keys. F13 notes "API key, consumer plan and native-client subscription are distinct" as currently blocked.
  - **Provider-Native Client**: Requires specific entitlements and generally prohibits session-token relay or proxying. Existing adapters are not compliant with terms for end-user subscription brokering.

  **Integrations**
  - **Stripe**: Existing integrations exist primarily as mock contracts and webhook stubs (e.g., `src/server/integrations/stripe`, `closer_agent_cuj.mock-contract.ts`). F08 (fabricating checkout URLs) is marked closed (indicating real provider sessions are attempted), but the full E2E flow is blocked by the lack of an invoice-grade meter (F05).
  - **Google Workspace**: Some clients exist, but the "connected reporting" capability is unresolved. Further proof is needed to show that connected reporting reduces effort beyond native tools.

  **Conclusion / No-Work Finding**
  This is a research report confirming no immediate code changes are needed. The requested research is complete. The gaps regarding F05, F11, F12, and F13 are documented as blocked in the remediation ledger. No generic features will be built.
