package orchestration

import (
	"github.com/onehumancorp/mono/srcs/server/lib/resilience"

	"context"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// BridgeEnvelope is used to prevent infinite broadcast loops across swarms.
type BridgeEnvelope struct {
	OriginOrgID string          `json:"origin_org_id"`
	Payload     json.RawMessage `json:"payload"`
}

type BridgeManager struct {
	mu          sync.RWMutex
	connections map[string]*websocket.Conn
	node        *CentrifugeNode
	topic       string
	localOrgID  string
}

func NewBridgeManager(node *CentrifugeNode, topic string, localOrgID string) *BridgeManager {
	return &BridgeManager{
		connections: make(map[string]*websocket.Conn),
		node:        node,
		topic:       topic,
		localOrgID:  localOrgID,
	}
}

func (bm *BridgeManager) Connect(ctx context.Context, remoteURL string, remoteOrgID string, tlsConfig *tls.Config) error {
	dialer := websocket.Dialer{
		TLSClientConfig: tlsConfig,
	}

	header := http.Header{}
	header.Add("X-OHC-Org-ID", bm.localOrgID)

	var conn *websocket.Conn

	err := resilience.WithRetry(ctx, 5, 1*time.Second, func(retryCtx context.Context) error {
		var dialErr error
		conn, _, dialErr = dialer.DialContext(retryCtx, remoteURL, header)
		if dialErr != nil {
			slog.Warn("Failed to connect to bridge, retrying...", "url", remoteURL, "err", dialErr)
			return dialErr
		}
		return nil
	})

	if err != nil {
		return fmt.Errorf("failed to establish bridge connection after retries: %w", err)
	}

	bm.mu.Lock()
	bm.connections[remoteOrgID] = conn
	bm.mu.Unlock()

	telemetry.RecordBridgeStatus(ctx, 1)

	// Start reading from the remote connection
	go bm.readLoop(ctx, remoteOrgID, conn)

	// Start forwarding local events to the remote connection
	go bm.forwardLoop(ctx, remoteOrgID, conn)

	return nil
}

func (bm *BridgeManager) readLoop(ctx context.Context, remoteOrgID string, conn *websocket.Conn) {
	defer func() {
		conn.Close()
		bm.mu.Lock()
		delete(bm.connections, remoteOrgID)
		bm.mu.Unlock()
		telemetry.RecordBridgeStatus(ctx, -1)
	}()

	for {
		select {
		case <-ctx.Done():
			return
		default:
		}

		_, message, err := conn.ReadMessage()
		if err != nil {
			slog.Error("Bridge read error", "remoteOrgID", remoteOrgID, "err", err)
			return
		}

		telemetry.RecordBridgeMessageReceived(ctx)

		// Check if it's an envelope
		var envelope BridgeEnvelope
		if err := json.Unmarshal(message, &envelope); err == nil && envelope.OriginOrgID != "" {
			if envelope.OriginOrgID == bm.localOrgID {
				// It's a reflection of our own message, drop it to prevent loop
				continue
			}
			// It's a bridged message from elsewhere, we process the payload locally
			// But to prevent loops when rebroadcasting, we must re-publish it so that it gets processed
			// by local agents but is recognized by our forwardLoop as not originating here.
			// Actually, the simplest way is to broadcast the ENVELOPE to the local mesh.
			// Local agents need to unwrap it, or we broadcast the payload but our forwardLoop tracks seen messages.

			// To keep it simple and fulfill the instruction:
			// "Handle inbound events from the remote connection by re-broadcasting them to the local mesh."
			if bm.node != nil && bm.node.meshTransport != nil {
				// We re-broadcast the raw enveloped message. The forwardLoop will see it,
				// check the OriginOrgID, and realize it's not local, thus skipping forwarding it back.
				_ = bm.node.meshTransport.BroadcastMeshEvent(ctx, bm.topic, message)
			}
		} else {
			// It's a raw message that didn't have an envelope.
			// We wrap it in an envelope pretending it came from remoteOrgID to avoid loops if we re-broadcast.
			env := BridgeEnvelope{
				OriginOrgID: remoteOrgID,
				Payload:     message,
			}
			envBytes, _ := json.Marshal(env)
			if bm.node != nil && bm.node.meshTransport != nil {
				_ = bm.node.meshTransport.BroadcastMeshEvent(ctx, bm.topic, envBytes)
			}
		}
	}
}

func (bm *BridgeManager) forwardLoop(ctx context.Context, remoteOrgID string, conn *websocket.Conn) {
	if bm.node == nil || bm.node.meshTransport == nil {
		return
	}

	eventsCh, err := bm.node.meshTransport.SubscribeMeshEvents(ctx, bm.topic)
	if err != nil {
		slog.Error("Bridge failed to subscribe to local mesh events", "topic", bm.topic, "err", err)
		return
	}

	for {
		select {
		case <-ctx.Done():
			return
		case msg, ok := <-eventsCh:
			if !ok {
				return
			}

			// Check if the message is already enveloped
			var envelope BridgeEnvelope
			if err := json.Unmarshal(msg, &envelope); err == nil && envelope.OriginOrgID != "" {
				if envelope.OriginOrgID != bm.localOrgID {
					// This message originated from another swarm, do not forward it back to prevent loops
					continue
				}
				// If it originated from us, we can forward it.
				err := conn.WriteMessage(websocket.TextMessage, msg)
				if err != nil {
					slog.Error("Bridge failed to forward enveloped message", "remoteOrgID", remoteOrgID, "err", err)
					return
				}
				telemetry.RecordBridgeMessageSent(ctx)
			} else {
				// It's a raw local message. Wrap it in an envelope indicating it originated from us.
				env := BridgeEnvelope{
					OriginOrgID: bm.localOrgID,
					Payload:     msg,
				}
				envBytes, _ := json.Marshal(env)
				err := conn.WriteMessage(websocket.TextMessage, envBytes)
				if err != nil {
					slog.Error("Bridge failed to forward message", "remoteOrgID", remoteOrgID, "err", err)
					return
				}
				telemetry.RecordBridgeMessageSent(ctx)
			}
		}
	}
}

func (bm *BridgeManager) Status() map[string]string {
	bm.mu.RLock()
	defer bm.mu.RUnlock()

	status := make(map[string]string)
	for orgID := range bm.connections {
		status[orgID] = "ACTIVE"
	}
	return status
}
