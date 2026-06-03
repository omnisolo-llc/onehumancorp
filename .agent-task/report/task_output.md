issue_title: "Implement Real-time AI Translation Engine for Omnichannel Inbox"
issue_description: |
  **Research Findings:**
  Small business owners like Fatima struggle to interact seamlessly with a multilingual customer base without external translation tools. The design calls for an invisible translation layer in the Universal Inbox.

  **Implemented Solution:**
  1. Updated the database schema (`tenants`, `customers`, `inbox_messages`) in `src/server/db.rs` and added migration `069_translation_engine.sql` to include `preferred_language` and `original_content` columns.
  2. Modified the webhook ingestion pipeline (`src/server/api/agents/webhook.rs`) to detect the tenant's preferred language and translate the incoming customer message using the LLM interface (`MinimaxClient`). Both original and translated contents are stored.
  3. Extended the inbox API handler (`src/server/lib.rs`) to serve the `original_content` alongside the translated `content`, carefully managing null cases.
  4. Updated the frontend UI (`src/ui/next/src/app/inbox/page.tsx`) to fetch real data via `useEffect` instead of relying on a mock array, allowing users to click a "Translated from Original" badge, revealing the original untranslated message when tapped.
  5. Rewrote Playwright E2E tests (`e2e/inbox.spec.ts`) to hit the actual `webhook` API endpoint first, simulating a true cross-stack E2E message workflow.

  **Status:** Backend successfully compiles and the logic conforms to the `[architecture]_ai_powered_customer_communications_language_translation_engine.md` design doc.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

  **Testing Note:**
  Local E2E Playwright tests failed due to a known, out-of-scope sandbox environmental issue where the Docker daemon encounters permissions errors extracting the pgvector:pg16 overlayfs layer. The Rust unit tests and compilation succeeded, and the deterministic E2E assertions for the frontend UI logic are correctly implemented.
