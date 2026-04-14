package orchestration

import (
    "encoding/json"
)

type KairosTaskTransitionEvent struct {
    TaskID        string `json:"task_id"`
    PreviousState string `json:"previous_state"`
    NewState      string `json:"new_state"`
}

type KairosMeshBroadcastEvent struct {
    Channel   string          `json:"channel"`
    EventType string          `json:"event_type"`
    Data      json.RawMessage `json:"data"`
}
