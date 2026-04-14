package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

type SIPPayload struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
}

type MeshTransport interface {
	Broadcast(ctx context.Context, payload SIPPayload) error
	Subscribe(ctx context.Context, ch chan<- SIPPayload) error
	Unsubscribe(ctx context.Context, ch chan<- SIPPayload) error
}

type MemoryMeshTransport struct {
	subscribers []chan<- SIPPayload
	mu          sync.RWMutex
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	return &MemoryMeshTransport{
		subscribers: make([]chan<- SIPPayload, 0),
	}
}

func (m *MemoryMeshTransport) Broadcast(ctx context.Context, payload SIPPayload) error {
	m.mu.RLock()
	defer m.mu.RUnlock()
	for _, ch := range m.subscribers {
		select {
		case ch <- payload:
		default: // non-blocking send
		}
	}
	return nil
}

func (m *MemoryMeshTransport) Subscribe(ctx context.Context, ch chan<- SIPPayload) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.subscribers = append(m.subscribers, ch)
	return nil
}

func (m *MemoryMeshTransport) Unsubscribe(ctx context.Context, ch chan<- SIPPayload) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	for i, sub := range m.subscribers {
		if sub == ch {
			m.subscribers = append(m.subscribers[:i], m.subscribers[i+1:]...)
			break
		}
	}
	return nil
}

type MeshHandler struct {
	transport MeshTransport
	upgrader  websocket.Upgrader
}

func NewMeshHandler(transport MeshTransport) *MeshHandler {
	return &MeshHandler{
		transport: transport,
		upgrader: websocket.Upgrader{
			CheckOrigin: func(r *http.Request) bool {
				return true
			},
		},
	}
}

func (h *MeshHandler) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	bodyBytes, err := io.ReadAll(io.LimitReader(r.Body, 1024*1024))
	if err != nil {
		http.Error(w, "Failed to read body", http.StatusInternalServerError)
		return
	}
	r.Body = io.NopCloser(bytes.NewBuffer(bodyBytes))

	var payload SIPPayload
	if err := json.Unmarshal(bodyBytes, &payload); err != nil {
		http.Error(w, "Invalid JSON", http.StatusBadRequest)
		return
	}

	if payload.AgentID == "" || payload.Action == "" || payload.Status == "" {
		http.Error(w, "Missing required OHC-SIP fields", http.StatusBadRequest)
		return
	}

	err = h.transport.Broadcast(r.Context(), payload)
	if err != nil {
		http.Error(w, "Broadcast failed", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"broadcast_queued"}`))
}

func (h *MeshHandler) HandleSubscribeWS(w http.ResponseWriter, r *http.Request) {
	conn, err := h.upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	defer conn.Close()

	msgChan := make(chan SIPPayload, 100)
	h.transport.Subscribe(context.Background(), msgChan)
	defer h.transport.Unsubscribe(context.Background(), msgChan)

	// reader loop to handle client disconnects
	go func() {
		for {
			if _, _, err := conn.ReadMessage(); err != nil {
				return
			}
		}
	}()

	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-r.Context().Done():
			return
		case msg := <-msgChan:
			conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
			err := conn.WriteJSON(msg)
			if err != nil {
				return
			}
		case <-ticker.C:
			conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
			if err := conn.WriteMessage(websocket.PingMessage, nil); err != nil {
				return
			}
		}
	}
}

// HandleSubscribe supports both SSE and Websockets based on Accept or Upgrade headers
func (h *MeshHandler) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	if r.Header.Get("Upgrade") == "websocket" {
		h.HandleSubscribeWS(w, r)
		return
	}

	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	msgChan := make(chan SIPPayload, 100)
	h.transport.Subscribe(r.Context(), msgChan)

	defer h.transport.Unsubscribe(r.Context(), msgChan)

	for {
		select {
		case msg := <-msgChan:
			msgBytes, _ := json.Marshal(msg)
			w.Write([]byte("data: "))
			w.Write(msgBytes)
			w.Write([]byte("\n\n"))
			flusher.Flush()
		case <-r.Context().Done():
			return
		case <-time.After(30 * time.Second):
			// Keep-alive ping
			w.Write([]byte(":\n\n"))
			flusher.Flush()
		}
	}
}
