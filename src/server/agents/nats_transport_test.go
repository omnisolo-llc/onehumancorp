package agents

import (
	"testing"
)

func TestNatsTransport_ConnectFailure(t *testing.T) {
	// Attempt to connect to a likely non-existent NATS server
	_, err := NewNatsTransport("nats://127.0.0.1:42222", "pub", "sub")
	if err == nil {
		t.Fatalf("Expected failure to connect to NATS, got nil")
	}
}
