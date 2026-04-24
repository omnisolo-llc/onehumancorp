package interop

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// TeammateMesh provides the interface for agents to publish and subscribe
// to real-time communication messages across the swarm.
type TeammateMesh interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string) (<-chan []byte, error)
}

// NewTeammateMesh returns a new TeammateMesh depending on the execution mode.
// If REDIS_URL is present and OHC_STANDALONE is not true, it returns a cloud mesh.
// Otherwise, it returns an IPC mesh based on the filesystem for true cross-process standalone communication.
func NewTeammateMesh() (TeammateMesh, error) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL != "" && os.Getenv("OHC_STANDALONE") != "true" {
		opts, err := rueidis.ParseURL(redisURL)
		if err != nil {
			return nil, fmt.Errorf("failed to parse REDIS_URL: %w", err)
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			return nil, fmt.Errorf("failed to connect to redis: %w", err)
		}
		slog.Info("TeammateMesh initialized in Cloud mode (Redis)")
		return &cloudMesh{client: c}, nil
	}

	slog.Info("TeammateMesh initialized in Standalone mode (IPC Filesystem)")

	// Create base directory for IPC
	ipcDir := filepath.Join(os.TempDir(), "ohc_mesh_ipc")
	if err := os.MkdirAll(ipcDir, 0700); err != nil {
		return nil, fmt.Errorf("failed to initialize IPC mesh directory: %w", err)
	}

	m := &ipcMesh{
		baseDir:  ipcDir,
		channels: make(map[string][]chan []byte),
		pollInterval: 100 * time.Millisecond,
		subSeen: make(map[string]map[string]bool),
	}
	m.startCleanupTask()
	return m, nil
}

// NewTeammateMeshWithClient returns a new TeammateMesh using an existing rueidis client.
// Useful for dependency injection in testing or sharing clients.
func NewTeammateMeshWithClient(c rueidis.Client) TeammateMesh {
	if c != nil {
		return &cloudMesh{client: c}
	}
	ipcDir := filepath.Join(os.TempDir(), "ohc_mesh_ipc_test_"+uuid.New().String())
	os.MkdirAll(ipcDir, 0700)

	m := &ipcMesh{
		baseDir: ipcDir,
		channels: make(map[string][]chan []byte),
		pollInterval: 10 * time.Millisecond,
		subSeen: make(map[string]map[string]bool),
	}
	// In test mode we might not run cleanup or we might run it more frequently.
	// For simplicity, we just won't run the global cleanup ticker here. Tests run fast.
	return m
}

// ipcMesh provides a local cross-process pub/sub using the filesystem.
type ipcMesh struct {
	mu           sync.RWMutex
	baseDir      string
	channels     map[string][]chan []byte
	pollInterval time.Duration
	subSeen      map[string]map[string]bool // subID -> messageID -> seen
}

func (m *ipcMesh) startCleanupTask() {
	go func() {
		ticker := time.NewTicker(5 * time.Minute)
		defer ticker.Stop()

		for {
			<-ticker.C

			// Clean up files older than 10 minutes from disk
			cutoff := time.Now().Add(-10 * time.Minute).Format("20060102150405.000000")

			m.mu.RLock()
			channelNames := make([]string, 0, len(m.channels))
			for ch := range m.channels {
				channelNames = append(channelNames, ch)
			}
			m.mu.RUnlock()

			for _, channel := range channelNames {
				safeChannel := strings.ReplaceAll(channel, "/", "_")
				channelDir := filepath.Join(m.baseDir, safeChannel)
				entries, err := os.ReadDir(channelDir)
				if err == nil {
					for _, entry := range entries {
						if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".msg") {
							msgTimeStr := strings.Split(entry.Name(), "_")[0]
							if msgTimeStr < cutoff {
								os.Remove(filepath.Join(channelDir, entry.Name()))
							}
						}
					}
				}
			}

			// Clean up in-memory cache for all subscribers of messages older than 10 mins
			// Even if file was deleted by someone else, we clean our memory cache based on parsing the message ID timestamp
			m.mu.Lock()
			for subID, seenMsgs := range m.subSeen {
				for msgID := range seenMsgs {
					msgTimeStr := strings.Split(msgID, "_")[0]
					if msgTimeStr < cutoff {
						delete(seenMsgs, msgID)
					}
				}
				// if subscriber is completely gone and cache is empty, we don't need to keep it
				if len(seenMsgs) == 0 {
					// We'd ideally need a way to know if sub is still active, but if it's empty, next seen will recreate.
					// Actually, better to just leave it or let unsubscribe handle deletion.
				}
			}
			m.mu.Unlock()
		}
	}()
}

func (m *ipcMesh) Publish(ctx context.Context, channel string, data []byte) error {
	if meshMessagesPublished != nil {
		meshMessagesPublished.Add(ctx, 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "standalone")))
	}

	safeChannel := strings.ReplaceAll(channel, "/", "_")
	channelDir := filepath.Join(m.baseDir, safeChannel)

	if err := os.MkdirAll(channelDir, 0700); err != nil {
		return err
	}

	msgID := time.Now().Format("20060102150405.000000") + "_" + uuid.New().String()
	tmpFile := filepath.Join(channelDir, msgID+".tmp")
	finalFile := filepath.Join(channelDir, msgID+".msg")

	if err := os.WriteFile(tmpFile, data, 0600); err != nil {
		return err
	}

	return os.Rename(tmpFile, finalFile)
}

func (m *ipcMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	out := make(chan []byte, 100)
	subID := uuid.New().String()

	m.mu.Lock()
	m.channels[channel] = append(m.channels[channel], out)
	m.subSeen[subID] = make(map[string]bool)
	m.mu.Unlock()

	safeChannel := strings.ReplaceAll(channel, "/", "_")
	channelDir := filepath.Join(m.baseDir, safeChannel)
	os.MkdirAll(channelDir, 0700)

	// Pre-populate seen for this new subscriber with existing messages so we only process new ones
	entries, _ := os.ReadDir(channelDir)
	m.mu.Lock()
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".msg") {
			m.subSeen[subID][entry.Name()] = true
		}
	}
	m.mu.Unlock()

	go func() {
		defer func() {
			m.mu.Lock()
			subs := m.channels[channel]
			for i, sub := range subs {
				if sub == out {
					m.channels[channel] = append(subs[:i], subs[i+1:]...)
					break
				}
			}
			delete(m.subSeen, subID)
			m.mu.Unlock()
			close(out)
		}()

		ticker := time.NewTicker(m.pollInterval)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				entries, err := os.ReadDir(channelDir)
				if err != nil {
					continue
				}

				for _, entry := range entries {
					if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".msg") {
						m.mu.RLock()
						seen := m.subSeen[subID][entry.Name()]
						m.mu.RUnlock()

						if !seen {
							msgPath := filepath.Join(channelDir, entry.Name())
							data, err := os.ReadFile(msgPath)
							if err == nil {
								select {
								case out <- data:
									m.mu.Lock()
									m.subSeen[subID][entry.Name()] = true
									m.mu.Unlock()
								case <-ctx.Done():
									return
								}
							}
						}
					}
				}
			}
		}
	}()

	// Intercept the output channel to track metrics before sending to consumer
	meteredOut := make(chan []byte, 100)
	go func() {
		defer close(meteredOut)
		for {
			select {
			case msg, ok := <-out:
				if !ok {
					return
				}
				if meshMessagesReceived != nil {
					meshMessagesReceived.Add(context.Background(), 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "standalone")))
				}
				select {
				case meteredOut <- msg:
				case <-ctx.Done():
					return
				}
			case <-ctx.Done():
				return
			}
		}
	}()

	return meteredOut, nil
}

// cloudMesh provides a Redis pub/sub backed mesh using rueidis.
type cloudMesh struct {
	client rueidis.Client
}

func (c *cloudMesh) Publish(ctx context.Context, channel string, data []byte) error {
	if meshMessagesPublished != nil {
		meshMessagesPublished.Add(ctx, 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "cloud")))
	}
	cmd := c.client.B().Publish().Channel(channel).Message(string(data)).Build()
	return c.client.Do(ctx, cmd).Error()
}

func (c *cloudMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	out := make(chan []byte, 100)

	go func() {
		defer close(out)

		err := c.client.Receive(ctx, c.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			if meshMessagesReceived != nil {
				meshMessagesReceived.Add(context.Background(), 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "cloud")))
			}
			select {
			case out <- []byte(msg.Message):
			case <-ctx.Done():
			}
		})

		if err != nil && err != context.Canceled {
			// Suppress expected transient errors if context is done or redis connection is closed during shutdown.
			slog.Error("Redis subscription failed", "channel", channel, "error", err)
		}
	}()

	return out, nil
}
