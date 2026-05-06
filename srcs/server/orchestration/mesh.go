package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"sync"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/redis/go-redis/v9"
)

// MeshHub defines the interface for the highly available realtime communication layer.
type MeshHub interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
}

type subscriber struct {
	id      int
	handler func(data []byte)
}

// LocalTeammateMesh implements MeshHub for standalone operation using Go channels.
type LocalTeammateMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]subscriber
	nextID      int
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh.
func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subscribers: make(map[string][]subscriber),
	}
}

// Publish sends data to all subscribers of the given channel concurrently.
func (m *LocalTeammateMesh) Publish(ctx context.Context, channel string, data []byte) error {
	m.mu.RLock()
	subs, ok := m.subscribers[channel]
	if !ok {
		m.mu.RUnlock()
		return nil
	}
	// Copy subs to avoid holding lock while dispatching
	subsCopy := make([]subscriber, len(subs))
	copy(subsCopy, subs)
	m.mu.RUnlock()

	dataCopy := make([]byte, len(data))
	copy(dataCopy, data)

	for _, sub := range subsCopy {
		go sub.handler(dataCopy)
	}

	return nil
}

// Subscribe registers a handler for the given channel. Unsubscribes when ctx is done.
func (m *LocalTeammateMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	m.mu.Lock()
	id := m.nextID
	m.nextID++
	m.subscribers[channel] = append(m.subscribers[channel], subscriber{id: id, handler: handler})
	m.mu.Unlock()

	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()
		subs := m.subscribers[channel]
		for i, sub := range subs {
			if sub.id == id {
				m.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
	}()

	return nil
}

// IpcTeammateMesh implements MeshHub using an SQLite database for cross-process standalone operation.
type IpcTeammateMesh struct {
	db *sql.DB
}

// NewIpcTeammateMesh creates a new IpcTeammateMesh and initializes the schema.
func NewIpcTeammateMesh(db *sql.DB) (*IpcTeammateMesh, error) {
	mesh := &IpcTeammateMesh{db: db}

	// Initialize schema to mirror Rust's IpcTransport
	_, err := db.Exec(`
		CREATE TABLE IF NOT EXISTS mesh_messages (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			topic TEXT NOT NULL,
			payload BLOB NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			msg_id TEXT
		);
		CREATE TABLE IF NOT EXISTS mesh_checkpoints (
			subscriber_id TEXT PRIMARY KEY,
			last_id INTEGER NOT NULL
		);
		CREATE TABLE IF NOT EXISTS mesh_locks (
			resource TEXT PRIMARY KEY,
			owner TEXT NOT NULL,
			expires_at DATETIME NOT NULL
		);
		CREATE TABLE IF NOT EXISTS mesh_presence (
			agent_id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			expires_at DATETIME NOT NULL
		);
	`)
	if err != nil {
		return nil, fmt.Errorf("failed to initialize schema: %w", err)
	}

	return mesh, nil
}

// Publish sends data by inserting it into the mesh_messages table.
func (m *IpcTeammateMesh) Publish(ctx context.Context, channel string, data []byte) error {
	_, err := m.db.ExecContext(ctx, "INSERT INTO mesh_messages (topic, payload) VALUES (?, ?)", channel, data)
	if err != nil {
		return fmt.Errorf("failed to publish message: %w", err)
	}
	return nil
}

// Subscribe registers a handler and spawns a polling goroutine.
func (m *IpcTeammateMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	subscriberID := fmt.Sprintf("go_server_node_%s", channel)

	// Get last processed ID
	var lastID int64
	err := m.db.QueryRowContext(ctx, "SELECT last_id FROM mesh_checkpoints WHERE subscriber_id = ?", subscriberID).Scan(&lastID)
	if err != nil && err != sql.ErrNoRows {
		return fmt.Errorf("failed to get last_id: %w", err)
	}

	go func() {
		ticker := time.NewTicker(50 * time.Millisecond)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				rows, err := m.db.QueryContext(ctx, "SELECT id, payload FROM mesh_messages WHERE id > ? AND topic = ? ORDER BY id ASC", lastID, channel)
				if err != nil {
					log.Printf("IpcTeammateMesh poll error: %v", err)
					continue
				}

				var maxID int64
				for rows.Next() {
					var id int64
					var payload []byte
					if err := rows.Scan(&id, &payload); err != nil {
						log.Printf("IpcTeammateMesh scan error: %v", err)
						continue
					}

					maxID = id
					// Execute handler in a goroutine to avoid blocking the poller
					go handler(payload)
				}
				rows.Close()

				if maxID > lastID {
					lastID = maxID
					_, err = m.db.ExecContext(ctx, "INSERT INTO mesh_checkpoints (subscriber_id, last_id) VALUES (?, ?) ON CONFLICT(subscriber_id) DO UPDATE SET last_id = excluded.last_id", subscriberID, lastID)
					if err != nil {
						log.Printf("IpcTeammateMesh checkpoint update error: %v", err)
					}
				}

				// Optional cleanup of old messages could be done here or in a separate background job.
				// We rely on Rust's IpcTransport for cleanup (it deletes messages older than 1 hour).
			}
		}
	}()

	return nil
}

// CentrifugeMesh implements MeshHub using go-redis for cloud-native setup.
type CentrifugeMesh struct {
	client *redis.Client
}

// NewCentrifugeMesh creates a new CentrifugeMesh.
func NewCentrifugeMesh(redisURL string) (*CentrifugeMesh, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, fmt.Errorf("failed to parse redis URL: %w", err)
	}

	client := redis.NewClient(opts)
	// Verify connection
	if err := client.Ping(context.Background()).Err(); err != nil {
		return nil, fmt.Errorf("failed to connect to redis: %w", err)
	}

	return &CentrifugeMesh{client: client}, nil
}

// Publish sends data via Redis Pub/Sub.
func (m *CentrifugeMesh) Publish(ctx context.Context, channel string, data []byte) error {
	err := m.client.Publish(ctx, channel, data).Err()
	if err != nil {
		return fmt.Errorf("failed to publish to redis: %w", err)
	}
	return nil
}

// Subscribe registers a handler and spawns a goroutine to listen for messages via Redis Pub/Sub.
func (m *CentrifugeMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	pubsub := m.client.Subscribe(ctx, channel)

	go func() {
		defer pubsub.Close()
		ch := pubsub.Channel()

		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				// Execute handler in a goroutine
				go handler([]byte(msg.Payload))
			}
		}
	}()

	return nil
}
