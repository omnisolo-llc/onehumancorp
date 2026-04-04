package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/gorilla/websocket"
)

// HandleMeshDirect handles sending a direct message to a specific agent's mailbox
func (s *Hub) HandleMeshDirect(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		AgentID string `json:"agent_id"`
		Payload string `json:"payload"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.AgentID == "" {
		http.Error(w, "missing agent_id", http.StatusBadRequest)
		return
	}

	// For cloud, use Redis directly or the hub
	// Since Teammate Mesh has its own interfaces, we'll route it via hub Publish
	msg := Message{
		ID:         fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent:  "system",
		ToAgent:    req.AgentID,
		Type:       "mesh:direct",
		Content:    req.Payload,
		OccurredAt: time.Now(),
	}

	if err := s.Publish(msg); err != nil {
		http.Error(w, "failed to send message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

// HandleMeshMailbox handles polling or websocket for agent mailbox
func (s *Hub) HandleMeshMailbox(w http.ResponseWriter, r *http.Request) {
	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "missing agent_id", http.StatusBadRequest)
		return
	}

	// If it's a websocket upgrade request
	if websocket.IsWebSocketUpgrade(r) {
		// Use the legacy teammate mesh for WS if available
		// The `LegacyTeammateMesh` has a HandleWebSocket method that takes roomID
		// Since we need to subscribe to the agent's mailbox

		// Wait, actually `s.Subscribe(agentID)` gives a channel of `struct{}` to trigger polling
		// For a real WS mailbox:
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}
		defer conn.Close()

		ch, unsubscribe := s.Subscribe(agentID)
		defer unsubscribe()

		ctx, cancel := context.WithCancel(r.Context())
		defer cancel()

		go func() {
			for {
				select {
				case <-ctx.Done():
					return
				case <-ch:
					// Drain inbox and send
					msgs := s.Inbox(agentID)
					for _, m := range msgs {
						b, _ := json.Marshal(m)
						_ = conn.WriteMessage(websocket.TextMessage, b)
					}
					// we can't use putMessageSlice here as it's not exported easily, or it is?
					// Wait, we don't have access to putMessageSlice here since it's internal.
				}
			}
		}()

		for {
			_, _, err := conn.ReadMessage()
			if err != nil {
				break
			}
		}
		return
	}

	// Poll: just drain the inbox and return JSON
	msgs := s.Inbox(agentID)
	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(msgs); err != nil {
		http.Error(w, "failed to encode response", http.StatusInternalServerError)
	}
}
