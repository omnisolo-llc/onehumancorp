package dashboard

import (
	"bytes"
	"crypto/tls"
	"crypto/x509"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func createMockTLSRequest(method, url string, body []byte, hasCert bool) *http.Request {
	req := httptest.NewRequest(method, url, bytes.NewBuffer(body))
	if hasCert {
		req.TLS = &tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{{}},
		}
	} else {
		req.TLS = nil
	}
	return req
}

func TestHandleMeshBroadcast(t *testing.T) {
	org := domain.Organization{ID: "org-mesh"}
	hub := orchestration.NewHub("test-mesh-db", "memory://")
	srv := &Server{
		org: org,
		hub: hub,
	}

	cn, _ := orchestration.NewCentrifugeNode()
	hub.SetCentrifugeNode(cn)

	t.Run("RequiresmTLS", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodPost, "/api/mesh/broadcast", nil, false)
		w := httptest.NewRecorder()

		srv.handleMeshBroadcast(w, req)

		if w.Code != http.StatusForbidden {
			t.Errorf("expected status 403 Forbidden, got %v", w.Code)
		}
	})

	t.Run("ValidBroadcastMeshTasks", func(t *testing.T) {
		payload := map[string]interface{}{
			"channel":  "mesh:tasks",
			"agent_id": "test-agent",
			"action":   "test-action",
			"status":   "test-status",
		}
		body, _ := json.Marshal(payload)

		req := createMockTLSRequest(http.MethodPost, "/api/mesh/broadcast", body, true)

		// Setup context to bypass auth middleware but satisfy auth role check
		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{
			Role:           "system",
			OrganizationID: "org-mesh",
		})
		req = req.WithContext(ctx)

		w := httptest.NewRecorder()

		srv.handleMeshBroadcast(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected status 200 OK, got %v", w.Code)
		}
	})

	t.Run("ValidBroadcastMeshCoordination", func(t *testing.T) {
		payload := map[string]interface{}{
			"channel":  "mesh:coordination",
			"agent_id": "test-agent",
			"action":   "test-action",
			"status":   "test-status",
		}
		body, _ := json.Marshal(payload)

		req := createMockTLSRequest(http.MethodPost, "/api/mesh/broadcast", body, true)

		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{
			Role:           "system",
			OrganizationID: "org-mesh",
		})
		req = req.WithContext(ctx)

		w := httptest.NewRecorder()

		srv.handleMeshBroadcast(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected status 200 OK, got %v", w.Code)
		}
	})
}

func TestHandleMeshDirect(t *testing.T) {
	org := domain.Organization{ID: "org-mesh"}
	hub := orchestration.NewHub("test-mesh-db", "memory://")
	srv := &Server{
		org: org,
		hub: hub,
	}

	t.Run("RequiresmTLS", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodPost, "/api/mesh/direct", nil, false)
		w := httptest.NewRecorder()

		srv.handleMeshDirect(w, req)

		if w.Code != http.StatusForbidden {
			t.Errorf("expected status 403 Forbidden, got %v", w.Code)
		}
	})

	t.Run("ValidDirectMessage", func(t *testing.T) {
		payload := map[string]interface{}{
			"toAgent": "target-agent",
			"payload": `{"test":"data"}`,
		}
		body, _ := json.Marshal(payload)

		req := createMockTLSRequest(http.MethodPost, "/api/mesh/direct", body, true)
		w := httptest.NewRecorder()

		srv.handleMeshDirect(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected status 200 OK, got %v", w.Code)
		}
	})
}

func TestHandleMeshMailbox(t *testing.T) {
	org := domain.Organization{ID: "org-mesh"}
	hub := orchestration.NewHub("test-mesh-db", "memory://")
	srv := &Server{
		org: org,
		hub: hub,
	}

	t.Run("RequiresmTLS", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodGet, "/api/mesh/mailbox?agent_id=test", nil, false)
		w := httptest.NewRecorder()

		srv.handleMeshMailbox(w, req)

		if w.Code != http.StatusForbidden {
			t.Errorf("expected status 403 Forbidden, got %v", w.Code)
		}
	})

	t.Run("ValidMailboxRequest", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodGet, "/api/mesh/mailbox?agent_id=test-agent", nil, true)
		w := httptest.NewRecorder()

		srv.handleMeshMailbox(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected status 200 OK, got %v", w.Code)
		}

		var resp struct {
			Messages []orchestration.Message `json:"messages"`
		}
		if err := json.NewDecoder(w.Body).Decode(&resp); err != nil {
			t.Fatalf("failed to decode response: %v", err)
		}
		if len(resp.Messages) != 0 {
			t.Errorf("expected empty messages, got %d", len(resp.Messages))
		}
	})
}
