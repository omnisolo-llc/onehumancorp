# Agent Assistant Real Data Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace Agent Assistant demo-backed memory, skills, connectors, and task result behavior with real backend/database-backed reads and mutations.

**Architecture:** The Rust backend owns Assistant persistence and exposes real routes under `/api/assistant`. Next.js Assistant routes become thin proxy/adapters that return backend data or clear upstream errors; they never fall back to `store.ts` seeded data for covered tabs.

**Tech Stack:** Rust, Axum, SQLx, SQLite/Postgres migrations, Next.js app router route handlers, TypeScript, Vitest.

---

## File Structure

- Modify `src/server/db/migrations/028_assistant_workstation.sql`: add Assistant feature-state tables next to the existing Assistant workspace/task tables.
- Modify `src/server/api/assistant.rs`: add serializable feature-state types, backend routes, SQLx CRUD handlers, and SQLite-backed unit tests.
- Modify `src/ui/next/src/app/api/assistant/tasks/route.ts`: remove local task fallback and synthetic task artifacts/messages.
- Modify `src/ui/next/src/app/api/assistant/memory/route.ts`: proxy to backend and remove `store.ts` imports.
- Modify `src/ui/next/src/app/api/assistant/skills/route.ts`: proxy to backend and remove `store.ts` imports.
- Modify `src/ui/next/src/app/api/assistant/connectors/route.ts`: proxy to backend and remove `store.ts` imports.
- Modify `src/ui/next/src/app/api/assistant/route.test.ts`: replace seeded-data assertions with upstream-proxy and fail-closed assertions.
- Modify `src/ui/next/src/app/assistant/page.test.tsx`: make UI tests assert empty/error states instead of seeded labels.

---

### Task 1: Add Real Assistant Feature Tables

**Files:**
- Modify: `src/server/db/migrations/028_assistant_workstation.sql`

- [ ] **Step 1: Add failing migration coverage by inspection**

Run:

```bash
rg -n "assistant_memory_records|assistant_skills|assistant_connectors" src/server/db/migrations/028_assistant_workstation.sql
```

Expected: no matches.

- [ ] **Step 2: Add the feature-state tables**

In `src/server/db/migrations/028_assistant_workstation.sql`, after `assistant_file_changes`, insert:

```sql
CREATE TABLE IF NOT EXISTS assistant_memory_records (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    content TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'global',
    source TEXT,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_skills (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'Custom',
    source TEXT NOT NULL DEFAULT 'database',
    status TEXT NOT NULL,
    version TEXT,
    description TEXT,
    config JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, name)
);

CREATE TABLE IF NOT EXISTS assistant_connectors (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    kind TEXT NOT NULL DEFAULT 'custom',
    status TEXT NOT NULL,
    oauth BOOLEAN DEFAULT FALSE,
    config JSONB,
    last_error TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, name)
);
```

Inside the existing RLS `DO $$` block, add policy sections for the three tables:

```sql
    IF to_regclass('assistant_memory_records') IS NOT NULL THEN
        ALTER TABLE assistant_memory_records ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_memory_records ON assistant_memory_records USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('assistant_skills') IS NOT NULL THEN
        ALTER TABLE assistant_skills ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_skills ON assistant_skills USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('assistant_connectors') IS NOT NULL THEN
        ALTER TABLE assistant_connectors ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_connectors ON assistant_connectors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
```

Inside the `Down` block, add:

```sql
    DROP POLICY IF EXISTS tenant_isolation_assistant_memory_records ON assistant_memory_records;
    DROP POLICY IF EXISTS tenant_isolation_assistant_skills ON assistant_skills;
    DROP POLICY IF EXISTS tenant_isolation_assistant_connectors ON assistant_connectors;
```

Before dropping `assistant_file_changes`, add:

```sql
DROP TABLE IF EXISTS assistant_connectors CASCADE;
DROP TABLE IF EXISTS assistant_skills CASCADE;
DROP TABLE IF EXISTS assistant_memory_records CASCADE;
```

- [ ] **Step 3: Verify migration contains all new tables**

Run:

```bash
rg -n "assistant_memory_records|assistant_skills|assistant_connectors" src/server/db/migrations/028_assistant_workstation.sql
```

Expected: matches for table creation, RLS policy creation, policy drops, and table drops.

- [ ] **Step 4: Commit**

```bash
git add src/server/db/migrations/028_assistant_workstation.sql
git commit -m "feat: add assistant feature state tables"
```

---

### Task 2: Add Backend Feature Route Tests First

**Files:**
- Modify: `src/server/api/assistant.rs`

- [ ] **Step 1: Write failing SQLite-backed tests**

Append this test module to `src/server/api/assistant.rs`:

```rust
#[cfg(test)]
mod real_feature_state_tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::Extension;
    use serde_json::json;
    use std::sync::Arc;
    use tower::ServiceExt;

    fn claims() -> Claims {
        Claims {
            sub: "user-1".to_string(),
            exp: 0,
            iat: 0,
            organization_id: Some("tenant-real".to_string()),
            username: "tester".to_string(),
            email: "tester@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: "jti-1".to_string(),
        }
    }

    async fn test_db() -> Arc<DB> {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        for statement in [
            "CREATE TABLE assistant_workspaces (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, default_work_dir TEXT, default_model TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_tasks (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, workspace_id TEXT NOT NULL, title TEXT NOT NULL, prompt TEXT NOT NULL, status TEXT NOT NULL, mode TEXT, permission_profile TEXT NOT NULL, model_config TEXT, current_step TEXT, archived INTEGER DEFAULT 0, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_messages (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, task_id TEXT NOT NULL, role TEXT NOT NULL, content TEXT NOT NULL, tool_metadata TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_artifacts (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, task_id TEXT NOT NULL, type TEXT NOT NULL, filename TEXT NOT NULL, path TEXT NOT NULL, mime_type TEXT NOT NULL, size INTEGER, preview_ref TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_file_changes (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, task_id TEXT NOT NULL, path TEXT NOT NULL, change_type TEXT NOT NULL, summary TEXT, approval_status TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_memory_records (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, content TEXT NOT NULL, scope TEXT NOT NULL DEFAULT 'global', source TEXT, enabled INTEGER DEFAULT 1, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_skills (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, category TEXT NOT NULL DEFAULT 'Custom', source TEXT NOT NULL DEFAULT 'database', status TEXT NOT NULL, version TEXT, description TEXT, config TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP, UNIQUE (tenant_id, name))",
            "CREATE TABLE assistant_connectors (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, kind TEXT NOT NULL DEFAULT 'custom', status TEXT NOT NULL, oauth INTEGER DEFAULT 0, config TEXT, last_error TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP, UNIQUE (tenant_id, name))",
        ] {
            sqlx::query(statement).execute(&pool).await.unwrap();
        }

        Arc::new(DB {
            pool: crate::db::create_dummy_pg_pool().await,
            store: DbStore::Sqlite(pool),
        })
    }

    async fn request_json(db: Arc<DB>, method: &str, uri: &str, body: serde_json::Value) -> (StatusCode, serde_json::Value) {
        let app = router::<()>(db).layer(Extension(claims()));
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value = serde_json::from_slice(&bytes).unwrap_or_else(|_| json!({}));
        (status, value)
    }

    #[tokio::test]
    async fn memory_import_edit_and_forget_uses_database() {
        let db = test_db().await;

        let (status, value) = request_json(db.clone(), "PATCH", "/memory", json!({
            "action": "import",
            "content": "Real persisted memory",
            "scope": "global"
        })).await;
        assert_eq!(status, StatusCode::OK);
        let memory_id = value["memories"][0]["id"].as_str().unwrap().to_string();

        let (_, listed) = request_json(db.clone(), "GET", "/memory", json!({})).await;
        assert_eq!(listed["memories"][0]["content"], "Real persisted memory");

        let (_, edited) = request_json(db.clone(), "PATCH", "/memory", json!({
            "action": "edit",
            "id": memory_id,
            "content": "Edited real memory"
        })).await;
        assert_eq!(edited["memories"][0]["content"], "Edited real memory");

        let (_, forgotten) = request_json(db, "PATCH", "/memory", json!({
            "action": "forget",
            "id": memory_id
        })).await;
        assert_eq!(forgotten["memories"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn skill_enable_disable_uses_database() {
        let db = test_db().await;
        let (_, installed) = request_json(db.clone(), "PATCH", "/skills", json!({
            "action": "install",
            "name": "Real Skill",
            "category": "Testing"
        })).await;
        assert_eq!(installed["skills"][0]["status"], "installed");

        let (_, disabled) = request_json(db.clone(), "PATCH", "/skills", json!({
            "action": "disable",
            "name": "Real Skill"
        })).await;
        assert_eq!(disabled["skills"][0]["status"], "disabled");

        let (_, listed) = request_json(db, "GET", "/skills", json!({})).await;
        assert_eq!(listed["skills"][0]["name"], "Real Skill");
        assert_eq!(listed["skills"][0]["status"], "disabled");
    }

    #[tokio::test]
    async fn connector_connect_disconnect_uses_database() {
        let db = test_db().await;
        let (_, connected) = request_json(db.clone(), "PATCH", "/connectors", json!({
            "action": "connect",
            "name": "Real Connector",
            "kind": "repository"
        })).await;
        assert_eq!(connected["connectors"][0]["status"], "connected");

        let (_, disconnected) = request_json(db.clone(), "PATCH", "/connectors", json!({
            "action": "disconnect",
            "name": "Real Connector"
        })).await;
        assert_eq!(disconnected["connectors"][0]["status"], "disconnected");

        let (_, listed) = request_json(db, "GET", "/connectors", json!({})).await;
        assert_eq!(listed["connectors"][0]["name"], "Real Connector");
        assert_eq!(listed["connectors"][0]["status"], "disconnected");
    }
}
```

- [ ] **Step 2: Run backend test and verify it fails**

Run:

```bash
bazel test //src/server/api:server_api_unit_test --test_filter=real_feature_state_tests
```

Expected: FAIL because `/memory`, `/skills`, and `/connectors` routes do not exist on the Rust Assistant router.

- [ ] **Step 3: Commit failing tests only**

```bash
git add src/server/api/assistant.rs
git commit -m "test: cover assistant database feature state"
```

---

### Task 3: Implement Backend Memory, Skill, and Connector Routes

**Files:**
- Modify: `src/server/api/assistant.rs`

- [ ] **Step 1: Add routes and payload types**

In `router`, add:

```rust
        .route("/memory", get(list_memory).patch(mutate_memory))
        .route("/skills", get(list_skills).patch(mutate_skill))
        .route("/connectors", get(list_connectors).patch(mutate_connector))
```

Change the routing import to:

```rust
    routing::{get, patch, post},
```

After `FileChange`, add:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantMemoryRecord {
    pub id: String,
    pub content: String,
    pub scope: String,
    pub source: Option<String>,
    pub enabled: bool,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantSkillRecord {
    pub id: String,
    pub name: String,
    pub category: String,
    pub source: String,
    pub status: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub config: Option<serde_json::Value>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantConnectorRecord {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub status: String,
    pub oauth: bool,
    pub config: Option<serde_json::Value>,
    pub last_error: Option<String>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Deserialize)]
struct FeatureMutation {
    action: String,
    id: Option<String>,
    name: Option<String>,
    content: Option<String>,
    scope: Option<String>,
    category: Option<String>,
    kind: Option<String>,
    version: Option<String>,
    description: Option<String>,
    config: Option<serde_json::Value>,
}
```

- [ ] **Step 2: Add helper functions**

Add these helpers before `list_workspaces`:

```rust
fn tenant_id_from(claims: &Claims) -> String {
    claims.organization_id.clone().unwrap_or_else(|| "default".to_string())
}

fn require_text(value: Option<String>, field: &str) -> Result<String, (StatusCode, String)> {
    let trimmed = value.unwrap_or_default().trim().to_string();
    if trimmed.is_empty() {
        Err((StatusCode::BAD_REQUEST, format!("{} is required", field)))
    } else {
        Ok(trimmed)
    }
}
```

- [ ] **Step 3: Implement memory handlers**

Add `list_memory`, `read_memory_records`, and `mutate_memory` handlers. Use SQLite `?` placeholders and Postgres `$1` placeholders. `import` inserts a record, `edit` updates content by `id`, and `forget` deletes by `id`.

The returned shape must be:

```rust
#[derive(Serialize)]
struct MemoryListResponse {
    memories: Vec<AssistantMemoryRecord>,
}
```

All successful mutations return `Json(MemoryListResponse { memories: read_memory_records(...).await? })`.

- [ ] **Step 4: Implement skills handlers**

Add `list_skills`, `read_skill_records`, and `mutate_skill` handlers. `install` upserts by `(tenant_id, name)` with status `installed`; `disable` updates status to `disabled`; `uninstall` deletes by name.

The returned shape must be:

```rust
#[derive(Serialize)]
struct SkillListResponse {
    skills: Vec<AssistantSkillRecord>,
}
```

Unsupported actions return:

```rust
Err((StatusCode::BAD_REQUEST, "unsupported skill action".to_string()))
```

- [ ] **Step 5: Implement connectors handlers**

Add `list_connectors`, `read_connector_records`, and `mutate_connector` handlers. `connect` upserts by `(tenant_id, name)` with status `connected`; `disconnect` updates status to `disconnected`.

The returned shape must be:

```rust
#[derive(Serialize)]
struct ConnectorListResponse {
    connectors: Vec<AssistantConnectorRecord>,
}
```

Unsupported actions return:

```rust
Err((StatusCode::BAD_REQUEST, "unsupported connector action".to_string()))
```

- [ ] **Step 6: Run backend feature tests**

Run:

```bash
bazel test //src/server/api:server_api_unit_test --test_filter=real_feature_state_tests
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/server/api/assistant.rs
git commit -m "feat: persist assistant feature state"
```

---

### Task 4: Make Next Assistant Routes Fail Closed

**Files:**
- Modify: `src/ui/next/src/app/api/assistant/tasks/route.ts`
- Modify: `src/ui/next/src/app/api/assistant/memory/route.ts`
- Modify: `src/ui/next/src/app/api/assistant/skills/route.ts`
- Modify: `src/ui/next/src/app/api/assistant/connectors/route.ts`

- [ ] **Step 1: Write failing route tests for no demo fallback**

In `src/ui/next/src/app/api/assistant/route.test.ts`, replace the seeded task test with:

```ts
test('assistant tasks fail closed when backend is unavailable', async () => {
  const originalFetch = global.fetch;
  global.fetch = vi.fn(async () => {
    throw new Error('backend unavailable');
  }) as any;

  const response = await getTasks();
  const body = await response.json();

  expect(response.status).toBe(502);
  expect(body.error).toContain('Assistant backend unavailable');
  expect(JSON.stringify(body)).not.toContain('Create this week');
  expect(JSON.stringify(body)).not.toContain('weekly-brief.md');

  global.fetch = originalFetch;
});
```

Replace the memory/skills/connectors seeded assertions with tests that stub `global.fetch` and assert proxy behavior:

```ts
test('memory route proxies database-backed backend state', async () => {
  global.fetch = vi.fn(async (url: RequestInfo | URL, init?: RequestInit) => {
    expect(String(url)).toContain('/api/assistant/memory');
    if (init?.method === 'PATCH') {
      return new Response(JSON.stringify({ memories: [{ id: 'mem-real', content: 'Persisted memory', scope: 'global' }] }), { status: 200 });
    }
    return new Response(JSON.stringify({ memories: [] }), { status: 200 });
  }) as any;

  const patchResponse = await patchMemory(patchRequest('http://localhost/api/assistant/memory', {
    action: 'import',
    content: 'Persisted memory',
    scope: 'global',
  }));
  const body = await patchResponse.json();
  expect(body.memories).toEqual([expect.objectContaining({ content: 'Persisted memory' })]);
  expect(JSON.stringify(body)).not.toContain('Prefer concise technical summaries');
});

test('skills route proxies persisted enable and disable state', async () => {
  global.fetch = vi.fn(async (url: RequestInfo | URL, init?: RequestInit) => {
    expect(String(url)).toContain('/api/assistant/skills');
    const payload = init?.body ? JSON.parse(String(init.body)) : {};
    const status = payload.action === 'disable' ? 'disabled' : 'installed';
    return new Response(JSON.stringify({ skills: [{ id: 'skill-real', name: payload.name || 'Real Skill', category: 'Testing', status }] }), { status: 200 });
  }) as any;

  const body = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
    action: 'disable',
    name: 'Real Skill',
  }))).json();
  expect(body.skills).toEqual([expect.objectContaining({ name: 'Real Skill', status: 'disabled' })]);
  expect(JSON.stringify(body)).not.toContain('Web Research');
});

test('connectors route proxies persisted connect and disconnect state', async () => {
  global.fetch = vi.fn(async (url: RequestInfo | URL, init?: RequestInit) => {
    expect(String(url)).toContain('/api/assistant/connectors');
    const payload = init?.body ? JSON.parse(String(init.body)) : {};
    const status = payload.action === 'disconnect' ? 'disconnected' : 'connected';
    return new Response(JSON.stringify({ connectors: [{ id: 'connector-real', name: payload.name || 'Real Connector', kind: payload.kind || 'custom', status }] }), { status: 200 });
  }) as any;

  const body = await (await patchConnectors(patchRequest('http://localhost/api/assistant/connectors', {
    action: 'disconnect',
    name: 'Real Connector',
  }))).json();
  expect(body.connectors).toEqual([expect.objectContaining({ name: 'Real Connector', status: 'disconnected' })]);
  expect(JSON.stringify(body)).not.toContain('MCP Endpoint');
});
```

- [ ] **Step 2: Run route tests and verify failure**

Run:

```bash
cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts
```

Expected: FAIL because routes still import `store.ts` and return local seeded data.

- [ ] **Step 3: Add shared proxy helpers inline**

In each of `memory/route.ts`, `skills/route.ts`, and `connectors/route.ts`, remove `store.ts` imports and add:

```ts
function backendUrl() {
  return process.env.BACKEND_URL || 'http://localhost:8080';
}

function backendHeaders(request?: Request) {
  const headers: Record<string, string> = {
    'x-tenant-id': request?.headers?.get('x-tenant-id') || 'storefront',
  };
  const authHeader = request?.headers?.get('Authorization');
  if (authHeader) headers.Authorization = authHeader;
  return headers;
}

async function upstreamJson(response: Response, fallbackMessage: string) {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    return NextResponse.json(
      { error: data.error || fallbackMessage },
      { status: response.status === 404 ? 404 : 502 },
    );
  }
  return NextResponse.json(data);
}
```

- [ ] **Step 4: Replace memory route implementation**

Use this code in `src/ui/next/src/app/api/assistant/memory/route.ts`:

```ts
import { NextResponse } from 'next/server';

function backendUrl() {
  return process.env.BACKEND_URL || 'http://localhost:8080';
}

function backendHeaders(request?: Request) {
  const headers: Record<string, string> = {
    'x-tenant-id': request?.headers?.get('x-tenant-id') || 'storefront',
  };
  const authHeader = request?.headers?.get('Authorization');
  if (authHeader) headers.Authorization = authHeader;
  return headers;
}

async function upstreamJson(response: Response, fallbackMessage: string) {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    return NextResponse.json({ error: data.error || fallbackMessage }, { status: response.status === 404 ? 404 : 502 });
  }
  return NextResponse.json(data);
}

export async function GET(request?: Request) {
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/memory`, {
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant memory unavailable');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'memory request failed'}` }, { status: 502 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/memory`, {
      method: 'PATCH',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    return upstreamJson(response, 'Assistant memory could not be updated');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'memory update failed'}` }, { status: 502 });
  }
}
```

- [ ] **Step 5: Replace skills route implementation**

Use this code in `src/ui/next/src/app/api/assistant/skills/route.ts`:

```ts
import { NextResponse } from 'next/server';

function backendUrl() {
  return process.env.BACKEND_URL || 'http://localhost:8080';
}

function backendHeaders(request?: Request) {
  const headers: Record<string, string> = {
    'x-tenant-id': request?.headers?.get('x-tenant-id') || 'storefront',
  };
  const authHeader = request?.headers?.get('Authorization');
  if (authHeader) headers.Authorization = authHeader;
  return headers;
}

async function upstreamJson(response: Response, fallbackMessage: string) {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    return NextResponse.json({ error: data.error || fallbackMessage }, { status: response.status === 404 ? 404 : 502 });
  }
  return NextResponse.json(data);
}

export async function GET(request?: Request) {
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/skills`, {
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant skills unavailable');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'skills request failed'}` }, { status: 502 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/skills`, {
      method: 'PATCH',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    return upstreamJson(response, 'Assistant skills could not be updated');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'skills update failed'}` }, { status: 502 });
  }
}
```

- [ ] **Step 6: Replace connectors route implementation**

Use this code in `src/ui/next/src/app/api/assistant/connectors/route.ts`:

```ts
import { NextResponse } from 'next/server';

function backendUrl() {
  return process.env.BACKEND_URL || 'http://localhost:8080';
}

function backendHeaders(request?: Request) {
  const headers: Record<string, string> = {
    'x-tenant-id': request?.headers?.get('x-tenant-id') || 'storefront',
  };
  const authHeader = request?.headers?.get('Authorization');
  if (authHeader) headers.Authorization = authHeader;
  return headers;
}

async function upstreamJson(response: Response, fallbackMessage: string) {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    return NextResponse.json({ error: data.error || fallbackMessage }, { status: response.status === 404 ? 404 : 502 });
  }
  return NextResponse.json(data);
}

export async function GET(request?: Request) {
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/connectors`, {
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant connectors unavailable');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'connectors request failed'}` }, { status: 502 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/connectors`, {
      method: 'PATCH',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    return upstreamJson(response, 'Assistant connector could not be updated');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'connector update failed'}` }, { status: 502 });
  }
}
```

- [ ] **Step 7: Remove task fallback and synthetic artifacts**

In `tasks/route.ts`:

- Remove `createAssistantTask` and `listAssistantTasks` imports.
- In `GET`, if backend fetch throws or is non-OK, return `502` JSON:

```ts
return NextResponse.json({ error: 'Assistant backend unavailable' }, { status: 502 });
```

- In `POST`, keep backend task creation and user message persistence if the backend accepts it, but delete the blocks that synthesize assistant messages and artifacts for `Code App` and `Presentation`.
- If backend task creation fails or throws, return:

```ts
return NextResponse.json({ error: 'Assistant backend unavailable' }, { status: 502 });
```

- [ ] **Step 8: Run route tests**

Run:

```bash
cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts
```

Expected: PASS for the updated covered-route assertions.

- [ ] **Step 9: Commit**

```bash
git add src/ui/next/src/app/api/assistant/tasks/route.ts src/ui/next/src/app/api/assistant/memory/route.ts src/ui/next/src/app/api/assistant/skills/route.ts src/ui/next/src/app/api/assistant/connectors/route.ts src/ui/next/src/app/api/assistant/route.test.ts
git commit -m "feat: proxy assistant feature state to backend"
```

---

### Task 5: Update Assistant UI Tests for Honest Empty and Error States

**Files:**
- Modify: `src/ui/next/src/app/assistant/page.test.tsx`

- [ ] **Step 1: Add UI tests that reject demo state**

In the test `beforeEach`, change the task GET response to an empty real response:

```ts
if (urlString.includes('/api/assistant/tasks')) {
  return new Response(JSON.stringify({
    tasks: [],
    capabilities: {
      outputFormats: ['Document', 'Presentation', 'PDF', 'Code App'],
      workModes: ['Ask', 'Agent', 'Plan', 'Coding'],
      modelProviders: ['Auto', 'Agent'],
    },
  }), { status: 200, headers: { 'Content-Type': 'application/json' } });
}
```

Add this test:

```ts
test('renders empty Assistant state without seeded demo records', async () => {
  renderAssistantPage();

  expect(await screen.findByRole('heading', { name: 'Agent Assistant' })).toBeDefined();
  expect(screen.getByText('0 tasks')).toBeDefined();
  expect(screen.getByText('No matching tasks.')).toBeDefined();
  expect(screen.queryByText("Create this week's operating brief")).toBeNull();
  expect(screen.queryByText('Organize Downloads by file type')).toBeNull();
});
```

Add this test:

```ts
test('shows resource error instead of connector demo records', async () => {
  global.fetch = vi.fn(async (url: RequestInfo | URL) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    if (urlString.includes('/api/assistant/tasks')) {
      return new Response(JSON.stringify({ tasks: [], capabilities: tasksPayload.capabilities }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/settings')) {
      return new Response(JSON.stringify({ settings: { agentName: 'Agent' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/connectors')) {
      return new Response(JSON.stringify({ error: 'Assistant backend unavailable' }), { status: 502, headers: { 'Content-Type': 'application/json' } });
    }
    return new Response(JSON.stringify({}), { status: 200, headers: { 'Content-Type': 'application/json' } });
  }) as any;

  renderAssistantPage();
  fireEvent.click(await screen.findByRole('button', { name: 'Connectors' }));

  expect(await screen.findByText('Assistant backend unavailable')).toBeDefined();
  expect(screen.queryByText('GitHub')).toBeNull();
  expect(screen.queryByText('Slack')).toBeNull();
});
```

- [ ] **Step 2: Remove tests that require seeded labels**

Delete or rewrite assertions that require:

```text
Create this week's operating brief
Organize Downloads by file type
GitHub
GitLab
Slack
Hourly
Daily
Weekly
One-time
212
```

For sections outside this pass, keep tests focused on rendering the section shell and backend-returned records supplied by the test stub.

- [ ] **Step 3: Run page tests**

Run:

```bash
cd src/ui/next && npm test -- src/app/assistant/page.test.tsx
```

Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/ui/next/src/app/assistant/page.test.tsx
git commit -m "test: require honest assistant empty states"
```

---

### Task 6: Final Verification

**Files:**
- Verify only.

- [ ] **Step 1: Search for covered fallback imports**

Run:

```bash
rg -n "listMemories|mutateMemory|listSkills|mutateSkill|listConnectors|mutateConnector|listAssistantTasks|createAssistantTask" src/ui/next/src/app/api/assistant
```

Expected: no matches in `tasks/route.ts`, `memory/route.ts`, `skills/route.ts`, or `connectors/route.ts`.

- [ ] **Step 2: Search for synthetic task artifacts in Next task route**

Run:

```bash
rg -n "Preview document|chart\\.png|presentation\\.pptx|I've received your request" src/ui/next/src/app/api/assistant/tasks/route.ts
```

Expected: no matches.

- [ ] **Step 3: Run covered frontend tests**

Run:

```bash
cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts src/app/assistant/page.test.tsx
```

Expected: PASS.

- [ ] **Step 4: Run backend Assistant tests**

Run:

```bash
bazel test //src/server/api:server_api_unit_test --test_filter=real_feature_state_tests
```

Expected: PASS.

- [ ] **Step 5: Commit any verification-only test fixes**

If verification required small fixes, commit them:

```bash
git add src/server/api/assistant.rs src/ui/next/src/app/api/assistant src/ui/next/src/app/assistant/page.test.tsx
git commit -m "fix: align assistant real data verification"
```

If no fixes were needed, do not create an empty commit.

---

## Self-Review

- Spec coverage: Tasks 1-3 implement backend/database persistence for memory, skills, and connectors. Task 4 removes Next demo fallbacks for tasks/results and covered feature tabs. Task 5 updates UI behavior for honest empty/error states. Task 6 verifies no covered fallback or synthetic result content remains.
- Completeness scan: this plan contains no incomplete implementation markers.
- Type consistency: backend response keys are `memories`, `skills`, and `connectors`; Next routes preserve those keys; UI resource config already reads those keys.
