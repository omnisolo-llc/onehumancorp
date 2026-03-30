package minimax

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestMinimaxClient_Reason(t *testing.T) {
	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		response := map[string]interface{}{
			"choices": []map[string]interface{}{
				{
					"message": map[string]interface{}{
						"content": "test response",
					},
				},
			},
		}
		json.NewEncoder(w).Encode(response)
	}))
	defer mockServer.Close()

	originalURL := minimaxAPIURL
	defer func() { SetMinimaxAPIURL(originalURL) }()
	SetMinimaxAPIURL(mockServer.URL)

	client := NewMinimaxClient("test-key")
	content, err := client.Reason(context.Background(), "test prompt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if content != "test response" {
		t.Errorf("expected 'test response', got '%s'", content)
	}
}

func TestMinimaxClient_CircuitBreaker(t *testing.T) {
	failCount := 0
	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if failCount < 3 {
			failCount++
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		response := map[string]interface{}{
			"choices": []map[string]interface{}{
				{
					"message": map[string]interface{}{
						"content": "test response",
					},
				},
			},
		}
		json.NewEncoder(w).Encode(response)
	}))
	defer mockServer.Close()

	originalURL := GetMinimaxAPIURL()
	defer func() { SetMinimaxAPIURL(originalURL) }()
	SetMinimaxAPIURL(mockServer.URL)

	client := NewMinimaxClient("test-key")

	// Trigger 3 failures
	for i := 0; i < 3; i++ {
		_, err := client.Reason(context.Background(), "test")
		if err == nil {
			t.Fatalf("expected error on failure %d", i+1)
		}
	}

	// Next request should fail immediately with open circuit breaker error
	_, err := client.Reason(context.Background(), "test")
	if err == nil || err.Error() != "minimax API circuit breaker is open" {
		t.Fatalf("expected open circuit breaker error, got %v", err)
	}

	// Manually reset time for test
	client.mu.Lock()
	client.lastFailure = time.Now().Add(-11 * time.Second)
	client.mu.Unlock()

	// Should allow one request, which will succeed and reset circuit breaker
	content, err := client.Reason(context.Background(), "test")
	if err != nil {
		t.Fatalf("expected success, got %v", err)
	}
	if content != "test response" {
		t.Fatalf("expected 'test response', got %q", content)
	}

	// Should be completely closed now
	if client.state != stateClosed {
		t.Fatalf("expected circuit breaker to be closed, got %v", client.state)
	}
}
