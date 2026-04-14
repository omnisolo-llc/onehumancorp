package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type BridgeManager struct {
	dbProvider db.Provider
	hub        *CentrifugeNode
	bridges    map[string]*websocket.Conn
	mu         sync.Mutex
}

func NewBridgeManager(dbProvider db.Provider, hub *CentrifugeNode) *BridgeManager {
	return &BridgeManager{
		dbProvider: dbProvider,
		hub:        hub,
		bridges:    make(map[string]*websocket.Conn),
	}
}

func (bm *BridgeManager) Connect(ctx context.Context, localOrgID, remoteURL, remoteOrgID string) error {
	bm.mu.Lock()
	defer bm.mu.Unlock()

	var err error
	var conn *websocket.Conn
	backoff := 100 * time.Millisecond

	for i := 0; i < 5; i++ {
		dialer := websocket.DefaultDialer
		headers := http.Header{}
		headers.Add("Authorization", "Bearer SVID-"+localOrgID) // SPIFFE Mock

		conn, _, err = dialer.Dial(remoteURL, headers)
		if err == nil {
			break
		}
		time.Sleep(backoff)
		backoff *= 2
	}

	if err != nil {
		return err
	}

	bm.bridges[remoteOrgID] = conn
	telemetry.RecordMeshBridgeStatus(ctx, localOrgID, 1)

	go bm.listenToRemote(ctx, localOrgID, remoteOrgID, conn)
	return nil
}

func (bm *BridgeManager) listenToRemote(ctx context.Context, localOrgID, remoteOrgID string, conn *websocket.Conn) {
	defer func() {
		conn.Close()
		bm.mu.Lock()
		delete(bm.bridges, remoteOrgID)
		bm.mu.Unlock()
		telemetry.RecordMeshBridgeStatus(ctx, localOrgID, -1)
	}()

	for {
		_, msg, err := conn.ReadMessage()
		if err != nil {
			return
		}

		var event map[string]interface{}
		if json.Unmarshal(msg, &event) == nil {
			if bm.hub != nil {
				bm.hub.PublishTaskBroadcast(remoteOrgID, event)
			}
			telemetry.RecordMeshBridgeMessageReceived(ctx, localOrgID)
		}
	}
}

func (bm *BridgeManager) ForwardEvent(ctx context.Context, localOrgID, remoteOrgID string, event map[string]interface{}) error {
	bm.mu.Lock()
	conn, ok := bm.bridges[remoteOrgID]
	bm.mu.Unlock()

	if !ok {
		return nil
	}

	msg, err := json.Marshal(event)
	if err != nil {
		return err
	}

	err = conn.WriteMessage(websocket.TextMessage, msg)
	if err == nil {
		telemetry.RecordMeshBridgeMessageSent(ctx, localOrgID)
	}
	return err
}
