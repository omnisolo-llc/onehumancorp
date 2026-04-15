package orchestration

import (
	"testing"
	"time"
)

func TestEventLogWorker_CoverageGaps(t *testing.T) {
	// Create a new Hub instance
	hub := NewHub()
	defer hub.Close()

	// 1. Test LogEvent channel full capacity dropping logic
	// We need to fill the channel *before* the worker starts draining it,
	// or create a scenario where the worker is too slow.
	// We can simply fill the channel completely right away.
	for i := 0; i < cap(hub.eventLogChan); i++ {
		hub.eventLogChan <- Message{ID: "fill"}
	}

	// Now LogEvent should drop the message because the channel is full.
	// (This executes the `default:` case in the select inside LogEvent)
	hub.LogEvent(Message{ID: "dropped"})

	// 2. Verify the background worker keeps recent events in memory.
	hub.LogEvent(Message{ID: "m1", Content: "test content"})
	time.Sleep(50 * time.Millisecond)

	events := hub.RecentEvents(5)
	if len(events) == 0 {
		t.Fatalf("expected recent events, got none")
	}
}
