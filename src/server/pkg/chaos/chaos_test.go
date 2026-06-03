package chaos

import (
	"testing"
)

func TestCircuitBreaker(t *testing.T) {
	cb := NewCircuitBreaker(3)
	if cb.IsOpen() {
		t.Errorf("Circuit breaker should be closed initially")
	}
	cb.RecordFailure()
	cb.RecordFailure()
	if cb.IsOpen() {
		t.Errorf("Circuit breaker should be closed after 2 failures")
	}
	cb.RecordFailure()
	if !cb.IsOpen() {
		t.Errorf("Circuit breaker should be open after 3 failures")
	}
	cb.Reset()
	if cb.IsOpen() {
		t.Errorf("Circuit breaker should be closed after reset")
	}
}
