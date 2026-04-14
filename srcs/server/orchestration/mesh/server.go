package mesh

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"os"
	"strings"

	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type contextKey string

const spiffeContextKey contextKey = "spiffe_id"

var (
	meter                    = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration/mesh")
	meshBroadcastEventsTotal metric.Int64Counter
)

func init() {
	var err error
	meshBroadcastEventsTotal, err = meter.Int64Counter("mesh_broadcast_events_total")
	if err != nil {
		// Log or handle error if needed
	}
}

// MeshEvent represents the JSON contract for the Teammate Mesh broadcast.
type MeshEvent struct {
	EventID   string                 `json:"event_id"`
	AgentID   string                 `json:"agent_id"`
	Action    string                 `json:"action"`
	Status    string                 `json:"status"`
	Payload   map[string]interface{} `json:"payload"`
	Timestamp string                 `json:"timestamp"`
}

// MeshBroadcaster defines the transport logic interface.
type MeshBroadcaster interface {
	Broadcast(ctx context.Context, event MeshEvent) error
}

// RueidisBroadcaster implements MeshBroadcaster using rueidis for Cloud-Native mode.
type RueidisBroadcaster struct {
	client rueidis.Client
}

// NewRueidisBroadcaster creates a new RueidisBroadcaster.
func NewRueidisBroadcaster(client rueidis.Client) *RueidisBroadcaster {
	return &RueidisBroadcaster{client: client}
}

func (b *RueidisBroadcaster) Broadcast(ctx context.Context, event MeshEvent) error {
	payload, err := json.Marshal(event)
	if err != nil {
		return err
	}
	cmd := b.client.B().Publish().Channel("mesh:coordination").Message(string(payload)).Build()
	return b.client.Do(ctx, cmd).Error()
}

// ChannelBroadcaster implements MeshBroadcaster using a Go channel for Standalone mode.
type ChannelBroadcaster struct {
	ch chan MeshEvent
}

// NewChannelBroadcaster creates a new ChannelBroadcaster.
func NewChannelBroadcaster() *ChannelBroadcaster {
	return &ChannelBroadcaster{
		ch: make(chan MeshEvent, 100),
	}
}

func (b *ChannelBroadcaster) Broadcast(ctx context.Context, event MeshEvent) error {
	select {
	case b.ch <- event:
		return nil
	case <-ctx.Done():
		return ctx.Err()
	default:
		return errors.New("channel full")
	}
}

// MeshGateway provides the HTTP API gateway.
type MeshGateway struct {
	broadcaster MeshBroadcaster
}

// NewMeshGateway creates a MeshGateway and auto-wires the broadcaster based on OHC_MULTITENANT.
func NewMeshGateway() (*MeshGateway, error) {
	var broadcaster MeshBroadcaster
	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisAddr := os.Getenv("REDIS_URL")
		if redisAddr == "" {
			redisAddr = "127.0.0.1:6379"
		}
		client, err := rueidis.NewClient(rueidis.ClientOption{
			InitAddress: []string{strings.TrimPrefix(redisAddr, "redis://")},
		})
		if err != nil {
			return nil, err
		}
		broadcaster = NewRueidisBroadcaster(client)
	} else {
		broadcaster = NewChannelBroadcaster()
	}
	return &MeshGateway{broadcaster: broadcaster}, nil
}

// SPIFFEMiddleware securely authenticates via SPIFFE.
func SPIFFEMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var spiffeID string
		if r.TLS != nil && len(r.TLS.PeerCertificates) > 0 {
			if len(r.TLS.PeerCertificates[0].URIs) > 0 {
				spiffeID = r.TLS.PeerCertificates[0].URIs[0].String()
			}
		}

		if spiffeID == "" {
			http.Error(w, "Unauthorized: missing or invalid SPIFFE certificate", http.StatusUnauthorized)
			return
		}

		ctx := context.WithValue(r.Context(), spiffeContextKey, spiffeID)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

// BroadcastHandler handles POST /api/mesh/broadcast requests.
func (g *MeshGateway) BroadcastHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		return
	}

	spiffeID, ok := r.Context().Value(spiffeContextKey).(string)
	if !ok || spiffeID == "" {
		http.Error(w, "Unauthorized: invalid SPIFFE context", http.StatusUnauthorized)
		return
	}

	var event MeshEvent
	if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
		http.Error(w, "Bad Request: invalid payload", http.StatusBadRequest)
		return
	}

	// Payload Validation
	if event.EventID == "" || event.AgentID == "" || event.Action == "" || event.Status == "" || event.Timestamp == "" {
		http.Error(w, "Bad Request: missing required fields", http.StatusBadRequest)
		return
	}

	if event.AgentID != spiffeID {
		http.Error(w, "Forbidden: agent_id mismatch", http.StatusForbidden)
		return
	}

	if err := g.broadcaster.Broadcast(r.Context(), event); err != nil {
		http.Error(w, "Internal Server Error: "+err.Error(), http.StatusInternalServerError)
		return
	}

	if meshBroadcastEventsTotal != nil {
		meshBroadcastEventsTotal.Add(r.Context(), 1)
	}

	w.WriteHeader(http.StatusOK)
}

// Note: Real-time bi-directional and streaming Teammate Mesh features (gRPC and WebSockets)
// are implemented in srcs/server/orchestration/service.go (StreamMeshEvents)
// and srcs/server/orchestration/centrifuge_hub.go.
