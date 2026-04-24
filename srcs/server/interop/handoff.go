package interop

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	pb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/protobuf/proto"
)

// StateHandoff provides a durable protocol to synchronize context and tasks seamlessly
// between Cloud (Redis/Postgres) and Standalone (local SQLite/File) modes when switching.
type StateHandoff struct {
	baseDir string
}

// NewStateHandoff creates a new StateHandoff instance using a durable file store.
func NewStateHandoff() (*StateHandoff, error) {
	baseDir := os.Getenv("OHC_DATA_DIR")
	if baseDir == "" {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			homeDir = os.TempDir()
		}
		baseDir = filepath.Join(homeDir, ".ohc-local-data", "handoff")
	}

	if err := os.MkdirAll(baseDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create handoff directory: %w", err)
	}

	return &StateHandoff{
		baseDir: baseDir,
	}, nil
}

// SyncToStandalone serializes a TaskNotification and durably saves it for offline access.
func (s *StateHandoff) SyncToStandalone(ctx context.Context, notification *pb.TaskNotification) error {
	data, err := proto.Marshal(notification)
	if err != nil {
		return fmt.Errorf("failed to marshal TaskNotification: %w", err)
	}

	filename := filepath.Join(s.baseDir, fmt.Sprintf("task_%s.pb", notification.TaskId))
	return os.WriteFile(filename, data, 0644)
}

// SyncToCloud serializes a TaskNotification and prepares it for cloud sync.
func (s *StateHandoff) SyncToCloud(ctx context.Context, notification *pb.TaskNotification) error {
	data, err := proto.Marshal(notification)
	if err != nil {
		return fmt.Errorf("failed to marshal TaskNotification: %w", err)
	}

	filename := filepath.Join(s.baseDir, fmt.Sprintf("cloud_sync_%s.pb", notification.TaskId))
	return os.WriteFile(filename, data, 0644)
}

// LoadPendingHandoffs reads all durable state files from the handoff directory.
func (s *StateHandoff) LoadPendingHandoffs() ([]*pb.TaskNotification, error) {
	entries, _ := os.ReadDir(s.baseDir)

	var notifications []*pb.TaskNotification
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".pb") {
			continue
		}

		filename := filepath.Join(s.baseDir, entry.Name())
		data, err := os.ReadFile(filename)
		if err != nil {
			os.Remove(filename)
			continue
		}

		var notification pb.TaskNotification
		if err := proto.Unmarshal(data, &notification); err == nil && notification.TaskId != "" {
			notifications = append(notifications, &notification)
		} else {
			os.Remove(filename) // Delete corrupt or empty file
		}
	}
	return notifications, nil
}

// MarkHandoffComplete deletes the handoff files associated with the task ID.
func (s *StateHandoff) MarkHandoffComplete(taskId string) error {
	// Delete both possible variants
	f1 := filepath.Join(s.baseDir, fmt.Sprintf("task_%s.pb", taskId))
	f2 := filepath.Join(s.baseDir, fmt.Sprintf("cloud_sync_%s.pb", taskId))

	os.Remove(f1)
	os.Remove(f2)
	return nil
}
