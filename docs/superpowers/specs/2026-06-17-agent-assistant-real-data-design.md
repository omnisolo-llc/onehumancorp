# Agent Assistant Real Data Design

## Goal

Agent Assistant must not show demo, seeded, or fabricated data in task results or feature tabs. Skills, memory, connectors, task results, and related enable/disable actions must reflect real backend/database state. If the backend or database cannot provide the data, the UI must show an honest empty or error state.

## Current Problem

The Next Assistant API currently mixes real backend proxying with in-memory fallback data from `src/ui/next/src/app/api/assistant/store.ts`. When backend calls fail, routes such as tasks, workspaces, skills, memory, connectors, and other tabs can return seeded records. Some task creation paths also synthesize assistant messages and artifacts. This makes UI tabs look implemented even when there is no real database-backed state.

## Scope

This first pass covers:

- Assistant tasks and result tabs: remove seeded fallback task/result data and stop fabricating artifacts or assistant replies.
- Memory: read and mutate memory records through backend/database-backed routes.
- Skills: read installed/discovered skill records from backend/database-backed routes and persist enable/disable/install state.
- Connectors: read connector records from backend/database-backed routes and persist connect/disconnect state.
- Tests: prove demo fallback is gone and that unavailable backend/database state is surfaced honestly.

Out of scope for this pass:

- Building full external OAuth flows for every connector.
- Implementing every possible skill runtime behavior.
- Redesigning the Assistant UI layout.
- Migrating non-Assistant product pages that also contain demo data.

## Architecture

The Rust backend is the source of truth for Assistant state. Next.js Assistant API routes become thin proxy/adaptation layers:

- `GET /api/assistant/tasks` proxies backend tasks and hydrates messages, artifacts, and file changes from backend task subresources.
- `POST /api/assistant/tasks` creates a backend task only. It does not create fake assistant replies or generated artifacts.
- `GET/PATCH /api/assistant/memory` proxies real memory records and mutations.
- `GET/PATCH /api/assistant/skills` proxies real skill records and persisted status changes.
- `GET/PATCH /api/assistant/connectors` proxies real connector records and persisted status changes.

If a backend request fails, the Next route returns a non-2xx error with a clear message. It does not call seeded local store functions as a fallback.

## Database Model

Use existing Assistant tables where already present:

- `assistant_workspaces`
- `assistant_tasks`
- `assistant_messages`
- `assistant_artifacts`
- `assistant_file_changes`

Add or use backend tables for feature state:

- `assistant_memory_records`: user-visible memory content and metadata.
- `assistant_skills`: skill id/name/source/category/status/version and timestamps.
- `assistant_connectors`: connector id/name/kind/status/config metadata and timestamps.

The database stores product state, not demo catalog entries. Runtime discovery can populate or sync records, but the UI reads from database-backed APIs.

## UI Behavior

The Assistant UI keeps the current section navigation and resource rendering pattern, but changes its data contract:

- Empty database result: show "No records" or the existing empty state.
- Backend unavailable: show an error state.
- Mutations: update UI only from the backend response after persistence succeeds.
- Result tabs: render files/artifacts/changes/previews only from persisted task subresources.

## Error Handling

Backend failures should be visible and actionable:

- Next routes return `502` for backend reachability or upstream failures.
- Validation failures return `400`.
- Missing records return `404`.
- UI sections display the returned error text and keep prior data only if it was loaded from a successful real response.

No route should silently return demo data to make a failed feature appear healthy.

## Testing

Add or update tests to cover:

- Task list GET fails honestly when backend is unavailable and does not return seeded tasks.
- Task creation does not create synthetic assistant messages or artifacts.
- Memory GET/PATCH uses backend/database responses.
- Skills GET/PATCH persists and reflects real status changes.
- Connectors GET/PATCH persists and reflects real status changes.
- Assistant page renders empty/error states without demo records.

## Acceptance Criteria

- Searching the Assistant API code finds no seeded fallback path for covered tabs.
- Skills, memory, and connector enable/disable actions are real persisted mutations.
- The four result tabs only display records returned by backend task subresources.
- Demo labels such as seeded connector names or fake generated artifacts do not appear unless they exist in the database.
- Tests fail if a covered route falls back to in-memory demo data.
