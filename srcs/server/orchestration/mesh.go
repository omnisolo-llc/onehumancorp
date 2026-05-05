package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"strings"

	"github.com/go-redis/redis/v8"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type AuthStore interface {
	ValidateToken(token string) bool
}

type AuthMiddleware struct {
	store AuthStore
}

func NewAuthMiddleware(store AuthStore) *AuthMiddleware {
	return &AuthMiddleware{store: store}
}

func (m *AuthMiddleware) Middleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		token := strings.TrimPrefix(authHeader, "Bearer ")
		if !m.store.ValidateToken(token) {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		next(w, r)
	}
}

type BroadcastPayload struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	Data    any    `json:"data,omitempty"`
}

type ClientInterface interface {
	Publish(ctx context.Context, channel string, message interface{}) *redis.IntCmd
}

type TeammateMesh struct {
	redisClient    ClientInterface
	publishCounter metric.Int64Counter
}

func NewTeammateMesh(redisURL string) (*TeammateMesh, error) {
	opt, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	meter := otel.Meter("teammate_mesh")
	publishCounter, _ := meter.Int64Counter("mesh.messages.published")

	return &TeammateMesh{
		redisClient:    redis.NewClient(opt),
		publishCounter: publishCounter,
	}, nil
}

func (m *TeammateMesh) BroadcastHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var payload BroadcastPayload
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}
	if payload.AgentID == "" || payload.Action == "" || payload.Status == "" {
		http.Error(w, "Missing fields", http.StatusBadRequest)
		return
	}
	dataBytes, _ := json.Marshal(payload)

	m.redisClient.Publish(context.Background(), "mesh:tasks", string(dataBytes))
	m.redisClient.Publish(context.Background(), "mesh:coordination", string(dataBytes))

	m.publishCounter.Add(r.Context(), 1)

	w.WriteHeader(http.StatusOK)
}

func RegisterRoutes(mux *http.ServeMux, mesh *TeammateMesh, authStore AuthStore) {
	authMw := NewAuthMiddleware(authStore)
	mux.HandleFunc("/api/mesh/broadcast", authMw.Middleware(mesh.BroadcastHandler))
}
