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

	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHybridMCPBridge_Escalation(t *testing.T) {
	// Set the environment to standalone mode for local tool simulation
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	// Create mock SIPDB
	db, err := orchestration.NewSIPDB("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to init sipdb: %v", err)
	}
	hub.SetSIPDB(db)

	srv := NewServer(org, hub, nil, nil)
	srv.dynamicMCPTools = append(srv.dynamicMCPTools, MCPTool{
		ID: "test-escalation-mcp", Name: "Test", Description: "Test", Category: "test", Status: "available", HybridEscalation: true,
	})

	reqPayload := mcpInvokeRequest{
		ToolID: "test-escalation-mcp",
		Action: "escalate",
		Params: json.RawMessage(`{"escalate": true, "pii_data": "secret@example.com", "content": "hello world"}`),
	}

	body, _ := json.Marshal(reqPayload)
	req := httptest.NewRequest("POST", "/api/mcp/tools/invoke", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	srv.handleMCPInvoke(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d. Body: %s", w.Code, w.Body.String())
	}

	var resp map[string]any
	if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
		t.Fatalf("Failed to unmarshal response: %v", err)
	}

	// 3. Verify HybridEscalation is true
	if esc, ok := resp["HybridEscalation"].(bool); !ok || !esc {
		t.Errorf("Expected HybridEscalation to be true in response, got %v", resp["HybridEscalation"])
	}

	// 4. Verify Database Sync
	var count int
	err = db.DB().QueryRow(context.Background(), "SELECT COUNT(*) FROM agent_missions WHERE status = 'CLOUD_ESCALATION'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query agent_missions count: %v", err)
	}

	if count == 0 {
		t.Fatalf("Expected 1 mission with status 'CLOUD_ESCALATION'")
	}

	var payloadStr string
	err = db.DB().QueryRow(context.Background(), "SELECT payload FROM agent_missions WHERE status = 'CLOUD_ESCALATION' LIMIT 1").Scan(&payloadStr)
	if err != nil {
		t.Fatalf("Failed to query agent_missions payload: %v", err)
	}

	var payload map[string]any
	if err := json.Unmarshal([]byte(payloadStr), &payload); err != nil {
		t.Fatalf("Failed to parse mission payload: %v", err)
	}

	params, ok := payload["params"].(map[string]any)
	if !ok {
		t.Fatalf("Expected params in payload")
	}

	// The payload should be redacted
	piiData, ok := params["pii_data"].(string)
	if ok && piiData == "secret@example.com" {
		t.Errorf("PII data was not redacted! Found: %s", piiData)
	}
	if piiData != "[REDACTED_EMAIL]" {
		t.Errorf("Expected redacted email, got %v", piiData)
	}
}
