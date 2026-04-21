package mesh

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func createMockTLSRequest(method, url string, body []byte, hasCert bool) *http.Request {
	req := httptest.NewRequest(method, url, bytes.NewBuffer(body))
	if hasCert {
		req.TLS = &tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{
				{
					URIs: []*url.URL{
						{Scheme: "spiffe", Host: "example.org", Path: "/workload"},
					},
				},
			},
		}
	} else {
		req.TLS = nil
	}
	return req
}

func TestHTTPHandler_HandleBroadcast_NoAuth(t *testing.T) {
	mesh := NewLocalMesh()
	handler := NewHTTPHandler(mesh)

	req := httptest.NewRequest(http.MethodPost, "/broadcast", bytes.NewBuffer([]byte(`{}`)))
	w := httptest.NewRecorder()

	handler.HandleBroadcast(w, req)

	if w.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden without TLS certs, got %d", w.Code)
	}
}

func TestHTTPHandler_HandleBroadcast_Valid(t *testing.T) {
	mesh := NewLocalMesh()
	handler := NewHTTPHandler(mesh)

	payload := []byte(`{"channel":"test_channel","event_type":"msg","data":{"foo":"bar"}}`)
	req := createMockTLSRequest(http.MethodPost, "/broadcast", payload, true)
	w := httptest.NewRecorder()

	handler.HandleBroadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected 200 OK, got %d", w.Code)
	}
}

func TestHTTPHandler_HandleSubscribe_Websocket(t *testing.T) {
	mesh := NewLocalMesh()
	handler := NewHTTPHandler(mesh)

	s := httptest.NewTLSServer(http.HandlerFunc(handler.HandleSubscribe))
	defer s.Close()

	u, _ := url.Parse(s.URL)
	u.Scheme = "wss"
	u.RawQuery = "channel=test_channel"

	// Create a client with skipping cert verification to connect to httptest TLS server
	dialer := websocket.Dialer{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}
	conn, _, err := dialer.Dial(u.String(), nil)
	// We expect 403 because we didn't send a client SPIFFE cert in the dialer
	if err == nil {
		conn.Close()
		t.Fatalf("expected error dialing websocket without SPIFFE cert")
	}
}
