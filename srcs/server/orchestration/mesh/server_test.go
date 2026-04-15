package mesh

import (
    "bytes"
    "net/http"
    "net/http/httptest"
    "testing"
    "crypto/tls"
    "crypto/x509"
    "net/url"
)

func TestMeshBroadcastValidation(t *testing.T) {
    s := NewMeshServer(nil)

    body := []byte(`{"agent_id": "", "action": "test", "status": "ok"}`)
    req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer(body))

    req.TLS = &tls.ConnectionState{
        PeerCertificates: []*x509.Certificate{
            {URIs: []*url.URL{{Scheme: "spiffe", Host: "example.org"}}},
        },
    }

    w := httptest.NewRecorder()
    s.HandleMeshBroadcast(w, req)

    if w.Code != http.StatusBadRequest {
        t.Errorf("Expected 400 Bad Request for invalid payload, got %d", w.Code)
    }
}
