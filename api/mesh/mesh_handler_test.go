package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockMeshService struct {
	lastIntent string
	err        error
}

func (m *mockMeshService) BroadcastIntent(ctx context.Context, intent string) error {
	m.lastIntent = intent
	return m.err
}

func (m *mockMeshService) Subscribe(ctx context.Context) (<-chan string, error) {
	return nil, nil
}

// withClaims is a helper for testing
func withClaims(ctx context.Context, claims *auth.Claims) context.Context {
	return context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)
}

func TestMeshHandler_handleBroadcast(t *testing.T) {
	service := &mockMeshService{}
	handler := NewMeshHandler(service)

	t.Run("Unauthorized", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodPost, "/broadcast", nil)
		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusUnauthorized {
			t.Errorf("expected 401, got %d", rr.Code)
		}
	})

	t.Run("ValidRequest", func(t *testing.T) {
		payload := MeshBroadcastRequest{
			AgentID: "agent-1",
			Action:  "test-action",
			Status:  "test-status",
		}
		body, _ := json.Marshal(payload)
		req, _ := http.NewRequest(http.MethodPost, "/broadcast", bytes.NewReader(body))

		// Add claims to context
		ctx := withClaims(req.Context(), &auth.Claims{OrganizationID: "org-1"})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", rr.Code)
		}

		var received MeshBroadcastRequest
		if err := json.Unmarshal([]byte(service.lastIntent), &received); err != nil {
			t.Fatal(err)
		}

		if received.AgentID != payload.AgentID {
			t.Errorf("expected agent-1, got %s", received.AgentID)
		}
	})

	t.Run("MissingFields", func(t *testing.T) {
		payload := MeshBroadcastRequest{
			AgentID: "agent-1",
			// Missing Action and Status
		}
		body, _ := json.Marshal(payload)
		req, _ := http.NewRequest(http.MethodPost, "/broadcast", bytes.NewReader(body))
		ctx := withClaims(req.Context(), &auth.Claims{OrganizationID: "org-1"})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("expected 400, got %d", rr.Code)
		}
	})
}
