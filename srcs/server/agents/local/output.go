package local

import (
	"fmt"
	"os"
	"path/filepath"
	"sync"
)

const (
	// maxOutputBytes is the hard cap per task output file.
	// We cap at 50 MB for local execution to avoid filling disk on resource-constrained
	// environments. CC-Source uses 5 GB because it runs on remote cloud machines.
	maxOutputBytes = 50 * 1024 * 1024
)

// taskOutput manages the disk file that accumulates the agent's streamed output.
// Multiple goroutines may call Append concurrently; writes are serialised by mu.
type taskOutput struct {
	mu      sync.Mutex
	path    string
	written int64
	f       *os.File
}

// newTaskOutput opens (or creates) the output file at path.
func newTaskOutput(path string) (*taskOutput, error) {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return nil, fmt.Errorf("taskOutput: mkdir %s: %w", filepath.Dir(path), err)
	}
	// O_CREATE | O_APPEND so concurrent writes don't race on the offset.
	f, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0o644)
	if err != nil {
		return nil, fmt.Errorf("taskOutput: open %s: %w", path, err)
	}
	return &taskOutput{path: path, f: f}, nil
}

// Append writes data to the output file, respecting the size cap.
func (o *taskOutput) Append(data []byte) error {
	if len(data) == 0 {
		return nil
	}
	o.mu.Lock()
	defer o.mu.Unlock()
	if o.written >= maxOutputBytes {
		return nil // silently drop once cap reached (mirrors CC-Source behaviour)
	}
	remaining := maxOutputBytes - o.written
	if int64(len(data)) > remaining {
		data = data[:remaining]
	}
	n, err := o.f.Write(data)
	o.written += int64(n)
	return err
}

// AppendString is a convenience helper.
func (o *taskOutput) AppendString(s string) error {
	return o.Append([]byte(s))
}

// Close flushes and closes the underlying file.
func (o *taskOutput) Close() error {
	o.mu.Lock()
	defer o.mu.Unlock()
	if o.f == nil {
		return nil
	}
	err := o.f.Close()
	o.f = nil
	return err
}

// Evict removes the output file from disk. Called after the task notification
// has been consumed so the file does not accumulate.
func (o *taskOutput) Evict() {
	o.mu.Lock()
	defer o.mu.Unlock()
	_ = os.Remove(o.path)
}

// taskOutputDir returns the directory used for all task output files.
// It follows the same layout as CC-Source:
//
//	{tempDir}/tasks/{taskID}.output
func taskOutputDir() string {
	base := os.Getenv("OHC_TASK_OUTPUT_DIR")
	if base == "" {
		base = filepath.Join(os.TempDir(), "ohc-tasks")
	}
	return base
}

// taskOutputPath returns the full path for the given task ID's output file.
func taskOutputPath(taskID string) string {
	return filepath.Join(taskOutputDir(), taskID+".output")
}
