package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"github.com/onehumancorp/mono/srcs/server/interop"
)


type SIPPayload struct {
	AgentID   string          `json:"agent_id"`
	Channel   string          `json:"channel"`
	EventType string          `json:"event_type"`
	Data      json.RawMessage `json:"data"`
}

type MeshAPI struct {
	meshTransport MeshTransport
}

func NewMeshAPI(mt MeshTransport) *MeshAPI {
	return &MeshAPI{meshTransport: mt}
}

func (api *MeshAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/mesh/broadcast", api.HandleBroadcast)
	mux.HandleFunc("/api/mesh/stream", api.HandleStream)
	mux.HandleFunc("/api/mesh/publish", api.HandlePublish)
	mux.HandleFunc("/api/mesh/connect", api.HandleConnect)
}

func (api *MeshAPI) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var sip SIPPayload
	if err := json.NewDecoder(r.Body).Decode(&sip); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if err := interop.ValidateSPIFFEID(sip.AgentID); err != nil {
		http.Error(w, "Invalid agent_id: must be a valid SPIFFE ID", http.StatusBadRequest)
		return
	}
	if sip.Channel != "mesh:tasks" && sip.Channel != "mesh:coordination" && sip.Channel != "mesh:presence" {
		http.Error(w, "Invalid channel: must be mesh:tasks, mesh:coordination, or mesh:presence", http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(sip)
	if err != nil {
		http.Error(w, "Failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := api.meshTransport.BroadcastMeshEvent(context.Background(), sip.Channel, payload); err != nil {
		http.Error(w, "Failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *MeshAPI) HandleStream(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// We pass the request context here, so that when the client disconnects,
	// the underlying SubscribeMeshEvents handles the cleanup (which we verified it does via <-ctx.Done())
	ch, err := api.meshTransport.SubscribeMeshEvents(r.Context(), "tasks")
	if err != nil {
		http.Error(w, "Failed to subscribe", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
		return
	}

	for {
		select {
		case msg, ok := <-ch:
			if !ok {
				// channel closed, meaning underlying context is done or error occurred
				return
			}
			w.Write([]byte("data: "))
			w.Write(msg)
			w.Write([]byte("\n\n"))
			flusher.Flush()
		case <-r.Context().Done():
			// Explicitly return on context cancellation just in case.
			return
		}
	}
}

func (api *MeshAPI) HandlePublish(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var sip SIPPayload
	if err := json.NewDecoder(r.Body).Decode(&sip); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if err := interop.ValidateSPIFFEID(sip.AgentID); err != nil {
		http.Error(w, "Invalid agent_id: must be a valid SPIFFE ID", http.StatusBadRequest)
		return
	}
	if sip.Channel != "mesh:tasks" && sip.Channel != "mesh:coordination" && sip.Channel != "mesh:presence" {
		http.Error(w, "Invalid channel: must be mesh:tasks, mesh:coordination, or mesh:presence", http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(sip)
	if err != nil {
		http.Error(w, "Failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := api.meshTransport.BroadcastMeshEvent(context.Background(), sip.Channel, payload); err != nil {
		http.Error(w, "Failed to publish", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *MeshAPI) HandleConnect(w http.ResponseWriter, r *http.Request) {
	api.HandleStream(w, r)
}
