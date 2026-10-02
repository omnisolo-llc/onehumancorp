outcome: no_work
issue_title: "Native Rust Omnichannel Chat System - Core Architecture"
issue_description: |
  According to RESEARCH.md, the core strategic directive is to integrate existing tools instead of rebuilding them. Requests to build native replacements for external services (such as Chatwoot) without explicit authorization, evidence, and an expansion gate must be rejected as no-work findings. The earlier Chatwoot integration codebase remains in its superseded status.

  The environment encountered build failures when running validation commands (`make test` and `make lint`) because `next` was not found during the frontend build step (`npm run build:web`), resolving it by installing `next@14` locally might work but modifying lockfiles and dependency trees just for generating a report-only PR is not necessary. We are submitting this blocked report due to the strict explicit authorization rule against rebuilding external tools natively.
