issue_title: "Research tool integration: Trello integration missing from catalog and registry"
issue_description: |
  # Trello Integration Gap Research

  ## Title
  Trello integration missing from catalog and registry

  ## Problem Statement
  The Trello integration is partially implemented in `src/server/integrations/trello/` but is completely missing from the system's active catalog (`src/server/integrations/catalog.rs`) and registry (`src/server/integrations/registry.rs`). This means owners cannot actually connect their Trello accounts or use the integration, despite the code existing. Non-technical owners rely on visual boards like Trello for project management, lead tracking, and content pipelines. This gap prevents them from bringing their existing workflow into the OHC platform.

  ## Research Report
  - **Tool Evaluated**: Trello (by Atlassian)
  - **Relevance**: Extremely popular Kanban-style visual project management tool used by freelancers, agencies, and small service businesses (like Nora, our target persona for digital services).
  - **Current Code State**:
    - Exists: `src/server/integrations/trello/client.rs` (implements API client for boards, lists, cards, labels)
    - Exists: `src/server/integrations/trello/provider.rs` (implements `IntegrationProvider`)
    - Missing: Registration in `src/server/integrations/catalog.rs`
    - Missing: Initialization in `src/server/integrations/registry.rs`
  - **Ease of Use**: Trello is the gold standard for non-technical visual project management. Integrating it allows users to see OHC actions (like "New Lead" or "Project Approved") directly in a format they understand.
  - **Integration Pattern**: REST API using API Key and Token authentication.

  ## Design Doc
  - **Trigger/Action**: When OHC identifies a new lead, a project milestone, or a task, it can automatically create or move cards in the user's Trello board.
  - **User Experience**: The user connects Trello by providing their API Key and Token. OHC can then manage cards on their behalf, mapping stages (e.g., "Lead", "Proposal", "Active", "Done") to Trello lists.
  - **Integration Points**:
    - `src/server/integrations/catalog.rs` needs a metadata entry for Trello.
    - `src/server/integrations/registry.rs` needs to instantiate `TrelloProvider` and add it to the registry.

  ## Implementation Prompt
  - Ensure the Trello integration is discoverable by adding it to `src/server/integrations/catalog.rs`.
  - Ensure the Trello integration is usable by adding it to `src/server/integrations/registry.rs`.
  - The integration must be successfully registered alongside other integrations like Asana or GitHub.

  ## Priority
  P2

  ## Estimated Scope
  Small
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
