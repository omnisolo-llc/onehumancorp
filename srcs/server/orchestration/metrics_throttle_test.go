package orchestration

import (
	"os"
	"testing"
)

func TestStandaloneSQLiteConcurrencyThrottling(t *testing.T) {
	// Enable standalone
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	hub := NewHub()
	hub.RegisterAgent(Agent{ID: "agent-a", Role: "role-a", OrganizationID: "org-a"})
	hub.RegisterAgent(Agent{ID: "agent-b", Role: "role-b", OrganizationID: "org-a"})

	// It shouldn't block indefinitely
	err := hub.DelegateTask("agent-a", "agent-b", Message{ID: "msg1", Content: "test"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
