package local

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestPlaywrightTool(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var req map[string]string
		json.NewDecoder(r.Body).Decode(&req)

		res := map[string]interface{}{
			"stdout": "Mocked " + req["type"],
			"stderr": "",
			"exit_code": 0,
		}
		json.NewEncoder(w).Encode(res)
	}))
	defer ts.Close()

	tool := &playwrightTool{daemonURL: ts.URL}

	// Test goto
	out, err := tool.Execute(context.Background(), "", map[string]interface{}{
		"action": "goto",
		"url": "http://example.com",
	})
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out, "Mocked goto") {
		t.Fatalf("unexpected output: %v", out)
	}
}
