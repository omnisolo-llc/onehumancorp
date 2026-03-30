package minimax

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestCircuitBreaker(t *testing.T) {
	fail := true
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if fail {
			w.WriteHeader(http.StatusInternalServerError)
			w.Write([]byte("internal error"))
			return
		}
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"choices":[{"message":{"content":"success"}}]}`))
	}))
	defer ts.Close()

	originalURL := minimaxAPIURL
	minimaxAPIURL = ts.URL
	defer func() { minimaxAPIURL = originalURL }()

	client := NewClient("valid-key")
	cb := NewCircuitBreaker(client, 2, 100*time.Millisecond)

	// Failure 1
	_, err := cb.Reason(context.Background(), "test")
	if err == nil {
		t.Fatalf("expected error")
	}

	// Failure 2 - circuit opens
	_, err = cb.Reason(context.Background(), "test")
	if err == nil {
		t.Fatalf("expected error")
	}

	// Circuit is open, should fail immediately
	_, err = cb.Reason(context.Background(), "test")
	if err == nil || err.Error() != "circuit breaker is open" {
		t.Fatalf("expected circuit breaker is open error, got %v", err)
	}

	// Wait for reset timeout
	time.Sleep(150 * time.Millisecond)

	// Half-open state, fail = true, should fail and open again
	_, err = cb.Reason(context.Background(), "test")
	if err == nil {
		t.Fatalf("expected error")
	}

	// Wait for reset timeout again
	time.Sleep(150 * time.Millisecond)

	// Half-open state, fail = false, should succeed and close
	fail = false
	res, err := cb.Reason(context.Background(), "test")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "success" {
		t.Fatalf("expected success, got %v", res)
	}

	// Closed state, should succeed
	res, err = cb.Reason(context.Background(), "test")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test Reset
	cb.Reset()
	cb.mu.Lock()
	if cb.state != StateClosed || cb.failures != 0 {
		cb.mu.Unlock()
		t.Fatalf("Reset did not reset properly")
	}
	cb.mu.Unlock()
}
