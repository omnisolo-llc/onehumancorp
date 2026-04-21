package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleTelemetrySyncV1(t *testing.T) {
	org := domain.Organization{ID: "test-org"}
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	store := auth.NewStore()

	server := NewServer(org, hub, tracker, store)

	// Create a request with multiple telemetry items
	batch := []map[string]interface{}{
		{
			"metric_name": "token_usage",
			"payload": map[string]interface{}{
				"agent_id": "agent-1",
				"role":     "SOFTWARE_ENGINEER",
				"model":    "gpt-4o",
				"type":     "prompt",
				"count":    100,
			},
		},
		{
			"metric_name": "agent_api_call",
			"payload": map[string]interface{}{
				"agent_id": "agent-1",
				"role":     "SOFTWARE_ENGINEER",
				"api":      "github",
			},
		},
	}

	body, _ := json.Marshal(batch)
	req := httptest.NewRequest(http.MethodPost, "/api/v1/telemetry/sync", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")

	// We need to bypass auth or provide a system role claim.
	// Since NewServer uses auth.Middleware, we should provide a token or inject claims.
	// auth.RequireRole checks claims from context.

	claims := &auth.Claims{
		UserID:         "sys-admin",
		Role:           "system",
		OrganizationID: "test-org",
	}
	ctx := auth.ContextWithClaims(req.Context(), claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	server.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("expected status OK, got %d: %s", rr.Code, rr.Body.String())
	}

	var resp map[string]string
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp["status"] != "ok" {
		t.Errorf("expected status ok, got %s", resp["status"])
	}
}
