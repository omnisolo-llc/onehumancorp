package agentgrpc

// memory.go implements AutoDream-style persistent memory for self-learning.
//
// After each completed task the agent writes a compressed YAML memory file to
// .agent-task/memory/<taskID>.yml.  On subsequent runs, the agent retrieves
// the N most-similar past memories and prepends them to the system prompt so
// the LLM can learn from prior successes.
//
// The design mirrors orchestration.AutoDreamPipeline (orchestration package)
// but is self-contained in the agent binary to avoid a circular dependency.
//
// Memory entries are compressed with gzip+base64 when they exceed 4 KiB to
// keep the filesystem footprint small.

import (
	"bytes"
	"compress/gzip"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

const (
	memoryDir            = ".agent-task/memory"
	memoryCompressThresh = 4 * 1024 // compress entries larger than 4 KiB
	maxMemoriesForPrompt = 5        // inject at most 5 past memories into prompt
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

// MemoryStore manages read/write of agent memory files.
// It is safe for concurrent use.
type MemoryStore struct {
	dir  string
	mu   sync.Mutex
	pool sync.Pool // []byte reuse for marshal/compress
}

// defaultMemoryStore is the process-level store.
var defaultMemoryStore = &MemoryStore{
	dir: memoryDir,
	pool: sync.Pool{New: func() any {
		b := make([]byte, 0, 4096)
		return &b
	}},
}

// Write persists a MemoryEntry to disk.  It silently no-ops when e is zero-value.
func (s *MemoryStore) Write(e MemoryEntry) {
	if e.TaskID == "" {
		return
	}
	s.mu.Lock()
	defer s.mu.Unlock()

	if err := os.MkdirAll(s.dir, 0o755); err != nil {
		slog.Warn("memory: mkdir failed", "err", err)
		return
	}

	data, err := json.Marshal(e)
	if err != nil {
		slog.Warn("memory: marshal failed", "err", err)
		return
	}

	if len(data) > memoryCompressThresh {
		data, err = compressBytes(data)
		if err != nil {
			slog.Warn("memory: compress failed", "err", err)
			// fall through and write uncompressed
			data, _ = json.Marshal(e)
		}
	}

	path := filepath.Join(s.dir, sanitizeID(e.TaskID)+".json")
	if err := writeFileAtomic(path, data); err != nil {
		slog.Warn("memory: write failed", "path", path, "err", err)
	}
}

// RecentSuccesses returns up to n recent successful memory summaries.
// The strings are suitable for direct inclusion in a system prompt.
func (s *MemoryStore) RecentSuccesses(n int) []string {
	s.mu.Lock()
	defer s.mu.Unlock()

	entries, err := filepath.Glob(filepath.Join(s.dir, "*.json"))
	if err != nil || len(entries) == 0 {
		return nil
	}

	// Read at most 2*n candidates (newest first via lexicographic sort of timestamps).
	limit := n * 2
	if limit > len(entries) {
		limit = len(entries)
	}
	// entries from Glob are lexicographically sorted; reverse to get newest first.
	for i, j := 0, limit-1; i < j; i, j = i+1, j-1 {
		entries[i], entries[j] = entries[j], entries[i]
	}

	var results []string
	for _, path := range entries {
		if len(results) >= n {
			break
		}
		e, err := readMemoryFile(path)
		if err != nil || e.Outcome != "success" {
			continue
		}
		if e.Summary == "" {
			continue
		}
		results = append(results, fmt.Sprintf("Past task (%s): %s", e.CompletedAt.Format("2006-01-02"), e.Summary))
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

// ── helpers ───────────────────────────────────────────────────────────────────

func compressBytes(data []byte) ([]byte, error) {
	var buf bytes.Buffer
	w := gzip.NewWriter(&buf)
	if _, err := w.Write(data); err != nil {
		return nil, err
	}
	if err := w.Close(); err != nil {
		return nil, err
	}
	encoded := base64.StdEncoding.EncodeToString(buf.Bytes())
	wrapper := struct {
		Compressed string `json:"_gz"`
	}{Compressed: encoded}
	return json.Marshal(wrapper)
}

func decompressBytes(data []byte) ([]byte, error) {
	var wrapper struct {
		Compressed string `json:"_gz"`
	}
	if err := json.Unmarshal(data, &wrapper); err != nil || wrapper.Compressed == "" {
		return data, nil // not compressed
	}
	raw, err := base64.StdEncoding.DecodeString(wrapper.Compressed)
	if err != nil {
		return nil, err
	}
	r, err := gzip.NewReader(bytes.NewReader(raw))
	if err != nil {
		return nil, err
	}
	defer r.Close()
	return io.ReadAll(r)
}

func readMemoryFile(path string) (MemoryEntry, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return MemoryEntry{}, err
	}
	data, err = decompressBytes(data)
	if err != nil {
		return MemoryEntry{}, err
	}
	var e MemoryEntry
	if err := json.Unmarshal(data, &e); err != nil {
		return MemoryEntry{}, err
	}
	return e, nil
}

// writeFileAtomic writes data to path via a temp file + rename to avoid
// partial writes on crash.
func writeFileAtomic(path string, data []byte) error {
	tmp := path + ".tmp"
	if err := os.WriteFile(tmp, data, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, path)
}

// sanitizeID strips characters unsafe for filesystem paths.
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
