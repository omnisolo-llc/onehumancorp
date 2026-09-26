issue_title: 'F13: API key, consumer plan and native-client subscription are distinct'
issue_description: |
  **Title:** F13: API key, consumer plan and native-client subscription are distinct

  **Problem Statement:**
  The current code does not verify native-client subscription support, lacks provider-permitted native-client subscription hosting, and does not differentiate between different subscription tiers and API keys correctly. This could lead to a violation of provider terms of service, pooling of subscriptions, and unauthorized token relay.

  **Research Report:**
  1.  **OpenAI:** Distinguishes Business from separately billed API usage. Codex authentication supports ChatGPT subscription or API-key login; recommends API keys for programmatic CLI work; and documents Enterprise access tokens for trusted private Codex automation, not general API calls.
  2.  **Anthropic:** Permits hosting the **unmodified** Claude Code binary under stated conditions, including users authenticating and paying directly. Prohibits a third-party Claude.ai login/token relay and end-user usage resale/intermediation.
  3.  **Google:** Distinguishes eligible Google AI/Code Assist login allowances from API-key and Vertex billing. A generic Workspace subscription is not automatically a Gemini API entitlement.

  *Superpowers Provenance:*
  - Read `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md` (no API changes necessary for no-work/blocked implementation job)
  - Loaded `superpowers:brainstorming` from revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d.

  **Design Doc:**
  *   **Blocked Prerequisites:** Provider-permitted native-client subscription hosting is a separate integration/terms/quotas decision and is not generally implemented. Clarification and business agreements with providers (OpenAI, Anthropic, Google) regarding native-client subscriptions are required before proceeding with code changes.
  *   **Mermaid.js architecture diagram:**
  ```mermaid
  graph TD;
      Client-->API_Proxy;
      API_Proxy-->Provider_Auth;
      Provider_Auth-->|Blocked: Missing Agreement|Native_Client_Sub;
  ```
  *   **UI Wireframes:** N/A
  *   **Mobile UX flow:** N/A
  *   **AI agent integration points:** N/A

  **Implementation Prompt:**
  Once business agreements and provider permissions are secured, implement distinct authentication paths for API keys and native-client subscriptions, ensuring strict adherence to provider terms (no session-token relay, no pooling).

  **Priority:** High (Compliance/Legal risk)
  **Estimated Scope:** Blocked
issue_priority: 'P0'
issue_category: 'security'
issue_type: 'feature'
issue_label: 'agent-report'
assignees: []
