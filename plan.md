1. **Restrict Teammate Mesh WebSocket origins in Cloud mode:**
   - Modify `upgrader` in `src/server/api/mesh/mesh.go`.
   - Instead of a global hardcoded `upgrader`, I'll update it to check `OHC_MULTITENANT`. Wait, since `mesh.go` might not have `envBoolDefault`, I'll redefine it locally or just use `os.Getenv("OHC_MULTITENANT")`.
   - Update `upgrader` initialization in `mesh.go` or `Stream` function:
     ```go
     var upgrader = websocket.Upgrader{
         ReadBufferSize:  1024,
         WriteBufferSize: 1024,
         CheckOrigin: func(r *http.Request) bool {
             isCloud := os.Getenv("OHC_MULTITENANT") == "true" || os.Getenv("OHC_MULTITENANT") == "1" || (os.Getenv("OHC_MULTITENANT") == "" && true) // wait, default is true. Actually, let's use a local envBoolDefault.
             // Actually, `envBoolDefault("OHC_MULTITENANT", true)`
             return true // Wait, I need to know how to restrict it. The issue says "Restrict Teammate Mesh WebSocket origins in Cloud mode for enhanced security". Is it enough to just reject if it doesn't match the host?
             // e.g. origin == "https://app.onehumancorp.com"
         },
     }
     ```
     Wait, I can just use `websocket.Upgrader` default behaviour (which checks if Origin matches Host) but we might need to allow specific domains if it's cloud mode. Let's look at standard practice or check if there's any instruction about allowed domains. The issue just says "Restrict Teammate Mesh WebSocket origins in Cloud mode for enhanced security."
     I will implement:
     ```go
     CheckOrigin: func(r *http.Request) bool {
         isCloud := true
         if val := os.Getenv("OHC_MULTITENANT"); val != "" {
             isCloud = (val == "true" || val == "1")
         }
         if isCloud {
             origin := r.Header.Get("Origin")
             return origin == "" || origin == "http://localhost:3000" || origin == "https://app.onehumancorp.com" || origin == "https://www.onehumancorp.com"
         }
         return true
     }
     ```

2. **Harden the Standalone Wrapper (`src/server/standalone_ohc.sh`):**
   - Refine cleanup logic for runaway processes and bloated tmp files.
   - In `cleanup_tmp_files()` in `standalone_ohc.sh`:
     ```bash
     find "${STATE_DIR}" -name "*.tmp" -type f -size +100M -delete 2>/dev/null || true
     ```
     Also refine runaway process cleanup in `stop_daemon()`:
     ```bash
     # We do NOT remove the PID file if the process did not stop, preventing it from being orphaned and becoming a runaway.
     # Wait, let's forcefully kill it instead of leaving it runaway.
     if ! kill -0 "${pid}" 2>/dev/null; then
        ...
     fi
     kill -9 "${pid}" 2>/dev/null || true
     pkill -9 -P "${pid}" 2>/dev/null || true
     ```

3. **Phase 2 cleanup protocol: Linear artifact pruning:**
   - In `src/server/standalone_ohc.sh`'s `cleanup_tmp_files`:
     The script currently has:
     ```bash
     # Prune legacy Linear artifacts if they leak into the standalone environment
     find "${STATE_DIR}" -name "*linear*" -type f -delete 2>/dev/null || true
     ```
     Wait, `*linear*` is case-sensitive! What if it's `Linear-state.tmp`? It is already cleaned by `find "${STATE_DIR}" -name "Linear-*.tmp" -type f -delete`.
     BUT what if it's `Linear` directory or something else?
     Let's use `-iname "*linear*"` to catch all case variants, like `*Linear*`.
     Change:
     ```bash
     find "${STATE_DIR}" -iname "*linear*" -delete 2>/dev/null || true
     ```
     Actually, let's look at `standalone_cleanup_test.sh`:
     ```bash
     touch "${STATE_DIR}/Linear-state.tmp"
     touch "${STATE_DIR}/some_linear_junk"
     ```

4. **Consolidate AutoDream memory ingestion and consolidation logic into a unified service:**
   - Update `src/server/agents/kairos/autodream_worker.go` to insert into `consolidated_memory` instead of `autodream_memories`.
   - Update `insertQuery := ... INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, 'shared_tasks')`
   - Oh, I need to make sure the schema matches:
     ```sql
     CREATE TABLE IF NOT EXISTS consolidated_memory (
         id TEXT PRIMARY KEY,
         organization_id TEXT NOT NULL,
         agent_id TEXT,
         content TEXT NOT NULL,
         embedding VECTOR(1536),
         source_type TEXT NOT NULL,
         created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
     );
     ```

