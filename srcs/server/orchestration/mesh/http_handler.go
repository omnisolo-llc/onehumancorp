package mesh

import (
	"encoding/json"
	"net/http"
)

// ValidationMiddleware enforces mTLS checks for mesh APIs.
func ValidationMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
			http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
			return
		}
		cert := r.TLS.PeerCertificates[0]
		if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
			http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
			return
		}
		next.ServeHTTP(w, r)
	}
}

func HandleMeshV2Broadcast(broker MeshBroker) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		r.Body = http.MaxBytesReader(w, r.Body, 1<<20)

		var req struct {
			Channel string                 `json:"channel"`
			Data    map[string]interface{} `json:"data"`
		}
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "invalid request", http.StatusBadRequest)
			return
		}

		if req.Channel == "" {
			http.Error(w, "invalid channel", http.StatusBadRequest)
			return
		}

		payloadBytes, err := json.Marshal(req.Data)
		if err != nil {
			http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
			return
		}

		if err := broker.Broadcast(r.Context(), req.Channel, payloadBytes); err != nil {
			http.Error(w, "failed to broadcast", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte(`{"status":"ok"}`))
	}
}
