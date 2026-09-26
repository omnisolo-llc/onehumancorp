issue_title: '🔍 Scout: Tool Integration Research - Google Workspace'
issue_description: |
  **Title**: 🔍 Scout: Tool Integration Research - Google Workspace

  **Problem Statement**: The current OHC codebase lists Google Workspace integration in some places, but `catalog.rs` and `registry.rs` lack a dedicated Google Workspace provider setup (though Google Calendar exists). We need to scout how to fully integrate Google Workspace (e.g. Gmail, Drive, Docs) to connect an imported business.

  **Research Report**:
  - The business owner needs to connect their Google Workspace to OHC for managing emails (Gmail), documents (Google Docs/Drive), and directory services.
  - The integration would allow OHC agents to draft/read emails, manage calendar events (already partially covered), and handle file attachments or document generation within Drive.
  - Existing Google Calendar integration uses `https://www.googleapis.com/calendar/v3`. Workspace APIs typically use `https://www.googleapis.com/auth/drive`, `https://mail.google.com/`, etc., or unified endpoints like `https://www.googleapis.com`.
  - Required capabilities: OAuth2 connection for the business owner, read/write access to Gmail for answering inquiries, and Drive access for fulfilling digital deliverables or parsing inbound documents.

  **Design Doc**:
  - **Tool Name**: Google Workspace
  - **Category**: `productivity` / `workspace`
  - **Capabilities**: Email reading/sending, Document reading/writing, File storage.

  **Implementation Prompt**:
  - Add `google_workspace` to `catalog.rs` pointing to the main Google API endpoint.
  - Add `google_workspace` client management to `registry.rs`.
  - Implement basic OAuth flow and client structures in `src/server/integrations/google_workspace`.

  **Superpowers Workflow Provenance**:
  - Loaded skills: `using-superpowers`, `brainstorming`
  - Repository URL: `https://github.com/obra/superpowers.git`
  - Revision hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks performed: Read `SKILL.md` files, verified `catalog.rs` and `registry.rs` contents.
  - Outcomes: Identified missing Google Workspace integration in catalog and registry.

  **Priority**: P2
  **Estimated Scope**: Medium (adding API client, OAuth, and basic endpoints)
issue_priority: P2
issue_category: Integrations
issue_type: Feature
issue_label: agent-report
assignees: []
