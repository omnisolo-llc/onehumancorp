package mesh

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/gorilla/websocket"
	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
)

type MeshAPI struct {
	mesh        TeammateMesh
	OnBroadcast func(channel string, payload []byte)
}

func NewMeshAPI(mesh TeammateMesh) *MeshAPI {
	return &MeshAPI{mesh: mesh}
}

func (api *MeshAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/mesh/v2/broadcast", api.HandleBroadcast)
	mux.HandleFunc("/api/mesh/v2/subscribe", api.HandleSubscribe)
}

func (api *MeshAPI) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "failed to read body", http.StatusInternalServerError)
		return
	}

	var req pb.PublishTeammateMeshEventRequest
	if err := protojson.Unmarshal(body, &req); err != nil {
		http.Error(w, "bad request: expected protobuf JSON", http.StatusBadRequest)
		return
	}

	if req.GetChannel() == "" {
		http.Error(w, "missing channel", http.StatusBadRequest)
		return
	}

	if req.GetChannel() == "mesh:tasks" || req.GetChannel() == "mesh:coordination" {
		if req.GetEvent() == nil || req.GetEvent().GetAgentId() == "" || req.GetEvent().GetAction() == "" || req.GetEvent().GetStatus() == "" {
			http.Error(w, "invalid request: missing required fields (agent_id, action, status)", http.StatusBadRequest)
			return
		}
	}

	payloadBytes, err := proto.Marshal(&req)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := api.mesh.Publish(r.Context(), req.GetChannel(), payloadBytes); err != nil {
		http.Error(w, "failed to publish", http.StatusInternalServerError)
		return
	}

	if api.OnBroadcast != nil {
		api.OnBroadcast(req.GetChannel(), payloadBytes)
	}

	telemetry.RecordTeammateMeshBroadcast(r.Context(), req.GetChannel())

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}

func (api *MeshAPI) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	channel := r.URL.Query().Get("channel")
	if channel == "" {
		http.Error(w, "missing channel", http.StatusBadRequest)
		return
	}

	defaultUpgrader := websocket.Upgrader{
		CheckOrigin: func(r *http.Request) bool {
			return true
		},
	}

	conn, err := defaultUpgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	defer conn.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	sub, err := api.mesh.Subscribe(ctx, channel, func(msg []byte) {
		conn.WriteMessage(websocket.TextMessage, msg)
	})
	if err != nil {
		conn.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("Failed to subscribe: %v", err)))
		return
	}
	defer sub.Close()

	clientDone := make(chan struct{})

	go func() {
		defer close(clientDone)
		for {
			if _, _, err := conn.ReadMessage(); err != nil {
				break
			}
		}
	}()

	for {
		select {
		case <-clientDone:
			return
		case <-r.Context().Done():
			return
		}
	}
}
