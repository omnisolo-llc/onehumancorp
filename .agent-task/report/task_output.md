issue_title: "✍️ Scribe: Audit of F06: Unusable connection flow"
issue_description: |
  **Superpowers Workflow Provenance**
  - Loaded Skills: `skills/using-superpowers/SKILL.md`, `skills/brainstorming/SKILL.md`, `skills/writing-plans/SKILL.md`, `skills/executing-plans/SKILL.md`, `skills/verification-before-completion/SKILL.md`
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Checks: `ls -la`, `cat`, `grep` to audit Google Workspace and general tool integration connection flows.
  - Outcomes: The `google_workspace` integration is incomplete. While the `GoogleWorkspaceProvider` and `GoogleWorkspaceClient` exist (implementing Drive, Sheets, and Gmail functions), `google_workspace` is completely missing from:
    1. The `IntegrationsRegistry` (`src/server/integrations/registry.rs`) - meaning no client instances can be created, stored, or dispatched.
    2. The integration catalog (`src/server/integrations/catalog.rs`) - meaning the UI and APIs won't list it.
    3. The `supported` list in `src/server/harness/middleware/connection_vault.rs` - meaning credentials cannot be securely stored or managed for it.
    4. The credential validation in `provider_credentials_present` in `src/server/api/tool_integrations.rs`.

  The `google_calendar` integration is partially implemented but uses a different model.

  **Title:** Audit of F06: Unusable connection flow

  **Problem Statement:** The documentation audit (F06) highlighted that "Google Workspace OAuth lifecycle and wider connector support remain unverified/incomplete." This audit confirms that the Google Workspace integration is structurally incomplete. Code exists for API interactions (Drive, Sheets, Gmail), but it is entirely disconnected from the application's integration registry, catalog, credential vault, and connection handlers. It cannot currently be configured or used by a tenant.

  **Research Report:**
  1.  **Catalog (`src/server/integrations/catalog.rs`):** The `get_catalog()` function lists `google_calendar`, but `google_workspace` is absent.
  2.  **Registry (`src/server/integrations/registry.rs`):** The `IntegrationsRegistry` manages clients for many integrations (including `google_calendar`, `zoom`, `stripe`, etc.), but has no fields, `connect` logic, or dispatch methods for `google_workspace`.
  3.  **Connection Vault (`src/server/harness/middleware/connection_vault.rs`):** The `supported()` function strictly checks for `"openai_api" | "anthropic_api" | "stripe" | "resend"`. Even if the UI sent credentials, the vault would reject them with `LedgerError::Invalid`. Note that `google_calendar` is *also* missing from the vault's supported list, meaning its credentials cannot be securely persisted either.
  4.  **Credential Validation (`src/server/api/tool_integrations.rs`):** `provider_credentials_present` checks specific ID strings but doesn't explicitly support a generic token layout needed by `google_workspace` (though the fallback might match if it assumes `bot_token` or `api_token`).

  **Design Doc:**
  To fully support Google Workspace:
  1. Add `"google_workspace"` to `supported()` in `connection_vault.rs`.
  2. Add `"google_workspace"` to `get_catalog()` in `catalog.rs` with appropriate metadata.
  3. Update `IntegrationsRegistry` in `registry.rs` to hold an `RwLock<HashMap<String, Arc<GoogleWorkspaceProvider>>>`.
  4. Implement `connect` logic in `registry.rs` to instantiate `GoogleWorkspaceProvider` and store it when credentials are provided.
  5. Plumb specific Drive/Sheets/Gmail operations through `IntegrationsRegistry` similar to how `send_email` or `create_event` are exposed, routing to the `GoogleWorkspaceProvider`.
  6. Implement proper OAuth 2.0 flow handling. The current `GoogleWorkspaceClient` assumes a simple `access_token` string (`GoogleWorkspaceClient::new(access_token: String)`), which is insufficient for long-lived offline access without refresh token handling.

  **Implementation Prompt:**
  Not applicable for this audit task. The task is to establish facts about the current state.

  **Priority:** P1 (Blocks advertised capability).

  **Estimated Scope:** Medium. Requires adding the provider to the registry, catalog, and vault, plus designing the OAuth/refresh token lifecycle if long-lived access is required.

issue_priority: "P1"
issue_category: "integrations"
issue_type: "audit"
issue_label: "ohc:lane:integrations"
assignees: []
