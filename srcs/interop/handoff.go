package interop

import (
	"encoding/json"
	"fmt"
)

// ParseHandoff parses a JSON string into a MultiAgentHandoff protobuf message.
func ParseHandoff(payload string) (*MultiAgentHandoff, error) {
	var handoff MultiAgentHandoff
	// Using basic JSON unmarshalling; for proto compliance one might use protojson,
	// but standard encoding/json is often sufficient for basic struct mappings.
	if err := json.Unmarshal([]byte(payload), &handoff); err != nil {
		return nil, fmt.Errorf("failed to parse handoff: %w", err)
	}
	return &handoff, nil
}

// SerializeHandoff serializes a MultiAgentHandoff protobuf message into a JSON string.
func SerializeHandoff(handoff *MultiAgentHandoff) (string, error) {
	b, err := json.Marshal(handoff)
	if err != nil {
		return "", fmt.Errorf("failed to serialize handoff: %w", err)
	}
	return string(b), nil
}
