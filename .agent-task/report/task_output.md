outcome: blocked
issue_title: "Native Rust Omnichannel Messaging System Replication"
issue_description: |
  Superpowers skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md) and brainstorming skill loaded.

  The native omnichannel chat feature (webhook endpoint `omnichannel_webhook`, db models like `Conversation`, `Message`, etc) already exists and is fully implemented in Rust in the `server_omnisolo` code, thus satisfying the requirements of the task. There is no work to do to implement this feature as it's already implemented.

  However, the E2E verification depends on Next builds which fail due to missing system dependencies (`libglib2.0-dev`) and NPM peer dependency conflicts, making full E2E testing blocked and therefore impossible to safely verify changes without resolving those environment issues first.
