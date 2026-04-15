package local

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"strings"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/db"
)

const (
	// maxOutputBytes is the hard cap per task output.
	// We cap at 50 MB for local execution to avoid filling disk on resource-constrained
	// environments.
	maxOutputBytes = 50 * 1024 * 1024
)

// taskOutput accumulates the agent's streamed output and persists it to the
// database. Multiple goroutines may call Append concurrently; writes are
// serialised by mu.
type taskOutput struct {
	mu      sync.Mutex
	taskID  string
	dbp     db.Provider
	written int64
	buf     strings.Builder // in-memory buffer for DB flush
	closed  bool
}

// newTaskOutput creates a task output writer that stores data in the database.
// If dbp is nil the output is silently discarded (test / fallback mode).
func newTaskOutput(taskID string, dbp db.Provider) (*taskOutput, error) {
	return &taskOutput{taskID: taskID, dbp: dbp}, nil
}

// Append writes data to the output, respecting the size cap.
func (o *taskOutput) Append(data []byte) error {
	if len(data) == 0 {
		return nil
	}
	o.mu.Lock()
	defer o.mu.Unlock()
	if o.written >= maxOutputBytes {
		return nil // silently drop once cap reached
	}
	remaining := maxOutputBytes - o.written
	if int64(len(data)) > remaining {
		data = data[:remaining]
	}
	o.buf.Write(data)
	o.written += int64(len(data))
	return nil
}

// AppendString is a convenience helper.
func (o *taskOutput) AppendString(s string) error {
	return o.Append([]byte(s))
}

// Close flushes any buffered output to the database.
func (o *taskOutput) Close() error {
	o.mu.Lock()
	defer o.mu.Unlock()
	if o.closed {
		return nil
	}
	o.closed = true
	return o.flushLocked()
}

// flushLocked persists the accumulated output to the database.
// Must be called with o.mu held.
func (o *taskOutput) flushLocked() error {
	if o.dbp == nil || o.buf.Len() == 0 {
		return nil
	}
	chunk := o.buf.String()
	id, err := randomHex(8)
	if err != nil {
		return fmt.Errorf("taskOutput: generate id: %w", err)
	}
	ctx := context.Background()
	_, err = o.dbp.Exec(ctx,
		`INSERT INTO agent_task_outputs (id, task_id, chunk) VALUES ($1, $2, $3)`,
		id, o.taskID, chunk,
	)
	if err != nil {
		return fmt.Errorf("taskOutput: persist chunk: %w", err)
	}
	o.buf.Reset()
	return nil
}

// Evict removes the task output rows from the database after the completion
// notification has been consumed.
func (o *taskOutput) Evict() {
	if o.dbp == nil {
		return
	}
	ctx := context.Background()
	_, _ = o.dbp.Exec(ctx, `DELETE FROM agent_task_outputs WHERE task_id = $1`, o.taskID)
}

func randomHex(n int) (string, error) {
	b := make([]byte, n)
	if _, err := rand.Read(b); err != nil {
		return "", err
	}
	return hex.EncodeToString(b), nil
}

// taskOutputPath returns a stable key identifying the task output location.
// It is kept as a string label used in notification payloads.
func taskOutputPath(taskID string) string {
	return "db://" + taskID
}
