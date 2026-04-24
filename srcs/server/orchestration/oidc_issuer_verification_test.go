package orchestration

import (
	"testing"
)

func TestHub_OidcIssuerVerification(t *testing.T) {
	h := NewHub()

	eventID := "test-event-1"
	agentID := "agent-1"
	payload := []byte(`{}`)

	err := h.OidcIssuerVerification(eventID, agentID, payload)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	// Test invalid payload to trigger DisallowUnknownFields error
	invalidPayload := []byte(`{"unknown": "field"}`)
	err = h.OidcIssuerVerification("test-event-2", agentID, invalidPayload)
	if err == nil {
		t.Fatalf("expected error for invalid payload")
	}

	// Test concurrent/duplicate eventID
	err1 := h.OidcIssuerVerification("test-event-3", agentID, payload)
	if err1 != nil {
		t.Fatalf("unexpected error: %v", err1)
	}
}

// To get 100% coverage on duplicate event processing we need a concurrent test.
func TestHub_OidcIssuerVerification_Duplicate(t *testing.T) {
	h := NewHub()

	// We can manually add an event to h.tokenTrackers using a helper or by starting one long process.
	// We'll simulate by locking the event tracker or calling concurrently.
	h.mu.Lock()
	if h.tokenTrackers == nil {
		h.tokenTrackers = make(map[string]struct{})
	}
	h.tokenTrackers["duplicate-event"] = struct{}{}
	h.mu.Unlock()

	err := h.OidcIssuerVerification("duplicate-event", "agent", []byte(`{}`))
	if err == nil || err.Error() != "event already being processed" {
		t.Fatalf("expected 'event already being processed' error, got: %v", err)
	}
}
