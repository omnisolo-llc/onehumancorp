outcome: blocked
issue_title: Native Rust Omnichannel Chat Integration - Core Engine & Widget API
issue_description: |
  The requested feature cannot be safely completed because the required database schema (e.g. `inboxes`, `contacts` tables) and migration framework context (SQLx vs SeaORM) are missing from the issue description and codebase. Adding these entities to the database layer `omnichannel_repo.rs` is impossible without corresponding table creations. Also, the frontend requirements for the Web Widget are undefined.
