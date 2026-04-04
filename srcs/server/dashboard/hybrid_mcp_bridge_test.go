package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHybridMCPBridgeSync(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	hub, err := orchestration.NewCentrifugeHub(nil, nil)
	if err != nil {
		t.Fatalf("failed to create hub: %v", err)
	}

	server := &Server{
		hub:             hub,
		rateLimitStates: make(map[string]*RateLimitState),
		dynamicMCPTools: []orchestration.Tool{
			{
				ID: "test-tool",
				Name: "Test Tool",
			},
		},
	}

	reqPayload := mcpInvokeRequest{
		ToolID:   "test-tool",
		Action:   "test-action",
		Params:   json.RawMessage(`{"secret": "my-secret-key", "email": "test@example.com"}`),
		AgentID:  "agent-1",
		HybridEscalation: true,
	}

	body, _ := json.Marshal(reqPayload)
	req, _ := http.NewRequest("POST", "/api/v1/mcp/invoke", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	server.handleMCPInvoke(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected status OK, got %d. Body: %s", rr.Code, rr.Body.String())
	}

	time.Sleep(100 * time.Millisecond)

	// Verify that the payload was redacted and saved to agent_missions
	db := hub.SIPDB().GetDB()
	var payloadStr string
	var status string
	err = db.QueryRow(context.Background(), "SELECT payload, status FROM agent_missions WHERE status = 'CLOUD_ESCALATION'").Scan(&payloadStr, &status)
	if err != nil {
		t.Fatalf("failed to query agent_missions: %v", err)
	}

	if status != "CLOUD_ESCALATION" {
		t.Errorf("expected status CLOUD_ESCALATION, got %s", status)
	}

	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(payloadStr), &payloadMap); err != nil {
		t.Fatalf("failed to unmarshal payload: %v", err)
	}

	params, ok := payloadMap["params"].(map[string]interface{})
	if !ok {
		t.Fatalf("expected params to be a map, got %v", payloadMap["params"])
	}

	if params["email"] != "[PRIVATE:EMAIL]" {
		t.Errorf("expected email to be redacted, got %s", params["email"])
	}
}
