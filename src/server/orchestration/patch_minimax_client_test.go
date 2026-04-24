package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestMinimaxClientReasonFailureWithContextTimeout(t *testing.T) {
	ResetCircuitBreakerForTest()
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("internal error"))
	}))
	defer ts.Close()

	originalURL := MinimaxAPIURL
	MinimaxAPIURL = ts.URL
	defer func() { MinimaxAPIURL = originalURL }()

	client := NewMinimaxClient("valid-key")

	// Create a context with a short timeout to prevent the test from hanging indefinitely
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	_, err := client.Reason(ctx, "test")
	if err == nil {
		t.Fatalf("expected error on 500 response")
	}
}
