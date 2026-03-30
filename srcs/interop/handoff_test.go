package interop

import (
	"testing"
)

func TestParseSerializeHandoff(t *testing.T) {
	handoff := &MultiAgentHandoff{
		MissionId:   "mission-123",
		Role:        "SOFTWARE_ENGINEER",
		TaskPayload: "Implement feature",
		Context: &HandoffContext{
			SourceAgent: "agent-a",
			TargetAgent: "agent-b",
			Priority:    "HIGH",
			Metadata: map[string]string{
				"project": "mono",
			},
		},
	}

	serialized, err := SerializeHandoff(handoff)
	if err != nil {
		t.Fatalf("Failed to serialize: %v", err)
	}

	parsed, err := ParseHandoff(serialized)
	if err != nil {
		t.Fatalf("Failed to parse: %v", err)
	}

	if parsed.MissionId != handoff.MissionId {
		t.Errorf("Expected MissionId %q, got %q", handoff.MissionId, parsed.MissionId)
	}
	if parsed.Role != handoff.Role {
		t.Errorf("Expected Role %q, got %q", handoff.Role, parsed.Role)
	}
	if parsed.TaskPayload != handoff.TaskPayload {
		t.Errorf("Expected TaskPayload %q, got %q", handoff.TaskPayload, parsed.TaskPayload)
	}
	if parsed.Context.SourceAgent != handoff.Context.SourceAgent {
		t.Errorf("Expected SourceAgent %q, got %q", handoff.Context.SourceAgent, parsed.Context.SourceAgent)
	}
}
