1. **Update API router and implement handleMeshV2Broadcast**: Add `mux.HandleFunc("/api/mesh/v2/broadcast", auth.RequireRole("system", server.handleMeshV2Broadcast))` in `srcs/server/dashboard/server.go`, and add `handleMeshV2Broadcast` method implementation using `run_in_bash_session` to execute a script:
```bash
cat << 'EOF2' > patch.py
import sys

with open("srcs/server/dashboard/server.go", "r") as f:
    content = f.read()

content = content.replace('mux.HandleFunc("/api/mesh/broadcast", auth.RequireRole("system", server.handleMeshBroadcast))', 'mux.HandleFunc("/api/mesh/broadcast", auth.RequireRole("system", server.handleMeshBroadcast))\n\tmux.HandleFunc("/api/mesh/v2/broadcast", auth.RequireRole("system", server.handleMeshV2Broadcast))')

new_handler = """
func (s *Server) handleMeshV2Broadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	var req struct {
		Channel string                 `json:"channel"`
		Data    map[string]interface{} `json:"data"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	payloadBytes, err := json.Marshal(req.Data)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	err = s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   "system",
		Type:      req.Channel,
		Content:   string(payloadBytes),
	})

	if err == nil {
		telemetry.RecordTeammateMeshBroadcast(r.Context(), req.Channel)

		// Map mesh channels to Centrifuge WebSocket channels for UI updates
		if s.hub != nil && s.hub.CentrifugeNode() != nil {
			if req.Channel == "mesh:tasks" {
				s.hub.CentrifugeNode().PublishTaskBroadcast(fmt.Sprintf("%d", time.Now().UnixNano()), req.Data)
			} else if req.Channel == "mesh:coordination" {
				s.hub.CentrifugeNode().PublishCoordinationMessage(orchestration.Message{
					ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
					FromAgent: "system",
					ToAgent:   "system",
					Type:      req.Channel,
					Content:   string(payloadBytes),
				})
			}
		}
	} else {
		http.Error(w, "failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}
"""

content = content + "\n" + new_handler

with open("srcs/server/dashboard/server.go", "w") as f:
    f.write(content)

EOF2
python3 patch.py && rm patch.py && git diff srcs/server/dashboard/server.go
```
2. **Write Tests**: Write tests for `handleMeshV2Broadcast` in `srcs/server/dashboard/mesh_test.go` using `run_in_bash_session`:
```bash
cat << 'EOF2' > patch_test.py
import sys

with open("srcs/server/dashboard/mesh_test.go", "r") as f:
    content = f.read()

new_tests = """
func TestHandleMeshV2Broadcast(t *testing.T) {
	org := domain.Organization{ID: "org-mesh"}
	hub := orchestration.NewHub("test-mesh-db", "memory://")
	srv := &Server{
		org: org,
		hub: hub,
	}

	cn, _ := orchestration.NewCentrifugeNode()
	hub.SetCentrifugeNode(cn)

	t.Run("RequiresmTLS", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodPost, "/api/mesh/v2/broadcast", nil, false)
		w := httptest.NewRecorder()

		srv.handleMeshV2Broadcast(w, req)

		if w.Code != http.StatusForbidden {
			t.Errorf("expected status 403 Forbidden, got %v", w.Code)
		}
	})

	t.Run("ValidBroadcastMeshTasks", func(t *testing.T) {
		payload := map[string]interface{}{
			"channel": "mesh:tasks",
			"data": map[string]interface{}{
				"event": "status_update",
				"status": "IN_PROGRESS",
			},
		}
		body, _ := json.Marshal(payload)

		req := createMockTLSRequest(http.MethodPost, "/api/mesh/v2/broadcast", body, true)

		// Setup context to bypass auth middleware but satisfy auth role check
		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{
			Role: "system",
			OrganizationID: "org-mesh",
		})
		req = req.WithContext(ctx)

		w := httptest.NewRecorder()

		srv.handleMeshV2Broadcast(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected status 200 OK, got %v", w.Code)
		}
	})
}
"""

content = content + "\n" + new_tests

with open("srcs/server/dashboard/mesh_test.go", "w") as f:
    f.write(content)

EOF2
python3 patch_test.py && rm patch_test.py && git diff srcs/server/dashboard/mesh_test.go
```
3. **Run tests**: Run `export PATH=$PATH:/home/jules/go/bin && bazelisk test //srcs/server/dashboard/...`.
4. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done**: Run `pre_commit_instructions` tool to run checks.
5. **Submit**: Use the `submit` tool to create the pull request and finalize the task.
