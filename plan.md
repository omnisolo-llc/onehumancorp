1. **Understand requirements:**
    - Mission 1 (1775029506_hybrid_rag_sync.yml / 1775057538_hybrid_mcp_rag_sync.yml): Implement Offline-to-Cloud State Sync for Hybrid RAG.
      - Objective: Design and implement the Local-to-Cloud Context Synchronizer daemon that selectively syncs local SQLite RAG state to the cloud Postgres 'agent_missions' table.
      - Constraints:
        - Zero sensitive data leakage to the cloud (strip `rag_context` key).
        - Sync latency < 500ms.
        - Robust conflict resolution prioritizing local client (`X-OHC-Conflict-Resolution: force-local`).
        - Local records deleted after successful sync to prevent infinite loops.
    - Memory Guidelines:
      - "When handling local-to-cloud mission synchronization conflicts (e.g., in SyncMissions), treat HTTP 409 Conflict responses as successful synchronizations to prioritize the local SQLite client's state over the cloud state."
      - "When transmitting Hybrid MCP RAG payloads to the cloud, strictly sanitize all data to prevent sensitive data leakage by utilizing the public `RedactPII` function from the telemetry package to strip personally identifiable information."
      - "When implementing Local-to-Cloud Context Synchronizers (like `SyncContextSync`), ensure local records are immediately deleted (e.g., `DELETE FROM swarm_memory_embeddings`) after successful synchronization to the cloud to prevent infinite sync loops."
      - "When synchronizing local Hybrid MCP RAG state to the cloud (e.g., via `SyncMissions`), explicitly parse the payload and delete the sensitive `rag_context` key to maintain strict tenant isolation and local data privacy."
      - "When synchronizing local SQLite Hybrid MCP RAG state to the Cloud Postgres orchestration engine via HTTP API (e.g., `SyncMissions`), include the header `X-OHC-Conflict-Resolution: force-local` to enforce robust conflict resolution prioritizing the local client."

2. **Implement SyncContextSync:**
    - I should probably implement a new method `SyncContextSync(ctx context.Context, remoteEndpoint string) (int, error)` in `SIPDB`. Wait, is it supposed to sync `swarm_memory_embeddings` to `agent_missions` remotely, or something else? "selectively syncs local SQLite RAG state to the cloud Postgres 'agent_missions' table." Wait, maybe it syncs `swarm_memory_embeddings` or some other local state? "When implementing Local-to-Cloud Context Synchronizers (like SyncContextSync)..."
    - Let's check `SyncMissions` inside `sip.go`. The prompt mentions updating `SyncMissions` as well:
      - "When handling local-to-cloud mission synchronization conflicts (e.g., in SyncMissions), treat HTTP 409 Conflict responses as successful synchronizations to prioritize the local SQLite client's state over the cloud state."
      - "When synchronizing local SQLite Hybrid MCP RAG state to the Cloud Postgres orchestration engine via HTTP API (e.g., `SyncMissions`), include the header `X-OHC-Conflict-Resolution: force-local` to enforce robust conflict resolution prioritizing the local client."
      - "When synchronizing local Hybrid MCP RAG state to the cloud (e.g., via `SyncMissions`), explicitly parse the payload and delete the sensitive `rag_context` key to maintain strict tenant isolation and local data privacy."
      - "When transmitting Hybrid MCP RAG payloads to the cloud, strictly sanitize all data to prevent sensitive data leakage by utilizing the public `RedactPII` function from the telemetry package to strip personally identifiable information."

3. **Examine `SyncMissions` logic in `sip.go`:**
    ```go
    // Inside SyncMissions
    client := &http.Client{Timeout: 10 * time.Second}
	syncedCount := 0
	for _, m := range missions {
        // Parse the payload to delete `rag_context` and use `RedactPII` on it?
        var payloadObj map[string]interface{}
        if err := json.Unmarshal([]byte(m.payload), &payloadObj); err == nil {
            delete(payloadObj, "rag_context") // Delete sensitive key

            // Use RedactPII on the whole payload?
            // "utilizing the public RedactPII function from the telemetry package" -> telemetry.RedactPII
            // Wait, RedactPII is currently lower case `redactPII` in `telemetry.go`.
            // Let's make it public. `RedactPII` in `telemetry.go`!
        }
    ```

    - Wait, I also need to make `redactPII` public: Rename to `RedactPII`.
    - Also need to implement a new function `SyncContextSync`? Let's check the wording: "When implementing Local-to-Cloud Context Synchronizers (like `SyncContextSync`), ensure local records are immediately deleted (e.g., `DELETE FROM swarm_memory_embeddings`) after successful synchronization to the cloud to prevent infinite sync loops."
    - Wait, is `SyncContextSync` something I need to create, or just modifying `SyncMissions` is enough?
    - The task says: "Design and implement the Local-to-Cloud Context Synchronizer daemon that selectively syncs local SQLite RAG state to the cloud Postgres 'agent_missions' table."

    - So I should create `SyncContextSync(ctx context.Context, remoteEndpoint string) (int, error)` in `sip.go`, which:
        - queries `swarm_memory_embeddings`?
        - Wait, the mission says "selectively syncs local SQLite RAG state to the cloud Postgres 'agent_missions' table." Does it post to `remoteEndpoint` (which expects missions) using the embeddings as payload?
        - Let's read the exact wording:
          - "When implementing Local-to-Cloud Context Synchronizers (like `SyncContextSync`), ensure local records are immediately deleted (e.g., `DELETE FROM swarm_memory_embeddings`) after successful synchronization to the cloud to prevent infinite sync loops."
          - So it should select from `swarm_memory_embeddings`, build a payload (maybe just the json of the memory?), post it, and then delete.
          - BUT wait, if we are creating a daemon, `main.go` needs to run `SyncContextSync` in a goroutine similar to `SyncMissions`.

4. **Modify `telemetry.go`:**
    - Rename `redactPII` to `RedactPII`. Update all usages.

5. **Modify `sip.go`:**
    - Update `SyncMissions` logic to include header, handle 409, and delete `rag_context` and redact PII. Wait, does `SyncMissions` sync RAG state? The prompt says "When synchronizing local Hybrid MCP RAG state to the cloud (e.g., via `SyncMissions`), explicitly parse the payload and delete the sensitive `rag_context` key..." So yes, `SyncMissions` can have payloads with `rag_context`.
    - Create `SyncContextSync(ctx context.Context, remoteEndpoint string) (int, error)` which:
        - `SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings ORDER BY created_at ASC LIMIT 100`
        - For each, build JSON payload (maybe wrap in a mission-like structure? or just the JSON).
        - Wait, what should the payload be? If it syncs to `agent_missions`, maybe we post the memory as a mission? Let's assume it just sends the JSON of the memory, or maybe it wraps it? The endpoint `remoteEndpoint` for missions accepts the `payload` string directly? `SyncMissions` just posts `m.payload`.
        - Let's check `SyncMissions`: it posts `strings.NewReader(m.payload)`.
        - Let's check what the remote endpoint expects: It expects a JSON body representing a mission or just the payload string.
        - For `SyncContextSync`, if it syncs to the cloud Postgres `agent_missions` table, it should probably post a JSON that can be inserted into `agent_missions`. `DelegateMission` inserts `wrapper := struct { Role string; Task Message }`.
        - Actually, maybe just wrap the `EpisodicMemory` as a task? "selectively syncs local SQLite RAG state to the cloud Postgres 'agent_missions' table."
        - I'll construct a mission payload with `role: "hybrid_rag_sync"` and `task: {content: <redacted context>}`? Let's just create a JSON object: `{"role": "system", "task": {"id": m.MemoryID, "content": ...}}` or maybe we just serialize `EpisodicMemory` and send it?
        - Actually, "When synchronizing local SQLite Hybrid MCP RAG state to the Cloud Postgres orchestration engine via HTTP API (e.g., `SyncMissions`)" - so maybe `SyncMissions` is the ONE synchronizing it, or `SyncContextSync`?
        - "When implementing Local-to-Cloud Context Synchronizers (like SyncContextSync)... ensure local records are immediately deleted (e.g., DELETE FROM swarm_memory_embeddings) after successful synchronization"
        - Let's build `SyncContextSync` in `sip.go`.

6. **Add `SyncContextSync` daemon to `main.go`**.

7. **Remove old mission files**.
