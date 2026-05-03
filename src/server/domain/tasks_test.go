package domain

import (
	"encoding/json"
	"testing"
	"time"
)

func TestSharedTask(t *testing.T) {
	now := time.Now()
	agentID := "agent123"
	payload := json.RawMessage(`{"key":"value"}`)

	task := SharedTask{
		ID:        "uuid-1234",
		AgentID:   &agentID,
		Status:    "PENDING",
		Payload:   payload,
		CreatedAt: now,
	}

	if task.ID != "uuid-1234" {
		t.Errorf("Expected ID 'uuid-1234', got '%s'", task.ID)
	}

	if task.AgentID == nil || *task.AgentID != "agent123" {
		t.Errorf("Expected AgentID 'agent123'")
	}

	if task.Status != "PENDING" {
		t.Errorf("Expected Status 'PENDING', got '%s'", task.Status)
	}

	if string(task.Payload) != `{"key":"value"}` {
		t.Errorf("Expected Payload `{\"key\":\"value\"}`")
	}

	if task.CreatedAt != now {
		t.Errorf("Expected CreatedAt to be equal to initial time")
	}

    if !task.IsPending() {
        t.Errorf("Expected IsPending to be true")
    }

    task.Assign("agent456")
    if task.IsPending() {
        t.Errorf("Expected IsPending to be false after Assign")
    }
    if *task.AgentID != "agent456" {
        t.Errorf("Expected AgentID 'agent456', got '%s'", *task.AgentID)
    }

    task.Complete(json.RawMessage(`{"result":"done"}`))
    if task.Status != "COMPLETED" {
        t.Errorf("Expected Status 'COMPLETED', got '%s'", task.Status)
    }
    if string(task.Payload) != `{"result":"done"}` {
        t.Errorf("Expected Payload `{\"result\":\"done\"}`, got '%s'", string(task.Payload))
    }
}
