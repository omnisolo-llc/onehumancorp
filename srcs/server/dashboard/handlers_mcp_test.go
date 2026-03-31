package dashboard

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleMCPInvokeCoverage(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
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
		req := httptest.NewRequest("POST", "/api/mcp/invoke", strings.NewReader(`{"spiffeId": "spiffe://onehumancorp.io/agent/1", "toolId": "dummy", "params": {"a": "` + largeStr + `"}}`))
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

func TestHandleMCPTools(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
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
