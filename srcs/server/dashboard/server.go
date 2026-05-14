package dashboard

import (
	"fmt"
	"net/http"
	"sync"
	"time"

	"onehumancorp/srcs/server/telemetry"
	"onehumancorp/srcs/server/onboarding"
)

type ClientConnection struct {
	TenantID string
	Chan     chan string
}

type TenantMessage struct {
	TenantID string
	Message  string
}

type EventBroker struct {
	mu          sync.Mutex
	clients     map[string]map[chan string]bool
	newClients  chan ClientConnection
	deadClients chan ClientConnection
	messages    chan TenantMessage
}

func NewEventBroker() *EventBroker {
	return &EventBroker{
		clients:     make(map[string]map[chan string]bool),
		newClients:  make(chan ClientConnection),
		deadClients: make(chan ClientConnection),
		messages:    make(chan TenantMessage),
	}
}

func (b *EventBroker) Start() {
	go func() {
		for {
			select {
			case s := <-b.newClients:
				b.mu.Lock()
				if _, ok := b.clients[s.TenantID]; !ok {
					b.clients[s.TenantID] = make(map[chan string]bool)
				}
				b.clients[s.TenantID][s.Chan] = true
				b.mu.Unlock()
			case s := <-b.deadClients:
				b.mu.Lock()
				if tenantClients, ok := b.clients[s.TenantID]; ok {
					delete(tenantClients, s.Chan)
					if len(tenantClients) == 0 {
						delete(b.clients, s.TenantID)
					}
				}
				close(s.Chan)
				b.mu.Unlock()
			case msg := <-b.messages:
				b.mu.Lock()
				if tenantClients, ok := b.clients[msg.TenantID]; ok {
					for s := range tenantClients {
						select {
						case s <- msg.Message:
						default:
						}
					}
				}
				b.mu.Unlock()
			}
		}
	}()
}


var GlobalBroker = NewEventBroker()

func init() {
	GlobalBroker.Start()
}

func HandleStream(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	tenantID, ok := r.Context().Value(onboarding.TenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	msgChan := make(chan string)
	clientConn := ClientConnection{TenantID: tenantID, Chan: msgChan}
	GlobalBroker.newClients <- clientConn
	defer func() {
		GlobalBroker.deadClients <- clientConn
	}()

	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming unsupported!", http.StatusInternalServerError)
		return
	}

	notify := r.Context().Done()

	for {
		select {
		case msg := <-msgChan:
			fmt.Fprintf(w, "data: %s\n\n", msg)
			flusher.Flush()
		case <-notify:
			return
		}
	}
}

func HandleAutoDreamSync(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	deploymentMode := r.Header.Get("X-Deployment-Mode")
	if deploymentMode == "" {
		deploymentMode = "standalone"
	}

	defer func() {
		telemetry.AutoDreamSyncDuration.WithLabelValues(deploymentMode).Observe(time.Since(start).Seconds())
	}()

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}

func HandleAutoDreamQuery(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	deploymentMode := r.Header.Get("X-Deployment-Mode")
	if deploymentMode == "" {
		deploymentMode = "standalone"
	}

	defer func() {
		telemetry.AutoDreamQueryDuration.WithLabelValues(deploymentMode).Observe(time.Since(start).Seconds())
	}()

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"results":[]}`))
}

func HandleMeshBroadcast(w http.ResponseWriter, r *http.Request) {
	deploymentMode := r.Header.Get("X-Deployment-Mode")
	if deploymentMode == "" {
		deploymentMode = "standalone"
	}

	telemetry.MeshBroadcastTotal.WithLabelValues(deploymentMode).Inc()

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"broadcasted"}`))
}
