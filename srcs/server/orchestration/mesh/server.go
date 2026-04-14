package mesh

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"

	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration/mesh")
	meshBroadcastEventsTotal metric.Int64Counter
)

func init() {
	var err error
	meshBroadcastEventsTotal, err = meter.Int64Counter(
		"mesh_broadcast_events_total",
		metric.WithDescription("Total number of Teammate Mesh broadcast events"),
	)
	if err != nil {
		panic(err)
	}
}

// SPIFFEMiddleware enforces mTLS SPIFFE identity for incoming HTTP requests.
func SPIFFEMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
			http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
			return
		}
		cert := r.TLS.PeerCertificates[0]
		if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
			http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
			return
		}

		spiffeID := cert.URIs[0].String()
		ctx := context.WithValue(r.Context(), "spiffe_id", spiffeID)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

type MeshEvent struct {
	EventID   string                 `json:"event_id"`
	AgentID   string                 `json:"agent_id"`
	Action    string                 `json:"action"`
	Status    string                 `json:"status"`
	Payload   map[string]interface{} `json:"payload"`
	Timestamp string                 `json:"timestamp"`
}

func (e *MeshEvent) Validate() error {
	if e.EventID == "" {
		return fmt.Errorf("event_id is required")
	}
	if e.AgentID == "" {
		return fmt.Errorf("agent_id is required")
	}
	if e.Action == "" {
		return fmt.Errorf("action is required")
	}
	if e.Status == "" {
		return fmt.Errorf("status is required")
	}
	if e.Timestamp == "" {
		return fmt.Errorf("timestamp is required")
	}
	return nil
}

type MeshBroadcaster interface {
	Broadcast(ctx context.Context, channel string, event MeshEvent) error
}

type CloudMeshBroadcaster struct {
	client rueidis.Client
}

func NewCloudMeshBroadcaster(client rueidis.Client) *CloudMeshBroadcaster {
	return &CloudMeshBroadcaster{client: client}
}

func (b *CloudMeshBroadcaster) Broadcast(ctx context.Context, channel string, event MeshEvent) error {
	data, err := json.Marshal(event)
	if err != nil {
		return err
	}
	cmd := b.client.B().Publish().Channel(channel).Message(string(data)).Build()
	return b.client.Do(ctx, cmd).Error()
}

type StandaloneMeshBroadcaster struct {
	ch chan MeshEvent
	mu sync.RWMutex
}

func NewStandaloneMeshBroadcaster() *StandaloneMeshBroadcaster {
	return &StandaloneMeshBroadcaster{
		ch: make(chan MeshEvent, 100),
	}
}

func (b *StandaloneMeshBroadcaster) Broadcast(ctx context.Context, channel string, event MeshEvent) error {
	b.mu.RLock()
	defer b.mu.RUnlock()
	select {
	case b.ch <- event:
	case <-ctx.Done():
		return ctx.Err()
	default:
		// Drop message if channel is full
	}
	return nil
}

type Server struct {
	broadcaster MeshBroadcaster
}

func NewServer(broadcaster MeshBroadcaster) *Server {
	return &Server{broadcaster: broadcaster}
}

// RegisterHandlers registers the Teammate Mesh APIs
func (s *Server) RegisterHandlers(mux *http.ServeMux) {
	mux.Handle("/api/mesh/broadcast", SPIFFEMiddleware(http.HandlerFunc(s.HandleBroadcast)))
}

func (s *Server) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	spiffeID, ok := r.Context().Value("spiffe_id").(string)
	if !ok || spiffeID == "" {
		http.Error(w, "unauthorized: missing or invalid SPIFFE ID", http.StatusUnauthorized)
		return
	}

	var event MeshEvent
	if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
		http.Error(w, "invalid json payload", http.StatusBadRequest)
		return
	}

	if err := event.Validate(); err != nil {
		http.Error(w, fmt.Sprintf("validation failed: %v", err), http.StatusBadRequest)
		return
	}

	if event.AgentID != spiffeID {
		http.Error(w, "agent_id in payload does not match SPIFFE ID", http.StatusForbidden)
		return
	}

	if err := s.broadcaster.Broadcast(r.Context(), "mesh:coordination", event); err != nil {
		http.Error(w, fmt.Sprintf("broadcast failed: %v", err), http.StatusInternalServerError)
		return
	}

	meshBroadcastEventsTotal.Add(r.Context(), 1)
	w.WriteHeader(http.StatusOK)
}
