package analytics

import "testing"

func TestTracker(t *testing.T) {
	tracker := NewTracker()
	tracker.Track("page_view", "user1", map[string]interface{}{"page": "home"})

	events := tracker.GetEvents()
	if len(events) != 1 {
		t.Fatalf("expected 1 event, got %d", len(events))
	}
	if events[0].Name != "page_view" {
		t.Errorf("expected page_view, got %s", events[0].Name)
	}
}
