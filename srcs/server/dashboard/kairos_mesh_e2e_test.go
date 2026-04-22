package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func TestKairosMesh_E2E_PublishSubscribe(t *testing.T) {
	_, server, token := newTestServer(t)
	defer server.Close()

	// 1. Establish WebSocket Subscription
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "/api/kairos/mesh/subscribe?channel=e2e_test"
	dialer := websocket.Dialer{}
	headers := http.Header{}
	headers.Add("Authorization", "Bearer "+token)

	ws, _, err := dialer.Dial(wsURL, headers)
	if err != nil {
	    t.Fatalf("failed to dial: %v", err)
	}
	defer ws.Close()

	// Wait a moment for subscription to propagate
	time.Sleep(100 * time.Millisecond)

	// 2. Publish Message via HTTP API
	pubURL := server.URL + "/api/kairos/mesh/publish"
	pubPayload := map[string]interface{}{
		"channel": "e2e_test",
		"message": map[string]string{"event": "test_event"},
	}
	bodyPub, _ := json.Marshal(pubPayload)
	req, _ := http.NewRequest(http.MethodPost, pubURL, bytes.NewBuffer(bodyPub))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
	    t.Fatalf("failed to post: %v", err)
	}

	if resp.StatusCode != http.StatusOK {
	    var errResult map[string]interface{}
	    json.NewDecoder(resp.Body).Decode(&errResult)
	    t.Errorf("expected 200, got %d, error: %v", resp.StatusCode, errResult)
	}
	resp.Body.Close()

	// 3. Verify Message Received via WebSocket
	ws.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, msg, err := ws.ReadMessage()
	if err != nil {
	    t.Fatalf("failed to read message: %v", err)
	}
	if !strings.Contains(string(msg), "test_event") {
	    t.Errorf("expected message to contain test_event, got %s", string(msg))
	}
}
