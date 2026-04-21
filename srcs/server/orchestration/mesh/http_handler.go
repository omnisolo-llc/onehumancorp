package mesh

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all origins for mesh
	},
}

type HTTPHandler struct {
	Mesh TeammateMesh
}

func NewHTTPHandler(mesh TeammateMesh) *HTTPHandler {
	return &HTTPHandler{Mesh: mesh}
}

func (h *HTTPHandler) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// mTLS / SPIFFE Authorization Check
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)

	var payload struct {
		Channel   string          `json:"channel"`
		EventType string          `json:"event_type"`
		Data      json.RawMessage `json:"data"`
	}

	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid request format", http.StatusBadRequest)
		return
	}

	if payload.Channel == "" {
		http.Error(w, "invalid channel", http.StatusBadRequest)
		return
	}

	rawMsg, err := json.Marshal(payload)
	if err != nil {
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}

	if err := h.Mesh.Publish(r.Context(), payload.Channel, rawMsg); err != nil {
		http.Error(w, fmt.Sprintf("failed to broadcast: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (h *HTTPHandler) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// mTLS / SPIFFE Authorization Check
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	channel := r.URL.Query().Get("channel")
	if channel == "" {
		http.Error(w, "channel parameter is required", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	defer conn.Close()

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	sub, err := h.Mesh.Subscribe(ctx, channel, func(msg []byte) {
		_ = conn.WriteMessage(websocket.TextMessage, msg)
	})
	if err != nil {
		conn.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("failed to subscribe: %v", err)))
		return
	}
	defer sub.Close()

	// Read from connection to detect client disconnect
	for {
		if _, _, err := conn.ReadMessage(); err != nil {
			break
		}
	}
}
