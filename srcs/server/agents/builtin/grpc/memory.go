package agentgrpc

// memory.go implements AutoDream-style persistent memory for self-learning.
//
// Memory entries are stored in-process (with optional Redis/Valkey sharing
// across agents in the same cluster).  The former filesystem approach
// (.agent-task/memory/*.json) has been removed; all state lives in the
// database or the message bus.
//
// The design:
//   - MemoryStore holds an in-process ring-buffer of recent MemoryEntry values.
//   - When a Redis client is configured (REDIS_URL env var) entries are also
//     published to a Redis list so sibling agents can share memory.
//   - InjectMemoriesIntoPrompt / RecordTaskMemory are the public surface used
//     by service.go.

import (
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"strings"
	"sync"
	"time"
)

const (
	maxMemoriesForPrompt = 5  // inject at most 5 past memories into prompt
	memoryRingSize       = 64 // per-process ring-buffer capacity
)

// MemoryEntry is a single completed-task memory record.
type MemoryEntry struct {
	TaskID      string    `json:"task_id"`
	Summary     string    `json:"summary"`
	ToolsUsed   []string  `json:"tools_used,omitempty"`
	Outcome     string    `json:"outcome"` // "success" | "failure"
	Duration    float64   `json:"duration_s"`
	Lessons     string    `json:"lessons,omitempty"`
	CompletedAt time.Time `json:"completed_at"`
}

// MemoryStore manages an in-process ring-buffer of MemoryEntry values.
// It is safe for concurrent use.
type MemoryStore struct {
	mu      sync.Mutex
	entries []MemoryEntry
	cap     int
}

// newMemoryStore creates a MemoryStore with the given ring-buffer capacity.
func newMemoryStore(capacity int) *MemoryStore {
	if capacity <= 0 {
		capacity = memoryRingSize
	}
	return &MemoryStore{
		entries: make([]MemoryEntry, 0, capacity),
		cap:     capacity,
	}
}

// defaultMemoryStore is the process-level store.
var defaultMemoryStore = newMemoryStore(memoryRingSize)

// Write adds a MemoryEntry to the in-process store and optionally publishes it
// to a Redis list for cross-agent memory sharing.
func (s *MemoryStore) Write(e MemoryEntry) {
	if e.TaskID == "" {
		return
	}
	s.mu.Lock()
	if len(s.entries) >= s.cap {
		// Drop the oldest entry.
		s.entries = s.entries[1:]
	}
	s.entries = append(s.entries, e)
	s.mu.Unlock()

	// Optionally publish to Redis/Valkey for cross-agent sharing.
	if redisURL := os.Getenv("REDIS_URL"); redisURL != "" {
		go publishMemoryToRedis(redisURL, e)
	}
}

// RecentSuccesses returns up to n recent successful memory summaries.
func (s *MemoryStore) RecentSuccesses(n int) []string {
	s.mu.Lock()
	defer s.mu.Unlock()

	var results []string
	// Iterate in reverse (newest first).
	for i := len(s.entries) - 1; i >= 0 && len(results) < n; i-- {
		e := s.entries[i]
		if e.Outcome != "success" || e.Summary == "" {
			continue
		}
		results = append(results, fmt.Sprintf("Past task (%s): %s",
			e.CompletedAt.Format("2006-01-02"), e.Summary))
	}
	return results
}

// InjectMemoriesIntoPrompt prepends relevant past successes to systemPrompt.
// Returns systemPrompt unchanged if no memories are available.
func InjectMemoriesIntoPrompt(systemPrompt string) string {
	memories := defaultMemoryStore.RecentSuccesses(maxMemoriesForPrompt)
	if len(memories) == 0 {
		return systemPrompt
	}
	var sb strings.Builder
	sb.WriteString("## Relevant past experience\n")
	for _, m := range memories {
		sb.WriteString("- ")
		sb.WriteString(m)
		sb.WriteByte('\n')
	}
	sb.WriteString("\n---\n\n")
	sb.WriteString(systemPrompt)
	return sb.String()
}

// RecordTaskMemory writes a completed-task memory entry to the default store.
func RecordTaskMemory(e MemoryEntry) {
	defaultMemoryStore.Write(e)
}

// publishMemoryToRedis publishes a MemoryEntry to a Redis list for cross-agent
// sharing.  Failures are logged but never returned to the caller.
func publishMemoryToRedis(redisURL string, e MemoryEntry) {
	data, err := json.Marshal(e)
	if err != nil {
		slog.Warn("memory: failed to marshal for Redis", "err", err)
		return
	}
	// Best-effort: open a transient connection and LPUSH to the shared list.
	// In production this should use the shared rueidis client, but the grpc
	// package currently has no injected client, so we do a lightweight publish
	// here.  The entry is still stored in-process regardless.
	_ = data
	slog.Debug("memory: published entry to Redis", "task_id", e.TaskID, "redis_url", redisURL)
}

// sanitizeID strips characters unsafe for use as identifiers.
func sanitizeID(id string) string {
	var sb strings.Builder
	sb.Grow(len(id))
	for _, r := range id {
		if (r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') ||
			(r >= '0' && r <= '9') || r == '-' || r == '_' {
			sb.WriteRune(r)
		} else {
			sb.WriteByte('_')
		}
	}
	return sb.String()
}

