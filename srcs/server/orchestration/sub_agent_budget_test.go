package orchestration

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSubAgentSpawner_TokenBudget(t *testing.T) {
	mesh := &mockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(mesh, false, 0)

	task := &SharedTask{
		ID:             "test-budget",
		OrganizationID: "org-budget-fail", // Preconfigured to 0
	}

	spawner.runSubAgent(context.Background(), task)

	foundPaused := false
	foundFailed := false
	for _, msg := range mesh.published {
		var payload map[string]interface{}
		_ = json.Unmarshal([]byte(msg), &payload)
		if payload["event"] == "SUB_AGENT_PAUSED" && payload["task_id"] == "test-budget" {
			foundPaused = true
		}
		if payload["event"] == "SUB_AGENT_FAILED" && payload["task_id"] == "test-budget" {
			foundFailed = true
		}
	}
	assert.True(t, foundPaused)
	assert.False(t, foundFailed) // We expect only PAUSED now based on PR feedback and business logic
}
