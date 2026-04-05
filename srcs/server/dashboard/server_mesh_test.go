package dashboard

import (
	"bytes"
	"crypto/tls"
	"crypto/x509"
	"net/http"
	"net/http/httptest"
	"net/url"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestMeshEndpoints_RequireMTLS(t *testing.T) {
	s := &Server{}

	// Mock hub to prevent nil pointer panics deep in the stack
	s.hub = &orchestration.Hub{}


	endpoints := []struct {
		method string
		path   string
		body   []byte
	}{
		{"POST", "/api/mesh/broadcast", []byte(`{"channel":"mesh:tasks", "agent_id":"a1", "action":"test", "status":"ok"}`)},
		{"POST", "/api/mesh/direct", []byte(`{"toAgent":"a2", "payload":"data"}`)},
		{"GET", "/api/mesh/mailbox?agent_id=a1", nil},
	}

	for _, ep := range endpoints {
		t.Run(ep.path, func(t *testing.T) {
			// Test without TLS
			req := httptest.NewRequest(ep.method, ep.path, bytes.NewReader(ep.body))
			w := httptest.NewRecorder()

			switch ep.path {
			case "/api/mesh/broadcast":
				s.handleMeshBroadcast(w, req)
			case "/api/mesh/direct":
				s.handleMeshDirect(w, req)
			case "/api/mesh/mailbox?agent_id=a1":
				s.handleMeshMailbox(w, req)
			}

			if w.Code != http.StatusForbidden {
				t.Errorf("expected 403 Forbidden without TLS, got %d", w.Code)
			}

			// Test with empty TLS certificates
			req.TLS = &tls.ConnectionState{
				PeerCertificates: []*x509.Certificate{},
			}
			w = httptest.NewRecorder()

			switch ep.path {
			case "/api/mesh/broadcast":
				s.handleMeshBroadcast(w, req)
			case "/api/mesh/direct":
				s.handleMeshDirect(w, req)
			case "/api/mesh/mailbox?agent_id=a1":
				s.handleMeshMailbox(w, req)
			}

			if w.Code != http.StatusForbidden {
				t.Errorf("expected 403 Forbidden with empty TLS certs, got %d", w.Code)
			}

			// Test with valid TLS certificate but no SPIFFE URI
			req.TLS.PeerCertificates = []*x509.Certificate{{}}
			w = httptest.NewRecorder()

			switch ep.path {
			case "/api/mesh/broadcast":
				s.handleMeshBroadcast(w, req)
			case "/api/mesh/direct":
				s.handleMeshDirect(w, req)
			case "/api/mesh/mailbox?agent_id=a1":
				s.handleMeshMailbox(w, req)
			}

			if w.Code != http.StatusForbidden {
				t.Errorf("expected 403 Forbidden without SPIFFE URI, got %d", w.Code)
			}

			// Test with valid TLS certificate and SPIFFE URI
			importURL, _ := url.Parse("spiffe://onehumancorp.io/agent/test")
			req.TLS.PeerCertificates = []*x509.Certificate{{
				URIs: []*url.URL{importURL},
			}}
			w = httptest.NewRecorder()

			switch ep.path {
			case "/api/mesh/broadcast":
				s.handleMeshBroadcast(w, req)
			case "/api/mesh/direct":
				s.handleMeshDirect(w, req)
			case "/api/mesh/mailbox?agent_id=a1":
				s.handleMeshMailbox(w, req)
			}

			if w.Code == http.StatusForbidden {
				t.Errorf("did not expect 403 Forbidden with valid SPIFFE URI certs for %s", ep.path)
			}
		})
	}
}
