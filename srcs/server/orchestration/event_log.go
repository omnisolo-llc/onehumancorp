package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

const maxInMemoryHubEvents = 200

func sanitizeHubEvent(raw interface{}) (HubEvent, error) {
	payload, err := json.Marshal(raw)
	if err != nil {
		return HubEvent{}, fmt.Errorf("marshal hub event: %w", err)
	}

	var decoded interface{}
	if err := json.Unmarshal(payload, &decoded); err == nil {
		decoded = telemetry.RedactInterfacePII(decoded)
		payload, err = json.Marshal(decoded)
		if err != nil {
			return HubEvent{}, fmt.Errorf("marshal redacted hub event: %w", err)
		}
		return HubEvent{
			Type:       detectHubEventType(decoded, raw),
			Payload:    payload,
			OccurredAt: time.Now().UTC(),
		}, nil
	}

	return HubEvent{
		Type:       detectHubEventType(nil, raw),
		Payload:    payload,
		OccurredAt: time.Now().UTC(),
	}, nil
}

func detectHubEventType(decoded interface{}, raw interface{}) string {
	if eventMap, ok := decoded.(map[string]interface{}); ok {
		if eventType, ok := eventMap["type"].(string); ok && eventType != "" {
			return eventType
		}
	}
	return fmt.Sprintf("%T", raw)
}

func (h *Hub) appendRecentEvent(event HubEvent) {
	h.mu.Lock()
	defer h.mu.Unlock()

	h.recentEvents = append(h.recentEvents, event)
	if len(h.recentEvents) > maxInMemoryHubEvents {
		h.recentEvents = append([]HubEvent(nil), h.recentEvents[len(h.recentEvents)-maxInMemoryHubEvents:]...)
	}
}

func (h *Hub) RecentEvents(limit int) []HubEvent {
	if limit <= 0 || limit > maxInMemoryHubEvents {
		limit = maxInMemoryHubEvents
	}

	h.mu.RLock()
	defer h.mu.RUnlock()

	if len(h.recentEvents) == 0 {
		return nil
	}

	if limit > len(h.recentEvents) {
		limit = len(h.recentEvents)
	}

	result := make([]HubEvent, 0, limit)
	for index := len(h.recentEvents) - 1; index >= 0 && len(result) < limit; index-- {
		result = append(result, h.recentEvents[index])
	}
	return result
}

func (h *Hub) eventLogWorker(ctx context.Context) {
	for {
		select {
		case <-ctx.Done():
			return
		case rawEvent, ok := <-h.eventLogChan:
			if !ok {
				return
			}

			event, err := sanitizeHubEvent(rawEvent)
			if err != nil {
				slog.Error("failed to sanitize hub event", "error", err)
				continue
			}

			if h.repo != nil {
				if err := h.repo.AppendEvent(ctx, event); err != nil {
					slog.Error("failed to persist hub event", "error", err)
				}
			}

			h.appendRecentEvent(event)
		}
	}
}
