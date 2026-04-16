package mesh

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
)

var (
	meter             = otel.Meter("github.com/onehumancorp/mono/srcs/server/api/mesh")
	broadcastCount, _ = meter.Int64Counter("mesh.broadcast.count")
	subscribeCount, _ = meter.Int64Counter("mesh.subscribe.count")
)

type TeammateMeshService interface {
	BroadcastIntent(ctx context.Context, intent string) error
	Subscribe(ctx context.Context) (<-chan string, error)
}

type RedisMeshService struct {
	client  rueidis.Client
	channel string
}

func NewRedisMeshService(client rueidis.Client, channel string) *RedisMeshService {
	return &RedisMeshService{
		client:  client,
		channel: channel,
	}
}

func (s *RedisMeshService) BroadcastIntent(ctx context.Context, intent string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	broadcastCount.Add(ctx, 1)

	var req struct {
		Channel string `json:"channel"`
	}
	ch := s.channel
	if err := json.Unmarshal([]byte(intent), &req); err == nil && req.Channel != "" {
		ch = req.Channel
	}

	cmd := s.client.B().Publish().Channel(ch).Message(intent).Build()
	return s.client.Do(ctx, cmd).Error()
}

func (s *RedisMeshService) Subscribe(ctx context.Context) (<-chan string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	subscribeCount.Add(ctx, 1)

	out := make(chan string, 100)

	go func() {
		defer close(out)

		err := s.client.Receive(ctx, s.client.B().Subscribe().Channel(s.channel).Build(), func(msg rueidis.PubSubMessage) {
			select {
			case out <- msg.Message:
			case <-ctx.Done():
			}
		})
		if err != nil && err != context.Canceled {
			// In a real application, handle error logging
		}
	}()

	return out, nil
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

	var req struct {
		Intent string `json:"intent"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	if err := h.Service.BroadcastIntent(r.Context(), req.Intent); err != nil {
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

	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming not supported", http.StatusInternalServerError)
		return
	}

	sub, err := h.Service.Subscribe(r.Context())
	if err != nil {
		http.Error(w, fmt.Sprintf("Failed to subscribe: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	for {
		select {
		case msg, ok := <-sub:
			if !ok {
				return
			}
			fmt.Fprintf(w, "data: %s\n\n", msg)
			flusher.Flush()
		case <-r.Context().Done():
			return
		}
	}
}
