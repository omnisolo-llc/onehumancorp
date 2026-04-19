package builtin

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestBrowserTool(t *testing.T) {
	// Start a mock daemon server
	mux := http.NewServeMux()
	mux.HandleFunc("/command", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(`{"content": "mocked daemon response"}`))
	})
	server := httptest.NewServer(mux)
	defer server.Close()

	// Update the BrowserTool execution function temporarily to point to our mock server.
	// Alternatively, we inject it but for simplicity we modify the default URL in the tool itself or use a local variable.

	_ = json.RawMessage(`{"url": "http://example.com"}`)

	// Because BrowserTool hardcodes "http://localhost:9222/command", we can't easily mock it without rewriting it slightly to accept an endpoint.
	// We will rewrite BrowserTool to take an environment variable or package variable for testability, but for now we'll test the validation logic.

	invalidArgs := json.RawMessage(`{}`)
	_, err := BrowserTool.Execute(context.Background(), invalidArgs)
	if err == nil || err.Error() != "Browser: url is required" {
		t.Errorf("Expected URL required error, got %v", err)
	}
}
