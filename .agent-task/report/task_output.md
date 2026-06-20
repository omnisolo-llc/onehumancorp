issue_title: "[Research] Deprecate UI Mock Data in favor of the Real Data Contract"
issue_description: |
  # Deprecate UI Mock Data in favor of the Real Data Contract

  ## Problem Statement
  The Next Assistant API currently mixes real backend proxying with in-memory fallback data from `src/ui/next/src/app/api/assistant/store.ts`. When backend calls fail, routes such as tasks, workspaces, skills, memory, connectors, and other tabs return seeded records. Some task creation paths also synthesize assistant messages and artifacts. This makes UI tabs look implemented even when there is no real database-backed state. We need to enforce a real data contract, meaning the UI should show real data from the backend or an honest empty/error state.

  The goal is to update the Next.js Assistant API route handlers so that they act as thin proxies and do not rely on local JSON mock stores. All API calls must flow to the backend (`/api/assistant/...`), and if the backend does not implement the capability, an honest 501/502 state is returned.

  ## Scope
  - Tasks and Result Tabs: Remove seeded fallback task/result data.
  - Connectors, Approvals, Skills, Memory, Automation, and all other routes under `src/ui/next/src/app/api/assistant/`.
  - Next API routes referencing `store.ts` should be updated to make real `fetch()` calls to the backend, returning appropriate JSON or error messages.
  - Remove `src/ui/next/src/app/api/assistant/store.ts`.

  ## Proposed Architecture & Fix
  The Next.js Assistant API routes become thin proxy layers exactly like `api/assistant/memory/route.ts`:
  - They should read `BACKEND_URL` and `x-tenant-id`.
  - They should forward the request to the corresponding backend route.
  - They should return the proxy response or an appropriate error response.

  ## Implementation Prompt
  **User Persona**: All personas. As a business owner using OHC, I should only see real actions that I have taken or real data from my account. I should not see fake test records in my task queue or memory.
  **CUJ**: When interacting with the Assistant features (e.g. creating a task or changing a setting), the change must reflect actual persistence in the backend.
  **Acceptance Criteria**:
  - All routes in `src/ui/next/src/app/api/assistant/` are updated to fetch from `BACKEND_URL`.
  - `src/ui/next/src/app/api/assistant/store.ts` is deleted.
  - No fallback/seeded data is shown in UI.
  - The E2E tests enforcing real data contracts pass successfully.

  ## Priority
  `P0`

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
