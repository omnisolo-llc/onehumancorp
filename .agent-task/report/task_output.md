issue_title: "🔍 Scout: Tool Integration Research - Google Workspace"
issue_description: |
  # Tool Integration Research: Google Workspace

  ## Title
  Integrate Google Workspace (Gmail, Drive, Sheets, Meet, Calendar) for Core Operations

  ## Problem Statement
  Digital service providers (like web designers, tutors, and marketers) run their businesses on Google Workspace. They spend hours manually copying data between their email, documents, spreadsheets, and calendar. An owner needs their AI team to read emails, save files, update tracking sheets, and schedule video meetings without leaving the OmniSolo platform or logging into multiple tabs.

  ## Research Report
  - **Tool Evaluated:** Google Workspace (Gmail, Google Drive, Google Sheets, Google Meet, Google Calendar).
  - **Value Proposition:** Connects OmniSolo directly to the daily operational tools of small businesses. Allows the AI team to orchestrate entire workflows (e.g., read an inquiry in Gmail, generate a proposal in Drive, log the prospect in Sheets, and schedule a Meet in Calendar).
  - **Capabilities:**
    - **Gmail:** Send emails, list unread messages, fetch message content.
    - **Drive:** List files, read file content, create files.
    - **Sheets:** Read ranges, write ranges, create spreadsheets.
    - **Calendar / Meet:** (Already partially in `google_calendar` but part of the broader Workspace).
  - **Limitations & Considerations:** Requires strict OAuth scope management (e.g., `https://www.googleapis.com/auth/gmail.send`). Handling token refresh and secure credential storage is critical. OmniSolo must respect the principle of least privilege, allowing the user to select which Google services to connect.
  - **Pricing/Viability:** Free tier APIs are generous and sufficient for SMB operators using their existing Google Workspace or personal accounts.

  ## Design Doc
  - **Trigger:** Connects during the onboarding flow or from the Settings -> Integrations dashboard.
  - **Action:**
    - User completes an OAuth 2.0 flow to authorize OmniSolo.
    - The `GoogleWorkspaceProvider` is instantiated and registered in the `IntegrationsRegistry` (`src/server/integrations/registry.rs`), just like `google_calendar`.
    - The `catalog.rs` is updated to expose `google_workspace` to the OmniSolo ecosystem.
  - **User Experience:** The owner sees a unified "Connect Google Workspace" button. Once connected, they can instruct their AI team to "Check my email for new leads" or "Save this invoice to Drive."
  - **Architecture:** The client and provider implementation already exist in `src/server/integrations/google_workspace`. The task is to expose it via the `IntegrationsRegistry` and `catalog.rs` so the AI orchestrator can utilize it.

  ## Implementation Prompt
  1. Add `google_workspace` to `src/server/integrations/catalog.rs` so it appears in the integration discovery list.
  2. Instantiate and register the `GoogleWorkspaceProvider` inside `src/server/integrations/registry.rs` (similar to how `google_calendar` and `google_analytics` are handled).
  3. Ensure the provider accepts the OAuth access token and provides access to its existing Drive, Sheets, and Gmail methods.
  4. Write unit/integration tests confirming the registry can retrieve the `GoogleWorkspaceProvider` and its metadata.

  ## Priority
  P1

  ## Estimated Scope
  Small

  ---
  ## Superpowers Workflow Provenance
  - **Skill Loaded:** `using-superpowers`, `brainstorming`
  - **Revision:** `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - **Checks Performed:** Explored `RESEARCH.md`, `business_capability_and_usage_economics_audit.md`, `catalog.rs`, and the `google_workspace` crate. Verified that `GoogleWorkspaceClient` and `GoogleWorkspaceProvider` are implemented but missing from the central `registry.rs` and `catalog.rs`.
  - **Outcome:** Prepared a comprehensive research report and implementation prompt for exposing the existing Google Workspace integration to the OmniSolo agent matrix.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
