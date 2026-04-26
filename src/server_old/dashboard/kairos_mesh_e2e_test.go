package dashboard

import (
	"bytes"
	"crypto/tls"
	"crypto/x509"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/src/server_old/billing"
	"github.com/onehumancorp/mono/src/server_old/domain"
	"github.com/onehumancorp/mono/src/server_old/orchestration"
)

func TestKairosMesh_E2E_PublishSubscribe(t *testing.T) {
	org := domain.NewSoftwareCompany("org-1", "Acme Software", "Casey CEO", time.Date(2026, 3, 10, 0, 0, 0, 0, time.UTC))
	hub := orchestration.NewHub()
	defer hub.Close()
	tracker := billing.NewTracker(billing.DefaultCatalog)

	baseHandler := NewServer(org, hub, tracker)

	wrapper := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		r.TLS = &tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{{
				URIs: []*url.URL{{Scheme: "spiffe"}},
			}},
		}
		baseHandler.ServeHTTP(w, r)
	})

	server := httptest.NewServer(wrapper)
	defer server.Close()

	token := loginForTest(t, server.URL)

	// 1. Establish WebSocket Subscription
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "/api/kairos/mesh/subscribe?channel=e2e_test"
	dialer := websocket.Dialer{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}
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
