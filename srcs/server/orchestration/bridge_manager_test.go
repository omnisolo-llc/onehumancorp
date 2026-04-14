package orchestration

import (
    "context"
    "net/http/httptest"
    "net/http"
    "strings"
    "testing"
)

func TestBridgeManager_ConnectAndStatus(t *testing.T) {
    bm := NewBridgeManager(nil)

    // Test status initially
    req := httptest.NewRequest("GET", "/api/v1/mesh/bridge/status", nil)
    w := httptest.NewRecorder()
    bm.HandleStatusRequest(w, req)

    if w.Code != http.StatusOK {
        t.Errorf("expected 200 OK, got %d", w.Code)
    }

    // Test connect request format parsing (mocking actual dial via failure)
    body := `{"remote_url":"ws://invalid.url"}`
    req = httptest.NewRequest("POST", "/api/v1/mesh/bridge/connect", strings.NewReader(body))
    w = httptest.NewRecorder()
    bm.HandleConnectRequest(w, req)

    // Should fail to dial
    if w.Code != http.StatusInternalServerError {
		t.Errorf("expected 500 Internal Server Error due to bad dial, got %d", w.Code)
	}
	if !strings.Contains(w.Body.String(), "Failed to connect") {
		t.Errorf("expected error body to contain safe message, got: %s", w.Body.String())
	}

    // Test ForwardEvent returns nil for unbridged topic
    err := bm.ForwardEvent(context.Background(), "unbridged_topic", []byte("data"))
    if err != nil {
        t.Errorf("expected nil for unbridged topic, got %v", err)
    }
}
