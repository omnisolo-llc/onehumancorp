package harness

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestDaemonHarness(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var req map[string]string
		json.NewDecoder(r.Body).Decode(&req)

		res := map[string]interface{}{
			"stdout": "Harness Mocked " + req["type"],
			"stderr": "",
			"exit_code": 0,
		}
		json.NewEncoder(w).Encode(res)
	}))
	defer ts.Close()

	harness := NewDaemonHarness(ts.URL)

	res, err := harness.Execute(context.Background(), "playwright goto http://example.com")
	if err != nil {
		t.Fatal(err)
	}
	if res.Stdout != "Harness Mocked goto" {
		t.Fatalf("unexpected stdout: %s", res.Stdout)
	}
}
