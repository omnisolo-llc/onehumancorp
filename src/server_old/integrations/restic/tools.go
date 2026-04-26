package restic

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"time"
)

// ExecutionResult represents the result of a tool execution.
type ExecutionResult struct {
	ToolID           string          `json:"tool_id"`
	Status           string          `json:"status"`
	ResultData       json.RawMessage `json:"result_data"`
	HybridEscalation bool            `json:"hybrid_escalation"`
	Escalation       bool            `json:"escalation"`
	ExecutedAt       time.Time       `json:"executed_at"`
}

// ResticTool provides methods to interact with local Restic backup repositories.
type ResticTool struct {
}

// NewResticTool initializes the Restic tool handler.
func NewResticTool() *ResticTool {
	return &ResticTool{}
}

// Execute performs a Restic MCP tool action.
func (t *ResticTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	if os.Getenv("OHC_STANDALONE") != "true" {
		return nil, fmt.Errorf("restic integration is only supported in standalone mode")
	}

	action, ok := payload["action"].(string)
	if !ok {
		return nil, fmt.Errorf("action is required")
	}

	repository, ok := payload["repository"].(string)
	if !ok {
		return nil, fmt.Errorf("repository is required")
	}

	password, ok := payload["password"].(string)
	if !ok {
		return nil, fmt.Errorf("password is required")
	}

	var resultData []byte
	var err error

	switch action {
	case "snapshot":
		targetDir, ok := payload["target_dir"].(string)
		if !ok {
			return nil, fmt.Errorf("target_dir is required for snapshot action")
		}
		resultData, err = t.ResticSnapshot(ctx, repository, password, targetDir)
	case "restore":
		snapshotID, ok := payload["snapshot_id"].(string)
		if !ok {
			return nil, fmt.Errorf("snapshot_id is required for restore action")
		}
		targetDir, ok := payload["target_dir"].(string)
		if !ok {
			return nil, fmt.Errorf("target_dir is required for restore action")
		}
		resultData, err = t.ResticRestore(ctx, repository, password, snapshotID, targetDir)
	case "status":
		resultData, err = t.ResticStatus(ctx, repository, password)
	default:
		return nil, fmt.Errorf("unknown action: %s", action)
	}

	if err != nil {
		return nil, err
	}

	return &ExecutionResult{
		ToolID:     "restic_" + action,
		Status:     "success",
		ResultData: resultData,
		ExecutedAt: time.Now().UTC(),
	}, nil
}

// ResticSnapshot creates a new snapshot of the target directory.
func (t *ResticTool) ResticSnapshot(ctx context.Context, repository, password, targetDir string) ([]byte, error) {
	cmd := exec.CommandContext(ctx, "restic", "backup", targetDir, "--json")
	cmd.Env = append(os.Environ(), fmt.Sprintf("RESTIC_REPOSITORY=%s", repository), fmt.Sprintf("RESTIC_PASSWORD=%s", password))

	out, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("restic backup failed: %w", err)
	}

	return out, nil
}

// ResticRestore restores a snapshot to the target directory.
func (t *ResticTool) ResticRestore(ctx context.Context, repository, password, snapshotID, targetDir string) ([]byte, error) {
	cmd := exec.CommandContext(ctx, "restic", "restore", snapshotID, "--target", targetDir, "--json")
	cmd.Env = append(os.Environ(), fmt.Sprintf("RESTIC_REPOSITORY=%s", repository), fmt.Sprintf("RESTIC_PASSWORD=%s", password))

	out, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("restic restore failed: %w", err)
	}

	return out, nil
}

// ResticStatus returns information about the repository.
func (t *ResticTool) ResticStatus(ctx context.Context, repository, password string) ([]byte, error) {
	cmd := exec.CommandContext(ctx, "restic", "snapshots", "--json")
	cmd.Env = append(os.Environ(), fmt.Sprintf("RESTIC_REPOSITORY=%s", repository), fmt.Sprintf("RESTIC_PASSWORD=%s", password))

	out, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("restic snapshots failed: %w", err)
	}

	return out, nil
}
