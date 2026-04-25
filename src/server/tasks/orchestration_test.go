package tasks

import (
    "context"
    "encoding/json"
    "net/http"
    "net/http/httptest"
    "testing"

    "github.com/onehumancorp/mono/src/server/db"
)

func TestClaimTask(t *testing.T) {
    ctx := context.Background()

    // Start mock mesh server
    var receivedPayload map[string]interface{}
    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        var req map[string]interface{}
        if err := json.NewDecoder(r.Body).Decode(&req); err == nil {
            if req["event_type"] == "TASK_TRANSITION" {
                receivedPayload = req
            }
        }
        w.WriteHeader(http.StatusOK)
    }))
    defer server.Close()

    provider := db.NewTestProvider(t)

    _, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS mission_queue (mission_id TEXT PRIMARY KEY, status TEXT, assigned_agent TEXT)`)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    _, err = provider.Exec(ctx, `INSERT INTO mission_queue (mission_id, status) VALUES ('123', 'QUEUED')`)
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    orchestrator := NewOrchestrator(provider, server.URL)
    missionID, err := orchestrator.ClaimTask(ctx, "agent1")
    if err != nil {
        t.Fatalf("failed to claim task: %v", err)
    }

    if missionID != "123" {
        t.Errorf("expected missionID 123, got %s", missionID)
    }

    if receivedPayload["agent_id"] != "agent1" {
        t.Errorf("expected payload agent_id agent1, got %v", receivedPayload["agent_id"])
    }

    err = orchestrator.CompleteTask(ctx, missionID, "agent1")
    if err != nil {
        t.Fatalf("failed to complete task: %v", err)
    }

    if receivedPayload["data"].(map[string]interface{})["new_state"] != "DONE" {
        t.Errorf("expected payload new_state DONE, got %v", receivedPayload["data"])
    }

    // Fails because it's already DONE
    err = orchestrator.FailTask(ctx, missionID, "agent1", "some reason")
    if err == nil {
        t.Fatalf("expected to fail to fail a DONE task")
    }
}

func TestFailTask(t *testing.T) {
    ctx := context.Background()

    var receivedPayload map[string]interface{}
    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        var req map[string]interface{}
        if err := json.NewDecoder(r.Body).Decode(&req); err == nil {
            if req["event_type"] == "TASK_TRANSITION" {
                receivedPayload = req
            }
        }
        w.WriteHeader(http.StatusOK)
    }))
    defer server.Close()

    provider := db.NewTestProvider(t)
    _, _ = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS mission_queue (mission_id TEXT PRIMARY KEY, status TEXT, assigned_agent TEXT)`)
    _, _ = provider.Exec(ctx, `INSERT INTO mission_queue (mission_id, status) VALUES ('124', 'QUEUED')`)

    orchestrator := NewOrchestrator(provider, server.URL)
    missionID, _ := orchestrator.ClaimTask(ctx, "agent2")

    err := orchestrator.FailTask(ctx, missionID, "agent2", "test fail")
    if err != nil {
        t.Fatalf("failed to fail task: %v", err)
    }

    if receivedPayload["data"].(map[string]interface{})["new_state"] != "BLOCKED" {
        t.Errorf("expected payload new_state BLOCKED, got %v", receivedPayload["data"])
    }
}
