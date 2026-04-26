package dashboard

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server_old/auth"
	"github.com/onehumancorp/mono/src/server_old/billing"
	"github.com/onehumancorp/mono/src/server_old/domain"
	"github.com/onehumancorp/mono/src/server_old/orchestration"

	"encoding/json"
	"bytes"
	"context")

func TestHandleMCPInvokeCoverage(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	_, err := authStore.CreateUser("adminuser", "admin@test.com", "adminpass123", []string{"admin"}, "")
	if err != nil {
		t.Fatal("create user failed", err)
	}
	user, err := authStore.Authenticate("adminuser", "adminpass123", "")
	if err != nil {
		t.Fatal("auth failed", err)
	}
	token, _ := authStore.IssueToken(user)

	srv := &Server{org: org, hub: hub, tracker: tracker, authStore: authStore}

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/api/mcp/invoke", nil)
		w := httptest.NewRecorder()
		srv.handleMCPInvoke(w, req)
		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("expected 405, got %d", w.Code)
		}
	})

	t.Run("invalid json", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{invalid}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPInvoke(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", w.Code)
		}
	})

	t.Run("missing toolId", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"params": {"a": "b"}}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPInvoke(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", w.Code)
		}
	})

	t.Run("missing params", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "toolId": "dummy"}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPInvoke(w, req)
		if w.Code != http.StatusNotFound {
			t.Errorf("expected 404, got %d", w.Code)
		}
	})

	t.Run("success_valid_tool_no_meeting_id", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "toolId": "dummy", "params": {"a": "b"}}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPInvoke(w, req)
		if w.Code != http.StatusNotFound {
			t.Errorf("expected 404, got %d", w.Code)
		}
	})

	t.Run("large payload", func(t *testing.T) {
		// generate > 1MB string
		largeStr := strings.Repeat("a", 2<<20)
		req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "toolId": "dummy", "params": {"a": "`+largeStr+`"}}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPInvoke(w, req)
		// Should fail due to MaxBytesReader
		if w.Code != http.StatusBadRequest && w.Code != http.StatusRequestEntityTooLarge {
			t.Errorf("expected 400 or 413, got %d", w.Code)
		}
	})

	t.Run("success valid tool", func(t *testing.T) {
		// Register a dummy meeting
		hub.OpenMeeting("m-1", []string{})

		req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "toolId": "dummy-tool", "params": {"a": "b"}}`))
		req.Header.Set("Authorization", "Bearer "+token)
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		handler := auth.Middleware(authStore)(http.HandlerFunc(srv.handleMCPInvoke))
		handler.ServeHTTP(w, req)

		if w.Code != http.StatusNotFound {
			t.Errorf("expected 404, got %d (body: %s)", w.Code, w.Body.String())
		}
	})
}

func TestHandleMCPInvoke_MissingSPIFFEID(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	srv := &Server{org: org, hub: hub, tracker: tracker, authStore: authStore}

	req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"toolId": "dummy-tool", "params": {"a": "b"}}`))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()
	srv.handleMCPInvoke(w, req)

	if w.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden for missing SPIFFE ID, got %d", w.Code)
	}
}

func TestHandleMCPInvoke_InvalidSPIFFEID(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	srv := &Server{org: org, hub: hub, tracker: tracker, authStore: authStore}

	req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"spiffeId": "spiffe://evil-hacker.com/agent/1", "toolId": "dummy-tool", "params": {"a": "b"}}`))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()
	srv.handleMCPInvoke(w, req)

	if w.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden for invalid SPIFFE ID, got %d", w.Code)
	}
}

func TestHandleMCPRegister_Dynamic(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	srv := &Server{org: org, hub: hub, tracker: tracker, authStore: authStore, dynamicMCPTools: []MCPTool{}}

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/api/mcp/tools/register", nil)
		w := httptest.NewRecorder()
		srv.handleMCPRegister(w, req)
		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("expected 405, got %d", w.Code)
		}
	})

	t.Run("invalid json", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/tools/register", strings.NewReader(`{invalid}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPRegister(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", w.Code)
		}
	})

	t.Run("missing toolId or name", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/tools/register", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "tool": {"id": ""}}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPRegister(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", w.Code)
		}
	})

	t.Run("invalid spiffe id", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/tools/register", strings.NewReader(`{"spiffeId": "invalid-spiffe", "tool": {"id": "my-tool", "name": "My Tool"}}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPRegister(w, req)
		if w.Code != http.StatusForbidden {
			t.Errorf("expected 403, got %d", w.Code)
		}
	})

	t.Run("success valid spiffe id", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/tools/register", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "tool": {"id": "my-tool", "name": "My Tool"}}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPRegister(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		// Verify dynamicMCPTools has 1 entry
		if len(srv.dynamicMCPTools) != 1 {
			t.Errorf("expected 1 tool in dynamicMCPTools, got %d", len(srv.dynamicMCPTools))
		}
		if srv.dynamicMCPTools[0].ID != "my-tool" {
			t.Errorf("expected tool ID to be my-tool, got %s", srv.dynamicMCPTools[0].ID)
		}
	})

	t.Run("duplicate tool registration updates existing", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/mcp/tools/register", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "tool": {"id": "my-tool", "name": "My Updated Tool"}}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleMCPRegister(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		// Verify dynamicMCPTools still has 1 entry, but updated
		if len(srv.dynamicMCPTools) != 1 {
			t.Errorf("expected 1 tool in dynamicMCPTools, got %d", len(srv.dynamicMCPTools))
		}
		if srv.dynamicMCPTools[0].Name != "My Updated Tool" {
			t.Errorf("expected tool Name to be My Updated Tool, got %s", srv.dynamicMCPTools[0].Name)
		}
	})
}

func TestHandleContextSync(t *testing.T) {
	hub := orchestration.NewHub()
	defer hub.Close()
	// Create an in-memory SIPDB
	sipdb, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create sipdb: %v", err)
	}
	hub.SetSIPDB(sipdb)

	srv := &Server{
		org: domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now()),
		hub: hub,
	}

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/api/context/sync", nil)
		w := httptest.NewRecorder()
		srv.handleContextSync(w, req)
		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("expected 405, got %d", w.Code)
		}
	})

	t.Run("invalid payload", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/context/sync", strings.NewReader(`{invalid}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleContextSync(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", w.Code)
		}
	})

	t.Run("successful sync with PII redaction", func(t *testing.T) {
		payload := `{"memory_id": "test-mem-1", "context": {"user_email": "alice@example.com", "nested": ["some text", "bob@example.com"]}, "source_plugin": "test-plugin"}`
		req := httptest.NewRequest("POST", "/api/context/sync", strings.NewReader(payload))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleContextSync(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		// Verify it was stored correctly and PII redacted
		memories, err := sipdb.GetEpisodicMemoriesByPlugin(req.Context(), "test-plugin")
		if err != nil {
			t.Fatalf("GetEpisodicMemoriesByPlugin failed: %v", err)
		}
		if len(memories) != 1 {
			t.Fatalf("expected 1 memory stored, got %d", len(memories))
		}
		if memories[0].MemoryID != "test-mem-1" {
			t.Errorf("expected MemoryID test-mem-1, got %s", memories[0].MemoryID)
		}

		if !strings.Contains(memories[0].Context, "[REDACTED_EMAIL]") {
			t.Errorf("expected context to contain [REDACTED_EMAIL], got %s", memories[0].Context)
		}
		if strings.Contains(memories[0].Context, "alice@example.com") || strings.Contains(memories[0].Context, "bob@example.com") {
			t.Errorf("expected context to NOT contain original emails, got %s", memories[0].Context)
		}
	})
}

func TestHandleMCPTools(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	srv := &Server{org: org, hub: hub, tracker: tracker, authStore: authStore, dynamicMCPTools: []MCPTool{{ID: "test-tool"}}}

	req := httptest.NewRequest("GET", "/api/mcp/tools", nil)
	w := httptest.NewRecorder()
	srv.handleMCPTools(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", w.Code)
	}

	if !strings.Contains(w.Body.String(), "test-tool") {
		t.Errorf("expected response to contain test-tool, got %s", w.Body.String())
	}
}

func TestHandleHybridSyncMissions(t *testing.T) {
	hub := orchestration.NewHub()
	defer hub.Close()

	// Create an in-memory SIPDB
	sipdb, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create sipdb: %v", err)
	}
	hub.SetSIPDB(sipdb)

	cnNode, err := orchestration.NewCentrifugeNode()
	if err == nil {
		hub.SetCentrifugeNode(cnNode)
	}

	srv := &Server{
		org: domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now()),
		hub: hub,
	}

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/api/sync/missions", nil)
		w := httptest.NewRecorder()
		srv.handleHybridSyncMissions(w, req)
		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("expected 405, got %d", w.Code)
		}
	})

	t.Run("invalid json", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/sync/missions", strings.NewReader(`{invalid}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleHybridSyncMissions(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", w.Code)
		}
	})

	t.Run("empty payload", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/sync/missions", strings.NewReader(`[]`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleHybridSyncMissions(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}
		if !strings.Contains(w.Body.String(), "no missions to sync") {
			t.Errorf("expected 'no missions to sync', got %s", w.Body.String())
		}
	})

	t.Run("success sync missions", func(t *testing.T) {
		payload := `[{"id": "m-1", "status": "CLOUD_ESCALATION", "payload": "{\"agent_id\": \"agent-1\", \"action\": \"test\"}"}]`
		req := httptest.NewRequest("POST", "/api/sync/missions", strings.NewReader(payload))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleHybridSyncMissions(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		if !strings.Contains(w.Body.String(), "synced_count\":1") {
			t.Errorf("expected 'synced_count\":1', got %s", w.Body.String())
		}
	})

	t.Run("skips invalid items", func(t *testing.T) {
		payload := `[{"id": "", "status": "CLOUD_ESCALATION", "payload": "{}"}]`
		req := httptest.NewRequest("POST", "/api/sync/missions", strings.NewReader(payload))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleHybridSyncMissions(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		if !strings.Contains(w.Body.String(), "synced_count\":0") {
			t.Errorf("expected 'synced_count\":0', got %s", w.Body.String())
		}
	})
}


func TestHandleSyncRAG_ConflictResolution(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	// Create mock SIPDB
	db, err := orchestration.NewSIPDB("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to init sipdb: %v", err)
	}
	hub.SetSIPDB(db)

	srv := &Server{org: org, hub: hub}

	payload := map[string]interface{}{
		"memory_id":        "ctx-conflict-1",
		"context":          "some data",
		"source_plugin":    "test-plugin",
		"vector_embedding": []float64{0.1, 0.2},
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPost, "/api/orchestration/sync/rag", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")
	rr := httptest.NewRecorder()

	srv.handleSyncRAG(rr, req)

	if rr.Code != http.StatusConflict {
		t.Fatalf("expected 409 Conflict, got %d", rr.Code)
	}

	// Verify memory was still stored correctly despite the conflict header
	memories, err := db.GetEpisodicMemoriesByPlugin(context.Background(), "test-plugin")
	if err != nil || len(memories) == 0 {
		t.Fatalf("memory not found in sipdb")
	}
	if memories[0].MemoryID != "ctx-conflict-1" {
		t.Errorf("expected memory ID ctx-conflict-1, got %v", memories[0].MemoryID)
	}
}

func TestHandleSyncRAG_Success(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	// Create mock SIPDB
	db, err := orchestration.NewSIPDB("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to init sipdb: %v", err)
	}
	hub.SetSIPDB(db)

	srv := &Server{org: org, hub: hub}

	payload := map[string]interface{}{
		"memory_id":     "ctx-success-1",
		"context":       "normal context",
		"source_plugin": "test-plugin-success",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPost, "/api/orchestration/sync/rag", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	srv.handleSyncRAG(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200 OK, got %d", rr.Code)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if resp["status"] != "success" {
		t.Errorf("expected status success, got %v", resp["status"])
	}

	// Verify it was stored
	memories, err := db.GetEpisodicMemoriesByPlugin(context.Background(), "test-plugin-success")
	if err != nil || len(memories) == 0 {
		t.Fatalf("memory not found in sipdb")
	}
	if memories[0].MemoryID != "ctx-success-1" {
		t.Errorf("expected memory ID ctx-success-1, got %v", memories[0].MemoryID)
	}
}
