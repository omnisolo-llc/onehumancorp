package api

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/integrations/mcp"
)

func TestMCPWebhookHandler(t *testing.T) {
	os.Setenv("MCP_WEBHOOK_SECRET", "") // Use test mode

	pool := db.NewTestProvider(t)
	defer pool.Close()

	dbWrapper := &db.DB{Provider: pool}
	if err := dbWrapper.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	tracker := mcp.NewAsyncTaskTracker(pool)
	handler := NewMCPWebhookHandler(tracker)

	// Create test task
	task := mcp.AsyncTask{
		ID:       "task-1",
		TenantID: "tenant-1",
		AgentID:  "agent-1",
		Status:   "pending",
	}
	err := tracker.CreateTask(context.Background(), task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	// Make webhook request
	payload := WebhookPayload{
		TaskID:  "task-1",
		Status:  "completed",
		Payload: `{"result": "success"}`,
	}
	body, _ := json.Marshal(payload)
	req := httptest.NewRequest(http.MethodPost, "/webhook", bytes.NewReader(body))
	req.Header.Set("X-MCP-Signature", "test-sig")
	w := httptest.NewRecorder()

	handler.ServeHTTP(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Errorf("expected 200 OK, got %v", w.Result().StatusCode)
	}

	// Verify task updated
	updatedTask, err := tracker.GetTask(context.Background(), "task-1")
	if err != nil {
		t.Fatalf("failed to get updated task: %v", err)
	}
	if updatedTask.Status != "completed" {
		t.Errorf("expected status 'completed', got %v", updatedTask.Status)
	}
	if updatedTask.Payload != `{"result": "success"}` {
		t.Errorf("expected payload `{\"result\": \"success\"}`, got %v", updatedTask.Payload)
	}
}
