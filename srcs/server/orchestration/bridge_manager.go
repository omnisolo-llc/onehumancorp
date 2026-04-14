package orchestration

import (
    "context"
    "encoding/json"
    "sync"
    "time"
    "net/http"
    "net/url"

    "github.com/gorilla/websocket"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

// BridgeManager manages outbound and inbound connections to remote swarms.
type BridgeManager struct {
    mu            sync.RWMutex
    centrifuge    *CentrifugeNode
    activeBridges map[string]*websocket.Conn
    bridgeTopics  map[string]bool
}

func NewBridgeManager(cn *CentrifugeNode) *BridgeManager {
    return &BridgeManager{
        centrifuge:    cn,
        activeBridges: make(map[string]*websocket.Conn),
        bridgeTopics:  map[string]bool{"mesh:tasks:shared": true},
    }
}

func (bm *BridgeManager) ConnectToRemoteSwarm(ctx context.Context, remoteURL string) error {
    u, err := url.Parse(remoteURL)
    if err != nil {
        return err
    }

    // Simulated backoff/retry
    var conn *websocket.Conn
    for i := 0; i < 3; i++ {
        dialer := websocket.Dialer{HandshakeTimeout: 10 * time.Second}
        conn, _, err = dialer.DialContext(ctx, u.String(), nil)
        if err == nil {
            break
        }
        time.Sleep(time.Duration(1<<i) * time.Second)
    }

    if err != nil {
        return err
    }

    bm.mu.Lock()
    bm.activeBridges[remoteURL] = conn
    bm.mu.Unlock()

    go bm.listenToRemote(ctx, remoteURL, conn)
    return nil
}

func (bm *BridgeManager) listenToRemote(ctx context.Context, remoteURL string, conn *websocket.Conn) {
	defer conn.Close()
    for {
        select {
        case <-ctx.Done():
            return
        default:
            _, msg, err := conn.ReadMessage()
            if err != nil {
                bm.mu.Lock()
                delete(bm.activeBridges, remoteURL)
                bm.mu.Unlock()
                return
            }
            // Re-broadcast to local mesh
            if bm.centrifuge != nil && bm.centrifuge.meshTransport != nil {
                bm.centrifuge.meshTransport.BroadcastMeshEvent(ctx, "bridge_inbound", msg)
            }
            telemetry.RecordAgentApiCall(ctx, "bridge", "system", "message_received")
        }
    }
}

func (bm *BridgeManager) ForwardEvent(ctx context.Context, topic string, payload []byte) error {
    bm.mu.RLock()
    defer bm.mu.RUnlock()

    if !bm.bridgeTopics[topic] {
        return nil // Topic not bridgeable
    }

    for _, conn := range bm.activeBridges {
        err := conn.WriteMessage(websocket.TextMessage, payload)
        if err != nil {
            continue // Handled in listenToRemote (disconnection)
        }
        telemetry.RecordAgentApiCall(ctx, "bridge", "system", "message_sent")
    }
    return nil
}

// HandleConnectRequest implements POST /api/v1/mesh/bridge/connect
func (bm *BridgeManager) HandleConnectRequest(w http.ResponseWriter, r *http.Request) {
    var req struct {
        RemoteURL string `json:"remote_url"`
    }
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    if err := bm.ConnectToRemoteSwarm(r.Context(), req.RemoteURL); err != nil {
		http.Error(w, "Failed to connect to remote swarm", http.StatusInternalServerError)
		return
	}
    w.WriteHeader(http.StatusOK)
}

// HandleStatusRequest implements GET /api/v1/mesh/bridge/status
func (bm *BridgeManager) HandleStatusRequest(w http.ResponseWriter, r *http.Request) {
    bm.mu.RLock()
    defer bm.mu.RUnlock()

    status := make(map[string]string)
    for url := range bm.activeBridges {
        status[url] = "ACTIVE"
    }

    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(status)
}
