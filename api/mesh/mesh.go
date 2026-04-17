package mesh

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
	"sync"

	"github.com/gorilla/websocket"
	"github.com/redis/go-redis/v9"
)

// MeshMessage represents a payload sent over the Teammate Mesh.
type MeshMessage struct {
	SenderID string          `json:"sender_id"`
	Topic    string          `json:"topic"`
	Payload  json.RawMessage `json:"payload"`
}

// TeammateMesh is the Redis Pub/Sub powered communication layer.
type TeammateMesh struct {
	client      *redis.Client
	mu          sync.RWMutex
	subscribers map[string][]chan MeshMessage
	mailboxes   map[string][]MeshMessage
}

// NewTeammateMesh initializes a new TeammateMesh.
func NewTeammateMesh(client *redis.Client) *TeammateMesh {
	return &TeammateMesh{
		client:      client,
		subscribers: make(map[string][]chan MeshMessage),
		mailboxes:   make(map[string][]MeshMessage),
	}
}

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		// Allow same-origin and localhost for Standalone mode
		origin := r.Header.Get("Origin")
		if origin == "" {
			return true
		}
		if strings.HasPrefix(origin, "http://localhost") || strings.HasPrefix(origin, "https://localhost") {
			return true
		}
		// In production Cloud-Native mode, we should ideally check against the platform domain
		return true // Keeping true for now to avoid breaking the Swarm across K8s namespaces
	},
}

// HandlePublish handles POST /mesh/publish requests.
func (m *TeammateMesh) HandlePublish(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var msg MeshMessage
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	if err := m.Publish(r.Context(), msg); err != nil {
		http.Error(w, fmt.Sprintf("Failed to publish: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "published"})
}

// HandleSubscribe handles GET /mesh/subscribe requests and upgrades them to WebSockets.
func (m *TeammateMesh) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	topic := r.URL.Query().Get("topic")
	if topic == "" {
		http.Error(w, "Missing topic parameter", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("Failed to upgrade to websocket: %v", err)
		return
	}
	defer conn.Close()

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	m.Subscribe(ctx, topic, func(msg MeshMessage) {
		if err := conn.WriteJSON(msg); err != nil {
			log.Printf("Failed to write to websocket: %v", err)
			cancel()
		}
	})

	// Keep the connection open until it's closed by the client or server
	for {
		if _, _, err := conn.ReadMessage(); err != nil {
			break
		}
	}
}

// Publish sends a message to the specified topic.
func (m *TeammateMesh) Publish(ctx context.Context, msg MeshMessage) error {
	if m.client != nil {
		data, err := json.Marshal(msg)
		if err != nil {
			return err
		}
		return m.client.Publish(ctx, msg.Topic, data).Err()
	}

	m.mu.RLock()
	defer m.mu.RUnlock()
	if subs, ok := m.subscribers[msg.Topic]; ok {
		for _, sub := range subs {
			select {
			case sub <- msg:
			default:
				// Skip slow subscribers
			}
		}
	}
	return nil
}

// Subscribe listens to a topic and handles messages.
func (m *TeammateMesh) Subscribe(ctx context.Context, topic string, handler func(MeshMessage)) {
	if m.client != nil {
		sub := m.client.Subscribe(ctx, topic)
		ch := sub.Channel()

		go func() {
			defer sub.Close()
			for {
				select {
				case <-ctx.Done():
					return
				case redisMsg := <-ch:
					var msg MeshMessage
					if err := json.Unmarshal([]byte(redisMsg.Payload), &msg); err != nil {
						log.Printf("Failed to unmarshal mesh message: %v", err)
						continue
					}
					handler(msg)
				}
			}
		}()
		return
	}

	// Local fallback for standalone mode
	ch := make(chan MeshMessage, 100)
	m.mu.Lock()
	m.subscribers[topic] = append(m.subscribers[topic], ch)
	m.mu.Unlock()

	go func() {
		defer func() {
			m.mu.Lock()
			subs := m.subscribers[topic]
			for i, s := range subs {
				if s == ch {
					m.subscribers[topic] = append(subs[:i], subs[i+1:]...)
					break
				}
			}
			m.mu.Unlock()
		}()

		for {
			select {
			case <-ctx.Done():
				return
			case msg := <-ch:
				handler(msg)
			}
		}
	}()
}


// SendToMailbox stores a message in the agent's mailbox.
func (m *TeammateMesh) SendToMailbox(ctx context.Context, agentID string, msg MeshMessage) error {
	if m.client != nil {
		data, err := json.Marshal(msg)
		if err != nil {
			return err
		}
		key := "mesh:mailbox:" + agentID
		// Push message to the right side of the list
		return m.client.RPush(ctx, key, data).Err()
	}

	// Local fallback for standalone mode
	m.mu.Lock()
	defer m.mu.Unlock()
	m.mailboxes[agentID] = append(m.mailboxes[agentID], msg)
	return nil
}

// GetMailbox retrieves all messages from an agent's mailbox and clears it.
func (m *TeammateMesh) GetMailbox(ctx context.Context, agentID string) ([]MeshMessage, error) {
	if m.client != nil {
		key := "mesh:mailbox:" + agentID

		// Use Lua script to atomically fetch all elements and delete the list
		script := `
			local elements = redis.call('LRANGE', KEYS[1], 0, -1)
			if #elements > 0 then
				redis.call('DEL', KEYS[1])
			end
			return elements
		`

		result, err := m.client.Eval(ctx, script, []string{key}).Result()
		if err != nil && err != redis.Nil {
			return nil, err
		}

		messages := []MeshMessage{} // Ensure empty slice instead of nil

		if result != nil {
			data := result.([]interface{})
			for _, d := range data {
				var msg MeshMessage
				if err := json.Unmarshal([]byte(d.(string)), &msg); err != nil {
					continue
				}
				messages = append(messages, msg)
			}
		}

		return messages, nil
	}

	// Local fallback for standalone mode
	m.mu.Lock()
	defer m.mu.Unlock()
	messages := m.mailboxes[agentID]
	// Clear the mailbox
	m.mailboxes[agentID] = nil
	if messages == nil {
		return []MeshMessage{}, nil
	}
	return messages, nil
}

// HandleMailbox handles requests for the mailbox API.
// GET /mesh/mailbox?agent_id=<id> retrieves messages.
// POST /mesh/mailbox?agent_id=<id> sends a message to the mailbox.
func (m *TeammateMesh) HandleMailbox(w http.ResponseWriter, r *http.Request) {
	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "Missing agent_id parameter", http.StatusBadRequest)
		return
	}

	if r.Method == http.MethodGet {
		messages, err := m.GetMailbox(r.Context(), agentID)
		if err != nil {
			http.Error(w, fmt.Sprintf("Failed to get mailbox: %v", err), http.StatusInternalServerError)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{"messages": messages})
		return
	}

	if r.Method == http.MethodPost {
		var msg MeshMessage
		if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
			http.Error(w, "Bad request", http.StatusBadRequest)
			return
		}
		if err := m.SendToMailbox(r.Context(), agentID, msg); err != nil {
			http.Error(w, fmt.Sprintf("Failed to send to mailbox: %v", err), http.StatusInternalServerError)
			return
		}
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(map[string]string{"status": "sent to mailbox"})
		return
	}

	http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
}
