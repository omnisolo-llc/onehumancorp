1. **Create the KAIROS schemas migration scripts:**
   Use `run_in_bash_session` to write the following Postgres migration using `cat << 'EOF' > srcs/server/db/migrations/20260419060000_kairos_master_orchestration_pg.sql`:
   ```sql
   -- +goose Up
   -- +goose StatementBegin
   CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
       id TEXT PRIMARY KEY,
       organization_id TEXT NOT NULL,
       parent_plan_id TEXT,
       title TEXT NOT NULL,
       status TEXT NOT NULL DEFAULT 'PENDING',
       assigned_agent_id TEXT,
       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE IF NOT EXISTS kairos_state_transitions (
       id TEXT PRIMARY KEY,
       task_id TEXT,
       from_state TEXT NOT NULL,
       to_state TEXT NOT NULL,
       agent_id TEXT NOT NULL,
       reason TEXT,
       occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
       id TEXT PRIMARY KEY,
       organization_id TEXT NOT NULL,
       parent_task_id TEXT,
       payload JSONB,
       status TEXT NOT NULL DEFAULT 'QUEUED',
       worker_id TEXT,
       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
   );

   CREATE EXTENSION IF NOT EXISTS vector;

   CREATE TABLE IF NOT EXISTS autodream_vector_memories (
       id TEXT PRIMARY KEY,
       source_mission_id TEXT,
       content TEXT NOT NULL,
       embedding vector(1536),
       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
   );
   -- +goose StatementEnd

   -- +goose Down
   -- +goose StatementBegin
   DROP TABLE IF EXISTS autodream_vector_memories;
   DROP TABLE IF EXISTS kairos_sub_agent_jobs;
   DROP TABLE IF EXISTS kairos_state_transitions;
   DROP TABLE IF EXISTS kairos_shared_tasks;
   -- +goose StatementEnd
   ```

   And the SQLite migration using `cat << 'EOF' > srcs/server/db/migrations/20260419060000_kairos_master_orchestration_sqlite.sql`:
   ```sql
   -- +goose Up
   -- +goose StatementBegin
   CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
       id TEXT PRIMARY KEY,
       organization_id TEXT NOT NULL,
       parent_plan_id TEXT,
       title TEXT NOT NULL,
       status TEXT NOT NULL DEFAULT 'PENDING',
       assigned_agent_id TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
       updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE IF NOT EXISTS kairos_state_transitions (
       id TEXT PRIMARY KEY,
       task_id TEXT,
       from_state TEXT NOT NULL,
       to_state TEXT NOT NULL,
       agent_id TEXT NOT NULL,
       reason TEXT,
       occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
       id TEXT PRIMARY KEY,
       organization_id TEXT NOT NULL,
       parent_task_id TEXT,
       payload TEXT,
       status TEXT NOT NULL DEFAULT 'QUEUED',
       worker_id TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
       updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE IF NOT EXISTS autodream_vector_memories (
       id TEXT PRIMARY KEY,
       source_mission_id TEXT,
       content TEXT NOT NULL,
       embedding TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   -- +goose StatementEnd

   -- +goose Down
   -- +goose StatementBegin
   DROP TABLE IF EXISTS autodream_vector_memories;
   DROP TABLE IF EXISTS kairos_sub_agent_jobs;
   DROP TABLE IF EXISTS kairos_state_transitions;
   DROP TABLE IF EXISTS kairos_shared_tasks;
   -- +goose StatementEnd
   ```

2. **Verify migrations creation:**
   Run `cat srcs/server/db/migrations/20260419060000_kairos_master_orchestration_pg.sql srcs/server/db/migrations/20260419060000_kairos_master_orchestration_sqlite.sql` via `run_in_bash_session` to verify the files are successfully created.

3. **Update the Teammate Mesh API endpoint (`POST /api/mesh/broadcast`):**
   I will use a Python script via `run_in_bash_session` to replace the parsing logic in `srcs/server/api/mesh/mesh.go` within `Broadcast` method of `MeshHandler`.
   ```python
   import re
   with open('srcs/server/api/mesh/mesh.go', 'r') as f:
       content = f.read()

   old_code = """	var req struct {
		Intent string `json:"intent"`
	}
	var sipReq struct {
		AgentID   string          `json:"agent_id"`
		Channel   string          `json:"channel"`
		EventType string          `json:"event_type"`
		Data      json.RawMessage `json:"data"`
	}

	var intentStr string

	if err := json.Unmarshal(bodyBytes, &sipReq); err == nil && sipReq.AgentID != "" && sipReq.EventType != "" {
		intentStr = string(bodyBytes)
	} else if err := json.Unmarshal(bodyBytes, &req); err == nil && req.Intent != "" {
		intentStr = req.Intent
	} else {
		intentStr = string(bodyBytes)
	}"""

   new_code = """	var sipReq struct {
		AgentID   string          `json:"agent_id"`
		Channel   string          `json:"channel"`
		EventType string          `json:"event_type"`
		Data      json.RawMessage `json:"data"`
	}

	if err := json.Unmarshal(bodyBytes, &sipReq); err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	if sipReq.AgentID == "" || sipReq.Channel == "" || sipReq.EventType == "" || len(sipReq.Data) == 0 || string(sipReq.Data) == "null" {
		http.Error(w, "Invalid payload: missing required OHC-SIP fields (agent_id, channel, event_type, data)", http.StatusBadRequest)
		return
	}

	intentStr := string(bodyBytes)"""

   content = content.replace(old_code, new_code)
   with open('srcs/server/api/mesh/mesh.go', 'w') as f:
       f.write(content)
   ```

4. **Fix an existing test that uses an old payload format:**
   I will use a Python script via `run_in_bash_session` to update `TestMeshHandlerBroadcastEvent` in `srcs/server/api/mesh/mesh_test.go` so it provides all OHC-SIP required fields (`agent_id`, `channel`, `event_type`, `data`):
   ```python
   with open('srcs/server/api/mesh/mesh_test.go', 'r') as f:
       content = f.read()

   old_code = """    body := `{"agent_id": "worker-1", "channel": "orchestration.tasks", "action": "TaskTransition", "status": "success", "payload": {}}`"""
   new_code = """    body := `{"agent_id": "worker-1", "channel": "orchestration.tasks", "event_type": "TaskTransition", "data": {"status": "success"}}`"""

   content = content.replace(old_code, new_code)
   with open('srcs/server/api/mesh/mesh_test.go', 'w') as f:
       f.write(content)
   ```

5. **Add Unit Tests for Mesh API Validation:**
   I will use a Python script via `run_in_bash_session` to append the new validation test cases to `srcs/server/api/mesh/mesh_test.go`:
   ```python
   with open('srcs/server/api/mesh/mesh_test.go', 'a') as f:
       f.write("""
func TestMeshHandlerBroadcastValidation(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	tests := []struct {
		name       string
		payload    string
		statusCode int
	}{
		{
			name:       "valid sip payload",
			payload:    `{"agent_id":"xyz","channel":"mesh:tasks","event_type":"TASK_TRANSITION","data":{"task_id":"123"}}`,
			statusCode: http.StatusOK,
		},
		{
			name:       "missing agent_id",
			payload:    `{"channel":"mesh:tasks","event_type":"TASK_TRANSITION","data":{"task_id":"123"}}`,
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "missing channel",
			payload:    `{"agent_id":"xyz","event_type":"TASK_TRANSITION","data":{"task_id":"123"}}`,
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "missing event_type",
			payload:    `{"agent_id":"xyz","channel":"mesh:tasks","data":{"task_id":"123"}}`,
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "missing data",
			payload:    `{"agent_id":"xyz","channel":"mesh:tasks","event_type":"TASK_TRANSITION"}`,
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "null data",
			payload:    `{"agent_id":"xyz","channel":"mesh:tasks","event_type":"TASK_TRANSITION","data":null}`,
			statusCode: http.StatusBadRequest,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(tt.payload)))
			req = req.WithContext(ctx)
			w := httptest.NewRecorder()

			handler.Broadcast(w, req)

			if w.Code != tt.statusCode {
				t.Errorf("expected status %d, got %d for %s", tt.statusCode, w.Code, tt.name)
			}
		})
	}
}
""")
   ```

6. **Verify code changes:**
   Run `git diff` via `run_in_bash_session` to confirm the edits in `mesh.go` and `mesh_test.go` are correct.

7. **Verify Tests:**
   Run `./bazelisk test //...` via `run_in_bash_session` to ensure everything passes and 100% coverage is maintained across the repository.

8. **Complete Pre-Commit Steps:**
   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Submit the PR:**
   Submit the PR with branch `maintainer-kairos-orchestration-schema` and details:
   - **Title:** `🧹 Maintainer: [Orchestration] Implement KAIROS Master Orchestration DB schemas & Mesh API`
   - **Description:**
     - **What:** Added KAIROS orchestration schemas (`kairos_shared_tasks`, `kairos_state_transitions`, `kairos_sub_agent_jobs`, `autodream_vector_memories`) and enforced OHC-SIP payload specifications on `/api/mesh/broadcast`.
     - **Why:** The OHC swarm requires a durable state machine to decompose goals and a strict Mesh API to broadcast real-time state changes reliably.
     - **Impact:** The KAIROS swarm can now coordinate tasks reliably across Hybrid Architecture using Postgres JSONB/pgvector or gracefully degraded SQLite.
     - **Measurement:** `bazelisk test //...` coverage on DB schemas and 100% test passing for mesh payload validation.
