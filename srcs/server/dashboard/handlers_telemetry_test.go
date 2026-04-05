package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleTelemetrySync(t *testing.T) {
	org := domain.Organization{ID: "test-org"}
	hub := orchestration.NewHub()
	server := &Server{
		org:       org,
		hub:       hub,
		authStore: auth.NewStore(),
	}

	payload := []map[string]interface{}{
		{
			"metric_type": "token_usage",
			"payload":     `{"agent_id":"agent1","role":"role1","model":"model1","type":"input","count":100}`,
		},
	}
	body, _ := json.Marshal(payload)

	req, _ := http.NewRequest(http.MethodPost, "/api/telemetry/sync", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	server.handleTelemetrySync(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected status OK, got %d", rr.Code)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}
	if resp["synced"] != float64(1) {
		t.Errorf("Expected synced count to be 1, got %v", resp["synced"])
	}
}
