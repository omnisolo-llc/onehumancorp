package interop

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
	"google.golang.org/protobuf/proto"
)

// HandoffStore defines the interface for state handoff between Cloud and Standalone.
type HandoffStore interface {
	WriteHandoff(ctx context.Context, taskID string, payload *agentservicepb.TaskNotification) error
	ReadHandoff(ctx context.Context, taskID string) (*agentservicepb.TaskNotification, error)
	ListHandoffs(ctx context.Context) ([]string, error)
	DeleteHandoff(ctx context.Context, taskID string) error
}

type FileHandoffStore struct {
	baseDir string
}

func NewFileHandoffStore(baseDir ...string) (*FileHandoffStore, error) {
	var dir string
	if len(baseDir) > 0 && baseDir[0] != "" {
		dir = baseDir[0]
	} else {
		dataDir := os.Getenv("OHC_DATA_DIR")
		if dataDir == "" {
			homeDir, err := os.UserHomeDir()
			if err != nil {
				return nil, fmt.Errorf("failed to get user home dir: %w", err)
			}
			dataDir = filepath.Join(homeDir, ".ohc-local-data")
		}
		dir = filepath.Join(dataDir, "handoff")
	}

	if err := os.MkdirAll(dir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create handoff directory: %w", err)
	}
	return &FileHandoffStore{baseDir: dir}, nil
}

func (s *FileHandoffStore) WriteHandoff(ctx context.Context, taskID string, payload *agentservicepb.TaskNotification) error {
	data, err := proto.Marshal(payload)
	if err != nil {
		return fmt.Errorf("failed to marshal handoff notification: %w", err)
	}
	handoffFile := filepath.Join(s.baseDir, taskID+".pb")

	// Create a temporary file and rename to ensure atomicity
	tmpFile := fmt.Sprintf("%s.tmp.%d", handoffFile, time.Now().UnixNano())
	if err := os.WriteFile(tmpFile, data, 0644); err != nil {
		return fmt.Errorf("failed to write temporary handoff file: %w", err)
	}
	if err := os.Rename(tmpFile, handoffFile); err != nil {
		os.Remove(tmpFile)
		return fmt.Errorf("failed to rename handoff file: %w", err)
	}
	return nil
}

func (s *FileHandoffStore) ReadHandoff(ctx context.Context, taskID string) (*agentservicepb.TaskNotification, error) {
	handoffFile := filepath.Join(s.baseDir, taskID+".pb")
	data, err := os.ReadFile(handoffFile)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil // Return nil if not exists, caller should handle
		}
		return nil, fmt.Errorf("failed to read handoff file: %w", err)
	}
	var payload agentservicepb.TaskNotification
	if err := proto.Unmarshal(data, &payload); err != nil {
		return nil, fmt.Errorf("failed to unmarshal handoff notification: %w", err)
	}
	return &payload, nil
}

func (s *FileHandoffStore) ListHandoffs(ctx context.Context) ([]string, error) {
	entries, err := os.ReadDir(s.baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to read handoff directory: %w", err)
	}
	var taskIDs []string
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".pb") {
			taskIDs = append(taskIDs, strings.TrimSuffix(entry.Name(), ".pb"))
		}
	}
	return taskIDs, nil
}

func (s *FileHandoffStore) DeleteHandoff(ctx context.Context, taskID string) error {
	handoffFile := filepath.Join(s.baseDir, taskID+".pb")
	if err := os.Remove(handoffFile); err != nil && !os.IsNotExist(err) {
		return fmt.Errorf("failed to delete handoff file: %w", err)
	}
	return nil
}
