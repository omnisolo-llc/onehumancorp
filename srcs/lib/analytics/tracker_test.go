package analytics

import (
	"context"
	"testing"
)

func TestInMemoryTracker(t *testing.T) {
	tracker := NewInMemoryTracker()

	event := Event{
		Name: "test_event",
		Properties: map[string]interface{}{
			"key": "value",
		},
	}

	err := tracker.Track(context.Background(), event)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	events := tracker.Events()
	if len(events) != 1 {
		t.Fatalf("expected 1 event, got %d", len(events))
	}

	if events[0].Name != event.Name {
		t.Errorf("expected event name %q, got %q", event.Name, events[0].Name)
	}
}
