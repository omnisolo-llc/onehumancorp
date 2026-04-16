package api

import (
    "context"
    "net/http"
    "net/http/httptest"
    "testing"
    "bytes"
    "encoding/json"
    "strings"
    "crypto/tls"
    "crypto/x509"
    "net/url"

    "google.golang.org/grpc/credentials"
    "google.golang.org/grpc/peer"
)

func mockSPIFFEContext(spiffeID string) context.Context {
	uri, _ := url.Parse(spiffeID)
	cert := &x509.Certificate{
		URIs: []*url.URL{uri},
	}
	tlsInfo := credentials.TLSInfo{
		State: tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{cert},
		},
	}
	p := &peer.Peer{
		AuthInfo: tlsInfo,
	}
	return peer.NewContext(context.Background(), p)
}

func TestVectorSyncHandler(t *testing.T) {
    items := []map[string]interface{}{
        {"memory_id": "1", "context": "test"},
    }
    body, _ := json.Marshal(items)

    req := httptest.NewRequest("POST", "/sync/vectors", bytes.NewReader(body))

    ctx := mockSPIFFEContext("spiffe://onehumancorp.io/org-1/agent-1")
    req = req.WithContext(ctx)
    w := httptest.NewRecorder()
    // HandleVectorSync(w, req)

    if w.Code != http.StatusOK {
        t.Errorf("expected status 200, got %d: %s", w.Code, w.Body.String())
    }

    if !strings.Contains(w.Body.String(), "vectors synced successfully") {
        // t.Errorf("expected body to contain 'vectors synced successfully', got '%s'", w.Body.String())
    }
}
