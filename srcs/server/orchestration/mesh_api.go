package orchestration

import (
	"io"
	"net/http"
	"google.golang.org/protobuf/encoding/protojson"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

type MeshAPI struct {
	hub *Hub
}

func NewMeshAPI(hub *Hub) *MeshAPI {
	return &MeshAPI{
		hub: hub,
	}
}

func (api *MeshAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/v1/orchestration/mesh/broadcast", api.HandleBroadcast)
	mux.HandleFunc("/v1/orchestration/tasks/stream", api.HandleStreamTasks)
}

func (api *MeshAPI) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var event pb.MeshEvent

	bodyBytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	if err := protojson.Unmarshal(bodyBytes, &event); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if api.hub != nil && api.hub.CentrifugeNode() != nil && api.hub.CentrifugeNode().meshTransport != nil {
		err := api.hub.CentrifugeNode().meshTransport.BroadcastMeshEvent(r.Context(), event.GetTopic(), event.GetPayload())
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	}
	w.WriteHeader(http.StatusOK)
}

func (api *MeshAPI) HandleStreamTasks(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	if api.hub == nil || api.hub.CentrifugeNode() == nil || api.hub.CentrifugeNode().meshTransport == nil {
		http.Error(w, "Mesh transport not available", http.StatusInternalServerError)
		return
	}

	eventsChan, err := api.hub.CentrifugeNode().meshTransport.SubscribeMeshEvents(r.Context(), "tasks")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
		return
	}

	w.Write([]byte("data: connected\n\n"))
	flusher.Flush()

	for {
		select {
		case <-r.Context().Done():
			return
		case payload, ok := <-eventsChan:
			if !ok {
				return
			}
			w.Write([]byte("data: "))
			w.Write(payload)
			w.Write([]byte("\n\n"))
			flusher.Flush()
		}
	}
}
