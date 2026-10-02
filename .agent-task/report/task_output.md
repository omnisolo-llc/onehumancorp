outcome: no_work
issue_title: Implement Native Rust Omnichannel Inbox to Replace Chatwoot Dependency
issue_description: |
  We confirmed that there are no active Chatwoot references inside the repository's configuration.
  A thorough search across the workspace (excluding `docs/` and `*.md` files) via `rg -i chatwoot --glob '!docs/' --glob '!*.md'` resulted in zero matches.

  Furthermore, `chatwoot` was already removed previously as recorded in:
  - `docs/superpowers/plans/2026-07-13-chatwoot-removal.md`

  The native omnichannel inbox feature replacing Chatwoot is already implemented across numerous components including `src/server/services/inbox/test_inbox.rs`, `src/server/api/inbox_api_test.rs`, and various `src/server/db/migrations/146_omni_inbox_rls.sql`.

  Therefore, the request to 'Implement Native Rust Omnichannel Inbox to Replace Chatwoot Dependency' has already been satisfied by prior work, and no further code changes are required.
issue_priority: P0
issue_category: integrations
issue_type: feature
issue_label: [ohc:lane:backend]
assignees: []
