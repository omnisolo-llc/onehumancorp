package orchestration

import (
	"encoding/json"
	"fmt"
	"onehumancorp/srcs/server/telemetry"
)

type HubEvent struct {
	Event string `json:"event"`
}

func sanitizeHubEvent(raw interface{}) (HubEvent, error) {
	redactedRaw := telemetry.RedactInterfacePII(raw.(map[string]interface{}))
	payload, err := json.Marshal(redactedRaw)
	if err != nil {
		return HubEvent{}, fmt.Errorf("marshal hub event: %w", err)
	}

	var event HubEvent
	if err := json.Unmarshal(payload, &event); err != nil {
		return HubEvent{}, err
	}

	return event, nil
}
