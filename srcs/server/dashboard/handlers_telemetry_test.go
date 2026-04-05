package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleTelemetrySync(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	store := auth.NewStore()

	// Initialize the Server with the NewServer method to make sure all endpoints are registered correctly
	handler := NewServer(org, hub, tracker, store)

	metricsPayload := []map[string]interface{}{
		{
			"metric_type": "token_usage",
			"agent_id":    "test-agent",
			"role":        "test-role",
			"model":       "gpt-4",
			"type":        "prompt",
			"count":       150,
		},
		{
			"metric_type": "agent_api_call",
			"agent_id":    "test-agent",
			"role":        "test-role",
			"api":         "github",
		},
	}

	body, _ := json.Marshal(metricsPayload)
	req := httptest.NewRequest("POST", "/api/telemetry/sync", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")

	// Need to bypass auth for this test since it's an internal test,
	// or mock the auth token. Dashboard routes under /api usually require auth.
	// But /api/telemetry/sync doesn't enforce Role yet, though it passes through auth.Middleware.
	// We'll add an auth context.
	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{
		OrganizationID: "test-org",
		UserID:         "system",
		Roles:          []string{"admin"},
	})
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	var resp map[string]string
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	if resp["status"] != "success" {
		t.Errorf("expected success status, got %v", resp["status"])
	}
}
