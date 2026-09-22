issue_title: "🔍 Scout: Tool Integration Research - Google Workspace Webhooks"
issue_description: |
  # Superpowers workflow compliance

  Skills loaded:
  - using-superpowers (revision: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20` from upstream repository)
  - brainstorming (revision: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20` from upstream repository)

  # Mission Queue Protocol Report

  ## Title
  Research and Architect Google Workspace Integrations (OAuth, Email, Calendar, and Webhooks)

  ## Problem Statement
  Many non-technical operators (like solo web/design/marketing services) rely heavily on Google Workspace for managing their business interactions. Existing implementations in OmniSolo do not provide robust, secure, and authenticated integrations. A complete connection lifecycle is missing, and the product needs a secure method to interact with Gmail, Google Calendar, and other Google APIs, handling OAuth correctly and safely (especially with webhook push notifications). We need an evidence-backed strategy for integrating Google Workspace capabilities.

  ## Research Report
  - **Ecosystem:** Google Workspace is ubiquitous for small businesses. Enabling OmniSolo to manage emails (Gmail) and appointments (Google Calendar) is a critical capability.
  - **Authentication (OAuth 2.0):** Google provides OAuth 2.0 flows. Crucially, as documented in `RESEARCH.md` and memory constraints, the OAuth state parameter must be signed, one-time verifiable, and include the tenant ID, expiry, and a random nonce.
  - **Webhooks/Push Notifications:** Google Calendar and Gmail support push notifications (webhooks). However, memory constraints explicitly state: "Do not implement webhook push notifications for external providers (e.g., Google Calendar) unless a full subscription lifecycle abstraction (creation, renewal, verification) exists; prefer polling or sync-token workflows otherwise."
  - **Data Privacy & Scope Limits:** We must carefully request only the required scopes (e.g., `https://www.googleapis.com/auth/calendar.readonly`, `https://www.googleapis.com/auth/gmail.readonly`).
  - **Storage:** Use `ConnectionVault` as the credential source of truth for integrations. Store multiple tokens (access, refresh) as a JSON-encoded string within the vault's single secret field. Do not write credentials to raw SQL tables.

  ## Design Doc
  - **Connection Lifecycle:** Implement a unified OAuth flow for Google Workspace via the `/integrations` page and `ProviderConnections.tsx`.
  - **Backend Setup:** A new Axum sub-router in `src/server/lib.rs` for Google integrations, mounted using `.nest()`.
  - **Polling vs. Webhooks:** Given the lack of a full webhook subscription lifecycle abstraction currently, the integration will initially use **polling** or **sync-token workflows** (e.g., using Google Calendar's `syncToken`) to fetch updates.
  - **Security:** Strict OAuth state validation (signed, expiring, nonce). Storage via `ConnectionVault`.

  ## Implementation Prompt
  Implement the Google Workspace integration for OmniSolo.
  1. Add a Google Workspace connection option in the frontend canonical `/integrations` page (`ProviderConnections.tsx`).
  2. Implement the backend OAuth 2.0 callback flow. Ensure the `state` parameter is signed, verifiable, includes the tenant ID, expiry, and a nonce, and that these are validated before exchanging the code.
  3. Store the retrieved access and refresh tokens securely in the `ConnectionVault` as a JSON string. Do not store them in standard SQL columns.
  4. Implement a polling-based synchronization mechanism (using `syncToken` for Calendar or `historyId` for Gmail) to fetch updates. Do not implement push webhooks.
  5. Ensure new Axum routers are nested correctly in `src/server/lib.rs`.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
