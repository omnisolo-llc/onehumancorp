package dashboard

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleMCPInvokeCoverage(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	_, err := authStore.CreateUser("adminuser", "admin@test.com", "adminpass123", []string{"admin"})
	if err != nil {
		t.Fatal("create user failed", err)
	}
	user, err := authStore.Authenticate("adminuser", "adminpass123")
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

func TestHandleRAGSync(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	sqlDB, _ := sql.Open("sqlite", "file::memory:?cache=shared")
	testDB := db.NewSqliteProvider(sqlDB)
	testDB.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	sipdb, _ := orchestration.NewSIPDBWithProvider(testDB)
	hub.SetSIPDB(sipdb)

	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	srv := &Server{org: org, hub: hub, tracker: tracker, authStore: authStore}

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/api/orchestration/sync/rag", nil)
		w := httptest.NewRecorder()
		srv.handleRAGSync(w, req)
		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("expected 405, got %d", w.Code)
		}
	})

	t.Run("invalid payload", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/orchestration/sync/rag", strings.NewReader(`{invalid}`))
		req.Header.Set("Content-Type", "application/json")
		w := httptest.NewRecorder()
		srv.handleRAGSync(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", w.Code)
		}
	})

	t.Run("successful sync with PII redaction and force-local", func(t *testing.T) {
		payload := `{"memory_id": "test-rag-1", "rag_context": "sensitive raw vector db text", "context": {"user_email": "alice@example.com", "nested": ["some text", "bob@example.com"]}}`
		req := httptest.NewRequest("POST", "/api/orchestration/sync/rag", strings.NewReader(payload))
		req.Header.Set("Content-Type", "application/json")
		req.Header.Set("X-OHC-Conflict-Resolution", "force-local")
		w := httptest.NewRecorder()
		srv.handleRAGSync(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		// Verify it was stored correctly and PII redacted, and rag_context removed
		var ctxStr string
		err := testDB.QueryRow(req.Context(), "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'test-rag-1'").Scan(&ctxStr)
		if err != nil {
			t.Fatalf("QueryRow failed: %v", err)
		}

		if strings.Contains(ctxStr, "sensitive raw vector db text") || strings.Contains(ctxStr, "rag_context") {
			t.Errorf("expected rag_context to be stripped, got %s", ctxStr)
		}
		if !strings.Contains(ctxStr, "[REDACTED_EMAIL]") {
			t.Errorf("expected context to contain [REDACTED_EMAIL], got %s", ctxStr)
		}
		if strings.Contains(ctxStr, "alice@example.com") || strings.Contains(ctxStr, "bob@example.com") {
			t.Errorf("expected context to NOT contain original emails, got %s", ctxStr)
		}
	})

	t.Run("conflict handling without force-local", func(t *testing.T) {
		// First insert to create a conflict state
		payload1 := `{"memory_id": "test-rag-conflict", "context": {"status": "original"}}`
		req1 := httptest.NewRequest("POST", "/api/orchestration/sync/rag", strings.NewReader(payload1))
		w1 := httptest.NewRecorder()
		srv.handleRAGSync(w1, req1)

		// Second insert without force-local
		payload2 := `{"memory_id": "test-rag-conflict", "context": {"status": "updated"}}`
		req2 := httptest.NewRequest("POST", "/api/orchestration/sync/rag", strings.NewReader(payload2))
		w2 := httptest.NewRecorder()
		srv.handleRAGSync(w2, req2)

		if w2.Code != http.StatusConflict {
			t.Errorf("expected 409 Conflict, got %d", w2.Code)
		}

		var ctxStr string
		_ = testDB.QueryRow(context.Background(), "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'test-rag-conflict'").Scan(&ctxStr)

		if strings.Contains(ctxStr, "updated") {
			t.Errorf("expected original data to be preserved, got %s", ctxStr)
		}
	})

	t.Run("conflict handling with force-local", func(t *testing.T) {
		// First insert to create a conflict state
		payload1 := `{"memory_id": "test-rag-force", "context": {"status": "original"}}`
		req1 := httptest.NewRequest("POST", "/api/orchestration/sync/rag", strings.NewReader(payload1))
		w1 := httptest.NewRecorder()
		srv.handleRAGSync(w1, req1)

		// Second insert with force-local
		payload2 := `{"memory_id": "test-rag-force", "context": {"status": "updated"}}`
		req2 := httptest.NewRequest("POST", "/api/orchestration/sync/rag", strings.NewReader(payload2))
		req2.Header.Set("X-OHC-Conflict-Resolution", "force-local")
		w2 := httptest.NewRecorder()
		srv.handleRAGSync(w2, req2)

		var ctxStr string
		_ = testDB.QueryRow(context.Background(), "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'test-rag-force'").Scan(&ctxStr)

		if !strings.Contains(ctxStr, "updated") {
			t.Errorf("expected data to be updated with force-local, got %s", ctxStr)
		}
	})
}
