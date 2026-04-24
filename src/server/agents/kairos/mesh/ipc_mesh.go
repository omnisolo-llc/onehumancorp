package mesh

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/google/uuid"
)

type ipcSubscription struct {
	mesh       *IPCMesh
	topic      string
	subscriber string
	handler    func(msg []byte)
	ctx        context.Context
	cancel     context.CancelFunc
}

func (s *ipcSubscription) Close() error {
	s.cancel()
	s.mesh.unsubscribe(s.topic, s)
	return nil
}

type ipcLockInfo struct {
	expiry time.Time
	token  string
}

type IPCMesh struct {
	baseDir     string
	mu          sync.RWMutex
	subscribers map[string]map[*ipcSubscription]struct{}
	locks       sync.Mutex
	activeLocks map[string]ipcLockInfo
	seenMu      sync.RWMutex
	seen        map[string]map[string]struct{} // subscriber ID -> msg filename
	presences   map[string]AgentPresence
}

func NewIPCMesh() *IPCMesh {
	baseDir := filepath.Join(os.TempDir(), "ohc_ipc_orch_mesh")
	if err := os.MkdirAll(baseDir, 0777); err != nil {
		fmt.Fprintf(os.Stderr, "failed to create IPC mesh base dir: %v\n", err)
	}

	return &IPCMesh{
		baseDir:     baseDir,
		subscribers: make(map[string]map[*ipcSubscription]struct{}),
		activeLocks: make(map[string]ipcLockInfo),
		seen:        make(map[string]map[string]struct{}),
		presences:   make(map[string]AgentPresence),
	}
}

func (m *IPCMesh) getTopicDir(topic string) string {
	safeTopic := strings.ReplaceAll(topic, "/", "_")
	safeTopic = strings.ReplaceAll(safeTopic, ":", "_")
	dir := filepath.Join(m.baseDir, safeTopic)
	os.MkdirAll(dir, 0777)
	return dir
}

func (m *IPCMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	dir := m.getTopicDir(topic)
	msgID := uuid.New().String()

	tmpFile := filepath.Join(dir, msgID+".tmp")
	if err := os.WriteFile(tmpFile, payload, 0666); err != nil {
		return err
	}

	msgFile := filepath.Join(dir, msgID+".msg")
	return os.Rename(tmpFile, msgFile)
}

func (m *IPCMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	subCtx, cancel := context.WithCancel(ctx)
	subscriberID := uuid.New().String()

	sub := &ipcSubscription{
		mesh:       m,
		topic:      topic,
		subscriber: subscriberID,
		handler:    handler,
		ctx:        subCtx,
		cancel:     cancel,
	}

	m.seenMu.Lock()
	if m.seen[subscriberID] == nil {
		m.seen[subscriberID] = make(map[string]struct{})
	}
	m.seenMu.Unlock()

	m.mu.Lock()
	if m.subscribers[topic] == nil {
		m.subscribers[topic] = make(map[*ipcSubscription]struct{})
	}
	m.subscribers[topic][sub] = struct{}{}
	m.mu.Unlock()

	dir := m.getTopicDir(topic)

	entries, _ := os.ReadDir(dir)
	m.seenMu.Lock()
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".msg") {
			m.seen[subscriberID][entry.Name()] = struct{}{}
		}
	}
	m.seenMu.Unlock()

	go m.pollLoop(subCtx, sub, dir, subscriberID)

	return sub, nil
}

func (m *IPCMesh) pollLoop(ctx context.Context, sub *ipcSubscription, dir string, subscriberID string) {
	ticker := time.NewTicker(100 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			entries, err := os.ReadDir(dir)
			if err != nil {
				continue
			}

			var newMsgs []string

			m.seenMu.RLock()
			for _, entry := range entries {
				if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".msg") {
					if _, ok := m.seen[subscriberID][entry.Name()]; !ok {
						newMsgs = append(newMsgs, entry.Name())
					}
				}
			}
			m.seenMu.RUnlock()

			for _, msgFile := range newMsgs {
				m.seenMu.Lock()
				m.seen[subscriberID][msgFile] = struct{}{}
				m.seenMu.Unlock()

				path := filepath.Join(dir, msgFile)
				data, err := os.ReadFile(path)
				if err == nil {
					sub.handler(data)
				}
			}
		}
	}
}

func (m *IPCMesh) unsubscribe(topic string, sub *ipcSubscription) {
	m.mu.Lock()
	if subs, ok := m.subscribers[topic]; ok {
		delete(subs, sub)
		if len(subs) == 0 {
			delete(m.subscribers, topic)
		}
	}
	m.mu.Unlock()

	m.seenMu.Lock()
	delete(m.seen, sub.subscriber)
	m.seenMu.Unlock()
}

func generateTokenIPC() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b)
}

func (m *IPCMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.locks.Lock()
	defer m.locks.Unlock()

	now := time.Now()
	if info, ok := m.activeLocks[key]; ok {
		if now.Before(info.expiry) {
			return false, nil // Lock is held
		}
	}

	token := generateTokenIPC()
	m.activeLocks[key] = ipcLockInfo{
		expiry: now.Add(ttl),
		token:  token,
	}
	return true, nil
}

func (m *IPCMesh) ReleaseLock(ctx context.Context, key string) error {
	m.locks.Lock()
	defer m.locks.Unlock()

	_, ok := m.activeLocks[key]
	if !ok {
		return errors.New("lock not found or expired")
	}

	delete(m.activeLocks, key)
	return nil
}

func (m *IPCMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.presences[agentID] = AgentPresence{AgentID: agentID, Status: status}
	return nil
}

func (m *IPCMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var agents []AgentPresence
	for _, p := range m.presences {
		agents = append(agents, p)
	}
	return agents, nil
}

func (m *IPCMesh) Cleanup() {
	// Utility for testing
	os.RemoveAll(m.baseDir)
}
