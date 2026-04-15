package mesh

import (
	"encoding/json"
	"net/http"
)

type HTTPHandler struct {
	broker MeshBroker
}

func NewHTTPHandler(broker MeshBroker) *HTTPHandler {
	return &HTTPHandler{broker: broker}
}

func (h *HTTPHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 || len(r.TLS.PeerCertificates[0].URIs) == 0 || r.TLS.PeerCertificates[0].URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)

	var req struct {
		Channel   string                 `json:"channel"`
		EventType string                 `json:"event_type"`
		Data      map[string]interface{} `json:"data"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(req.Data)
	if err != nil {
		http.Error(w, "marshal error", http.StatusInternalServerError)
		return
	}

	h.broker.Broadcast(r.Context(), req.Channel, payload)
	w.WriteHeader(http.StatusOK)
}
