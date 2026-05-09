package dashboard

import (
	"fmt"
	"net/http"
	"sync"
	"time"

	"onehumancorp/srcs/server/telemetry"
)

type EventBroker struct {
	mu          sync.RWMutex
	clients     map[chan string]bool
	newClients  chan chan string
	deadClients chan chan string
	messages    chan string
}

func NewEventBroker() *EventBroker {
	return &EventBroker{
		clients:     make(map[chan string]bool),
		newClients:  make(chan chan string),
		deadClients: make(chan chan string),
		messages:    make(chan string),
	}
}

func (b *EventBroker) Start() {
	go func() {
		for {
			select {
			case s := <-b.newClients:
				b.mu.Lock()
				b.clients[s] = true
				b.mu.Unlock()
			case s := <-b.deadClients:
				b.mu.Lock()
				delete(b.clients, s)
				close(s)
				b.mu.Unlock()
			case msg := <-b.messages:
				b.mu.RLock()
				for s := range b.clients {
					select {
					case s <- msg:
					default:
					}
				}
				b.mu.RUnlock()
			}
		}
	}()
}

var GlobalBroker = NewEventBroker()

// Pre-allocate successful response payloads
var (
	autoDreamSyncSuccessJSON  = []byte(`{"status":"ok"}`)
	autoDreamQuerySuccessJSON = []byte(`{"results":[]}`)
	meshBroadcastSuccessJSON  = []byte(`{"status":"broadcasted"}`)
)

func init() {
	GlobalBroker.Start()
}

func HandleStream(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	msgChan := make(chan string, 1) // Buffer channel to prevent blocking
	GlobalBroker.newClients <- msgChan
	defer func() {
		GlobalBroker.deadClients <- msgChan
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
	w.Write(autoDreamSyncSuccessJSON)
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
	w.Write(autoDreamQuerySuccessJSON)
}

func HandleMeshBroadcast(w http.ResponseWriter, r *http.Request) {
	deploymentMode := r.Header.Get("X-Deployment-Mode")
	if deploymentMode == "" {
		deploymentMode = "standalone"
	}

	telemetry.MeshBroadcastTotal.WithLabelValues(deploymentMode).Inc()

	w.WriteHeader(http.StatusOK)
	w.Write(meshBroadcastSuccessJSON)
}
