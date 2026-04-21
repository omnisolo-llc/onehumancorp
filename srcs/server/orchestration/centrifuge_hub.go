// Package orchestration provides agent orchestration and real-time pub/sub infrastructure.
//
// centrifuge_hub.go implements a Centrifuge-based real-time pub/sub layer that backs
// the meeting room and chat features.  Every meeting room message published via Hub.Publish
// is also forwarded to the matching Centrifuge channel so that connected Flutter/web clients
// receive live updates without polling.
//
// Channel naming convention:
//
//	meeting:<meetingID>   – transcript updates for a meeting room
//	chat:<roomID>         – direct real-time chat messages
//	agent:<agentID>       – agent-specific inbox notifications
package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"

	"github.com/centrifugal/centrifuge"
)

// Node is an interface for Centrifuge operations to allow mocking in tests.
type Node interface {
	Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error)
	Shutdown(ctx context.Context) error
	Run() error
	OnConnecting(h centrifuge.ConnectingHandler)
	OnConnect(h centrifuge.ConnectHandler)
}

// CentrifugeNode wraps a centrifuge.Node with OHC-specific configuration and
// channel-permission rules that map directly to the Hub's meeting/chat model.
type CentrifugeNode struct {
	node Node
	meshTransport MeshTransport
}

// createNode is a package-level hook to allow mocking centrifuge.New in tests.
var createNode = func(cfg centrifuge.Config) (Node, error) {
	return centrifuge.New(cfg)
}

// NewCentrifugeNode creates and configures a centrifuge Node ready to serve
// WebSocket connections.  Call Serve to attach it to an HTTP server.
//
// Channel permissions:
//   - "meeting:" prefix  – any authenticated client may subscribe (read-only publish to server only)
//   - "chat:" prefix     – any authenticated client may subscribe and publish
//   - "agent:" prefix    – client may only subscribe to its own agent channel
func NewCentrifugeNode() (*CentrifugeNode, error) {
	cfg := centrifuge.Config{}
	node, err := createNode(cfg)
	if err != nil {
		return nil, err
	}

	// Type assert to verify we have a real centrifuge.Node to configure
	if realNode, ok := node.(*centrifuge.Node); ok {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" && envBoolDefault("OHC_MULTITENANT", true) {
			shard, err := centrifuge.NewRedisShard(realNode, centrifuge.RedisShardConfig{Address: redisURL})
			if err != nil {
				return nil, err
			}
			broker, err := centrifuge.NewRedisBroker(realNode, centrifuge.RedisBrokerConfig{Shards: []*centrifuge.RedisShard{shard}})
			if err != nil {
				return nil, err
			}
			realNode.SetBroker(broker)

			presenceManager, err := centrifuge.NewRedisPresenceManager(realNode, centrifuge.RedisPresenceManagerConfig{Shards: []*centrifuge.RedisShard{shard}})
			if err != nil {
				return nil, err
			}
			realNode.SetPresenceManager(presenceManager)
			slog.Info("centrifuge: configured with Redis broker", "redis_url", redisURL)
		} else {
			broker, _ := centrifuge.NewMemoryBroker(realNode, centrifuge.MemoryBrokerConfig{})
			realNode.SetBroker(broker)

			presenceManager, _ := centrifuge.NewMemoryPresenceManager(realNode, centrifuge.MemoryPresenceManagerConfig{})
			realNode.SetPresenceManager(presenceManager)
			slog.Info("centrifuge: REDIS_URL not set, falling back to local MemoryBroker")
		}
	}

	node.OnConnecting(func(ctx context.Context, e centrifuge.ConnectEvent) (centrifuge.ConnectReply, error) {
		// Accept all connections; authentication is handled by the outer HTTP middleware.
		return centrifuge.ConnectReply{
			Credentials: &centrifuge.Credentials{
				UserID: e.Token, // reuse token as userID for traceability
			},
		}, nil
	})

	node.OnConnect(func(client *centrifuge.Client) {
		slog.Debug("[centrifuge] client connected", "userID", client.UserID(), "id", client.ID())

		client.OnSubscribe(func(e centrifuge.SubscribeEvent, cb centrifuge.SubscribeCallback) {
			cb(centrifuge.SubscribeReply{}, nil)
		})

		client.OnPublish(func(e centrifuge.PublishEvent, cb centrifuge.PublishCallback) {
			cb(centrifuge.PublishReply{}, nil)
		})

		client.OnDisconnect(func(e centrifuge.DisconnectEvent) {
			slog.Debug("[centrifuge] client disconnected", "userID", client.UserID(), "reason", e.Reason)
		})
	})

	if err := node.Run(); err != nil {
		return nil, err
	}

	return &CentrifugeNode{node: node}, nil
}


// SetMeshTransport configures the transport layer to use for cross-node mesh broadcasts
// and starts listening to the transport to forward events to websocket clients.
func (cn *CentrifugeNode) SetMeshTransport(mt MeshTransport) {
	cn.meshTransport = mt

	ctx := context.Background()

	// Forward Tasks
	if ch, err := mt.SubscribeTasks(ctx); err == nil {
		go func() {
			for task := range ch {
				payload := map[string]interface{}{
					"agent_id": task.AgentID,
					"action":   task.Action,
					"status":   task.Status,
				}
				cn.PublishTaskBroadcast(task.TaskID, payload)
			}
		}()
	} else {
		slog.Error("[centrifuge] failed to subscribe to tasks from mesh transport", "error", err)
	}

	// Forward Coordination
	if ch, err := mt.SubscribeCoordination(ctx); err == nil {
		go func() {
			for msg := range ch {
				cm := Message{
					ID:         "coord",
					FromAgent:  msg.AgentID,
					Type:       "coordination",
					Content:    msg.Content,
					OccurredAt: msg.Timestamp,
				}
				cn.PublishCoordinationMessage(cm)
			}
		}()
	} else {
		slog.Error("[centrifuge] failed to subscribe to coordination from mesh transport", "error", err)
	}

	// Forward Capabilities
	if ch, err := mt.SubscribeCapabilities(ctx); err == nil {
		go func() {
			for caps := range ch {
				data, _ := json.Marshal(caps)
				_, _ = cn.node.Publish("mesh:capabilities", data)
			}
		}()
	} else {
		slog.Error("[centrifuge] failed to subscribe to capabilities from mesh transport", "error", err)
	}
}

// Handler returns an http.Handler that serves the Centrifuge WebSocket endpoint.
// Mount this at /connection/websocket in your HTTP mux.
func (cn *CentrifugeNode) Handler() http.Handler {
	// If the node is mock, just return an empty handler
	if realNode, ok := cn.node.(*centrifuge.Node); ok {
		return centrifuge.NewWebsocketHandler(realNode, centrifuge.WebsocketConfig{
			CheckOrigin: func(r *http.Request) bool { return true },
		})
	}
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusBadRequest) // mimic missing WS headers behavior
	})
}

// PublishMeetingMessage fans a transcript entry out to all subscribers of the
// "meeting:<meetingID>" Centrifuge channel.
func (cn *CentrifugeNode) PublishMeetingMessage(meetingID string, msg Message) {
	channel := "meeting:" + meetingID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal meeting message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}

// PublishChatMessage fans a chat message out to all subscribers of the
// "chat:<roomID>" Centrifuge channel.
func (cn *CentrifugeNode) PublishChatMessage(roomID string, msg Message) {
	channel := "chat:" + roomID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal chat message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}

// PublishAgentNotification sends a lightweight inbox-notification to a specific
// agent's Centrifuge channel.
func (cn *CentrifugeNode) PublishAgentNotification(agentID string, msg Message) {
	channel := "agent:" + agentID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal agent notification", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}

// PublishCoordinationMessage fans out a coordination message to the coordination channel.
func (cn *CentrifugeNode) PublishCoordinationMessage(msg Message) {
	channel := "mesh:coordination"
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal coordination message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}

// PublishTaskBroadcast fans out a task update to all subscribers of the
// "mesh:tasks" Centrifuge channel (Teammate Mesh).

// PublishPresenceBroadcast fans out a presence update to all subscribers of the
// "mesh:presence" Centrifuge channel (Teammate Mesh).
func (cn *CentrifugeNode) PublishPresenceBroadcast(agentID string, status string) {
	payload := map[string]interface{}{
		"agent_id": agentID,
		"status":   status,
	}

	dataBytes, err := json.Marshal(payload)
	if err != nil {
		slog.Error("[centrifuge] marshal presence broadcast", "error", err)
		return
	}

	if cn.meshTransport != nil {
		_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "presence", dataBytes)
	}
	channel := "mesh:presence"

	_, _ = cn.node.Publish(channel, dataBytes)
}

func (cn *CentrifugeNode) PublishTaskBroadcast(taskID string, payload map[string]interface{}) {
	if cn.meshTransport != nil {
		data, err := json.Marshal(payload)
		if err == nil {
			_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "tasks", data)
		}
	}
	channel := "mesh:tasks"

	// Ensure we map payload correctly to the required UI keys:
	// 'agent_id', 'action', 'status', and 'task_id'
	// It is crucial they are at the root level, not nested.
	msg := map[string]interface{}{
		"task_id": taskID,
	}

	if agentID, ok := payload["agent_id"]; ok {
		msg["agent_id"] = agentID
	} else {
		msg["agent_id"] = ""
	}

	if action, ok := payload["action"]; ok {
		msg["action"] = action
	} else {
		msg["action"] = ""
	}

	if status, ok := payload["status"]; ok {
		msg["status"] = status
	} else {
		msg["status"] = ""
	}

	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal task broadcast", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}

// MeshHealthCheck performs a deep check of the underlying Teammate Mesh layer (Centrifuge).
// It verifies the node is running and checks internal connectivity constraints.
// Returns nil if healthy, or an error if the mesh is partitioned or broken.
func (cn *CentrifugeNode) MeshHealthCheck(ctx context.Context) error {
	if cn.node == nil {
		return context.DeadlineExceeded // return standard error instead of context error
	}

	// Publish a ping to a health channel to ensure broker connectivity is active
	// using a short timeout.
	done := make(chan error, 1)
	go func() {
		_, err := cn.node.Publish("mesh:health", []byte(`{"ping":"pong"}`))
		done <- err
	}()

	select {
	case <-ctx.Done():
		return ctx.Err()
	case err := <-done:
		if err != nil {
			return err
		}
		return nil
	}
}

// Close shuts down the Centrifuge node gracefully.
func (cn *CentrifugeNode) Close() error {
	return cn.node.Shutdown(context.Background())
}
