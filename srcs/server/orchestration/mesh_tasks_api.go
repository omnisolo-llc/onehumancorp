package orchestration

import (
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"

	"github.com/gorilla/websocket"
)

// TaskMeshHandler handles websocket connections specifically for the tasks stream.
func (tm *TeammateMesh) TaskMeshHandler(w http.ResponseWriter, r *http.Request) {
	// The tasks stream is essentially a global "swarm:tasks:updates" channel
	roomID := "swarm:tasks:updates"

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		slog.Error("task mesh: upgrade error", "err", err)
		return
	}

	msgChan := make(chan []byte, 256)
	tm.subscribe(roomID, conn, msgChan)
	defer tm.unsubscribe(roomID, conn)

	ctx := r.Context()

	// Start a single write goroutine
	go func() {
		defer conn.Close()
		for {
			select {
			case <-ctx.Done():
				return
			case payload, ok := <-msgChan:
				if !ok {
					return
				}
				_ = conn.WriteMessage(websocket.TextMessage, payload)
			}
		}
	}()

	// If cloud, subscribe to redis channel
	if tm.isCloud && tm.redisClient != nil {
		pubsub := tm.redisClient.Subscribe(ctx, roomID)
		defer pubsub.Close()

		go func() {
			ch := pubsub.Channel()
			for {
				select {
				case <-ctx.Done():
					return
				case msg := <-ch:
					select {
					case msgChan <- []byte(msg.Payload):
					default:
					}
				}
			}
		}()
	}

	for {
		_, payload, err := conn.ReadMessage()
		if err != nil {
			break
		}

		// Validate it as a MeshMessage or Task update payload
		var msg map[string]interface{}
		if err := json.Unmarshal(payload, &msg); err != nil {
			continue
		}

		// For task updates, we just re-publish them
		broadcastPayload, _ := json.Marshal(msg)
		tm.Publish(ctx, roomID, string(broadcastPayload))
	}
}

// BroadcastTaskUpdate sends a state change over the Teammate Mesh for a task.
func (tm *TeammateMesh) BroadcastTaskUpdate(ctx context.Context, task *SharedTask) error {
	payload, err := json.Marshal(map[string]interface{}{
		"type": "TASK_BROADCAST",
		"task": task,
	})
	if err != nil {
		return fmt.Errorf("failed to marshal task update: %w", err)
	}

	return tm.Publish(ctx, "swarm:tasks:updates", string(payload))
}
