package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestMeshAPI_BroadcastHandler(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	mesh := NewLocalTeammateMesh(prov)
	handler := NewMeshAPIHandler(prov, mesh)

	task := Task{
		AgentID: "agent1",
		Action:  "DO_WORK",
		Status:  "PENDING",
		TaskID:  "task1",
	}

	body, _ := json.Marshal(task)
	req := httptest.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	handler.BroadcastHandler(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Errorf("Expected status 200, got %d", w.Result().StatusCode)
	}
}

func TestMeshAPI_DirectMessageHandler(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	// Create table agent_inbox for sqlite test provider
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS agent_inbox (
			seq INTEGER PRIMARY KEY AUTOINCREMENT,
			message_id TEXT NOT NULL,
			agent_id TEXT NOT NULL,
			from_agent TEXT,
			to_agent TEXT NOT NULL,
			type TEXT,
			content TEXT NOT NULL,
			meeting_id TEXT,
			occurred_at DATETIME,
			organization_id TEXT
		);
		CREATE TABLE IF NOT EXISTS agents (
			id TEXT PRIMARY KEY,
			name TEXT,
			role TEXT,
			organization_id TEXT,
			status TEXT,
			provider_type TEXT,
			region TEXT
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	// insert mock agent
	_, err = prov.Exec(context.Background(), "INSERT INTO agents (id, name, organization_id) VALUES ('agent2', 'Agent Two', 'org1')")
	if err != nil {
		t.Fatalf("failed to insert agent: %v", err)
	}

	mesh := NewLocalTeammateMesh(prov)
	handler := NewMeshAPIHandler(prov, mesh)

	msg := Message{
		ToAgent: "agent2",
		Content: "Hello",
	}

	body, _ := json.Marshal(msg)
	req := httptest.NewRequest("POST", "/api/mesh/direct", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-Org-ID", "org1")
	w := httptest.NewRecorder()

	handler.DirectMessageHandler(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Errorf("Expected status 200, got %d", w.Result().StatusCode)
	}
}
