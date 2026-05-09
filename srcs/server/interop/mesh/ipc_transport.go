package mesh

import (
	"context"
	"database/sql"
	"fmt"
	"sync"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"onehumancorp/srcs/server/pb"
)

type IpcTransport struct {
	db   *sql.DB
	subs map[string][]func(data []byte)
	mu   sync.RWMutex
}

func NewIpcTransport(dbPath string) (*IpcTransport, error) {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, fmt.Errorf("failed to open sqlite db: %w", err)
	}

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS bus_checkpoints (
			subscriber_id TEXT PRIMARY KEY,
			last_id INTEGER NOT NULL
		);
		CREATE TABLE IF NOT EXISTS bus_messages (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			topic TEXT NOT NULL,
			payload BLOB NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS bus_locks (
			resource TEXT PRIMARY KEY,
			owner TEXT NOT NULL,
			expires_at INTEGER NOT NULL
		);
	`)
	if err != nil {
		return nil, fmt.Errorf("failed to init schema: %w", err)
	}

	transport := &IpcTransport{
		db:   db,
		subs: make(map[string][]func(data []byte)),
	}

	go transport.startWorker()

	return transport, nil
}

func (t *IpcTransport) Publish(ctx context.Context, channel string, data []byte) error {
	var retries int
	for {
		_, err := t.db.ExecContext(ctx, "INSERT INTO bus_messages (topic, payload) VALUES (?, ?)", channel, data)
		if err == nil {
			return nil
		}
		if retries >= 3 {
			return fmt.Errorf("failed to publish to sqlite ipc after retries: %w", err)
		}
		retries++
		time.Sleep(time.Duration(100*retries) * time.Millisecond)
	}
}

func (t *IpcTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	t.mu.Lock()
	t.subs[channel] = append(t.subs[channel], handler)
	t.mu.Unlock()

	go func() {
		<-ctx.Done()
		// Omitted cleanup for simplicity
	}()
	return nil
}

func (t *IpcTransport) startWorker() {
	subscriberID := "standalone_node_go"
	var lastID int64

	err := t.db.QueryRow("SELECT last_id FROM bus_checkpoints WHERE subscriber_id = ?", subscriberID).Scan(&lastID)
	if err != nil && err != sql.ErrNoRows {
		fmt.Printf("Worker init error: %v\n", err)
	}

	for {
		rows, err := t.db.Query("SELECT id, topic, payload FROM bus_messages WHERE id > ? ORDER BY id ASC", lastID)
		if err != nil {
			time.Sleep(10 * time.Millisecond)
			continue
		}

		var maxID int64 = lastID
		var hasResults bool

		for rows.Next() {
			hasResults = true
			var id int64
			var topic string
			var payload []byte
			if err := rows.Scan(&id, &topic, &payload); err != nil {
				continue
			}

			maxID = id

			t.mu.RLock()
			handlers := t.subs[topic]
			t.mu.RUnlock()

			for _, h := range handlers {
				go h(payload)
			}
		}
		rows.Close()

		if hasResults {
			lastID = maxID
			t.db.Exec(`
				INSERT INTO bus_checkpoints (subscriber_id, last_id)
				VALUES (?, ?)
				ON CONFLICT(subscriber_id) DO UPDATE SET last_id = excluded.last_id`,
				subscriberID, lastID)
		}

		time.Sleep(10 * time.Millisecond)
	}
}

func (t *IpcTransport) AcquireLock(ctx context.Context, resource, owner string, ttlSeconds int) (bool, error) {
	expiresAt := time.Now().Unix() + int64(ttlSeconds)

	// Cleanup expired locks
	t.db.ExecContext(ctx, "DELETE FROM bus_locks WHERE expires_at <= ?", time.Now().Unix())

	res, err := t.db.ExecContext(ctx, "INSERT OR IGNORE INTO bus_locks (resource, owner, expires_at) VALUES (?, ?, ?)", resource, owner, expiresAt)
	if err != nil {
		return false, err
	}

	affected, _ := res.RowsAffected()
	if affected > 0 {
		return true, nil
	}

	// Check if already owner (for renewal)
	var currentOwner string
	err = t.db.QueryRowContext(ctx, "SELECT owner FROM bus_locks WHERE resource = ?", resource).Scan(&currentOwner)
	if err == nil && currentOwner == owner {
		t.db.ExecContext(ctx, "UPDATE bus_locks SET expires_at = ? WHERE resource = ?", expiresAt, resource)
		return true, nil
	}

	return false, nil
}

func (t *IpcTransport) ReleaseLock(ctx context.Context, resource, owner string) error {
	_, err := t.db.ExecContext(ctx, "DELETE FROM bus_locks WHERE resource = ? AND owner = ?", resource, owner)
	return err
}

func (t *IpcTransport) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
	return nil
}

func (t *IpcTransport) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	return nil, nil
}

func (t *IpcTransport) StartHeartbeat(ctx context.Context, agent pb.Agent) {}
