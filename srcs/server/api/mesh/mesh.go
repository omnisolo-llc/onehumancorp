package mesh

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
		"github.com/gorilla/websocket"
	"net/http"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"go.opentelemetry.io/otel"
)

var (
	meter             = otel.Meter("github.com/onehumancorp/mono/srcs/server/api/mesh")
	broadcastCount, _ = meter.Int64Counter("mesh.broadcast.count")
	subscribeCount, _ = meter.Int64Counter("mesh.subscribe.count")
)


type MeshEvent struct {
	AgentID string `json:"agent_id"`
	Channel string `json:"channel"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	Payload map[string]interface{} `json:"payload"`
}

type TeammateMeshService interface {
	BroadcastIntent(ctx context.Context, intent string) error
	Subscribe(ctx context.Context) (<-chan string, error)
}

type MemoryMeshService struct {
	subscribers map[chan string]struct{}
	mu          sync.RWMutex
}

func NewMemoryMeshService() *MemoryMeshService {
	return &MemoryMeshService{
		subscribers: make(map[chan string]struct{}),
	}
}

func (s *MemoryMeshService) BroadcastIntent(ctx context.Context, intent string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	broadcastCount.Add(ctx, 1)

	s.mu.RLock()
	defer s.mu.RUnlock()

	for sub := range s.subscribers {
		select {
		case sub <- intent:
		case <-time.After(10 * time.Millisecond): // Drop if blocked
		}
	}
	return nil
}

func (s *MemoryMeshService) Subscribe(ctx context.Context) (<-chan string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	subscribeCount.Add(ctx, 1)

	out := make(chan string, 100)

	s.mu.Lock()
	s.subscribers[out] = struct{}{}
	s.mu.Unlock()

	go func() {
		<-ctx.Done()
		s.mu.Lock()
		delete(s.subscribers, out)
		s.mu.Unlock()
		close(out)
	}()

	return out, nil
}

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
}

type MeshHandler struct {
	Service TeammateMeshService
}

func NewMeshHandler(service TeammateMeshService) *MeshHandler {
	return &MeshHandler{
		Service: service,
	}
}

func (h *MeshHandler) Broadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Fallback to parse either old payload or new KAIROS payload
	bodyBytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	var req struct {
		Intent string `json:"intent"`
	}
	var sipReq struct {
		AgentID   string          `json:"agent_id"`
		Channel   string          `json:"channel"`
		EventType string          `json:"event_type"`
		Data      json.RawMessage `json:"data"`
	}
	var intentStr string
	if err := json.Unmarshal(bodyBytes, &sipReq); err == nil {
		if sipReq.AgentID != "" && sipReq.Channel != "" && sipReq.EventType != "" && len(sipReq.Data) > 0 && string(sipReq.Data) != "null" {
			intentStr = string(bodyBytes)
		} else if sipReq.AgentID != "" || sipReq.Channel != "" || sipReq.EventType != "" {
			// It looks like a SIP request but is missing required fields
			http.Error(w, "Bad request: invalid SIP payload", http.StatusBadRequest)
			return
		} else if err := json.Unmarshal(bodyBytes, &req); err == nil && req.Intent != "" {
			intentStr = req.Intent
		} else {
			intentStr = string(bodyBytes)
		}
	} else if err := json.Unmarshal(bodyBytes, &req); err == nil && req.Intent != "" {
		intentStr = req.Intent
	} else {
		intentStr = string(bodyBytes)
	}

	if err := h.Service.BroadcastIntent(r.Context(), intentStr); err != nil {
		http.Error(w, fmt.Sprintf("Failed to broadcast: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *MeshHandler) Stream(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		// upgrader.Upgrade already replies with an error to the client
		return
	}
	defer conn.Close()

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	sub, err := h.Service.Subscribe(ctx)
	if err != nil {
		conn.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("Failed to subscribe: %v", err)))
		return
	}

	// Start a goroutine to read from the websocket to handle client disconnects
	go func() {
		defer cancel()
		for {
			if _, _, err := conn.ReadMessage(); err != nil {
				break
			}
		}
	}()

	for {
		select {
		case msg, ok := <-sub:
			if !ok {
				return
			}
			if err := conn.WriteMessage(websocket.TextMessage, []byte(msg)); err != nil {
				return
			}
		case <-ctx.Done():
			return
		}
	}
}

func (h *MeshHandler) Publish(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	bodyBytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	if err := h.Service.BroadcastIntent(r.Context(), string(bodyBytes)); err != nil {
		http.Error(w, fmt.Sprintf("Failed to publish: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *MeshHandler) Subscribe(w http.ResponseWriter, r *http.Request) {
	h.Stream(w, r)
}
// Adding a dummy comment

func (h *MeshHandler) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/mesh/publish", h.Publish)
	mux.HandleFunc("/mesh/subscribe", h.Subscribe)
}
