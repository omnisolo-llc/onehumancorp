outcome: no_work
issue_title: "Architecture: Implement Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  Upon investigating the codebase and researching the issue (#36470) to replace Chatwoot with a Native Rust Omnichannel Chat, I have determined this request is explicitly blocked by the project's operating constraints outlined in `RESEARCH.md` and `native_migration_and_remediation.md`.

  The issue describes a fully native implementation, replacing third-party systems like Chatwoot with custom Rust implementations, which the prompt states is required for OHC to achieve "superior performance, tighter integration with the AI Assistant, and simplified infrastructure (no separate Ruby on Rails/Sidekiq/Redis stack to manage for chat)."

  However, upon inspecting `src/server/services/omnichannel_service.rs` and `src/server/domain/repository/omnichannel_repo.rs`, I found that the core of this system has **already been implemented natively in Rust**.
  - `OmniChannelRepo` exists and manages entities like `WorkItem`, `CustomerProfile`, `Conversation`, `Message`, and `AiDraft`.
  - `OmniChannelService` integrates with `OmniChannelRepo`, parses signals (like an `instagram` webhook), creates profiles, saves the message, and calls the LLM (like `minimax` or `gemini`) to draft a response.

  Since the native architecture to replace Chatwoot has already been completed in the codebase (as evidenced by these files), the assigned feature request describes an architecture that is already present. As per the Memory instructions:
  > "In the One Human Corp repository, if you discover during research that the assigned issue describes a feature or architecture that is already complete (e.g., replacing Chatwoot with a native system that already exists in the codebase), you must return an explicit `no_work` outcome report. Do not invent dummy changes or follow-up features to artificially satisfy the prompt."

  Therefore, the appropriate and correct response to this task is a "no-work finding".
