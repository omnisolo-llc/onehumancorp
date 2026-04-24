package obsidian

import (
	"github.com/onehumancorp/mono/src/server/telemetry"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
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

// ObsidianTool provides methods to interact with local Obsidian vaults.
type ObsidianTool struct {
	dbProvider db.Provider
}

// NewObsidianTool initializes the Obsidian tool handler.
func NewObsidianTool(dbProvider db.Provider) *ObsidianTool {
	return &ObsidianTool{dbProvider: dbProvider}
}

// Note represents a Markdown note in the vault.
type Note struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// Execute performs an Obsidian MCP tool action.
func (t *ObsidianTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	action, ok := payload["action"].(string)
	if !ok {
		return nil, fmt.Errorf("action is required")
	}

	vaultPath, _ := payload["vault_path"].(string)

	// If in Standalone mode, buffer metadata for sync.
	if os.Getenv("OHC_STANDALONE") == "true" && t.dbProvider != nil {
		argsBytes, _ := json.Marshal(telemetry.RedactInterfacePII(payload))
		query := "INSERT INTO hybrid_mcp_sync_queue (id, tool_name, arguments, status, created_at) VALUES ($1, $2, $3, 'PENDING', CURRENT_TIMESTAMP)"
		// Use a UUID or similar for the ID. For simplicity here, we'm using a timestamped string.
		id := fmt.Sprintf("obsidian-%d", time.Now().UnixNano())
		_, _ = t.dbProvider.Exec(ctx, query, id, "obsidian_"+action, string(argsBytes))
	}

	// Mode Distinction: Mock in Cloud mode, Real in Standalone mode.
	if os.Getenv("OHC_STANDALONE") != "true" {
		mockResult := []byte(`{"status": "mocked", "message": "Obsidian tool mocked in Cloud mode"}`)
		return &ExecutionResult{
			ToolID:     "obsidian_" + action,
			Status:     "success",
			ResultData: mockResult,
			ExecutedAt: time.Now().UTC(),
		}, nil
	}

	switch action {
	case "list_notes":
		notes, err := t.ListNotes(ctx, vaultPath)
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(notes)
		return &ExecutionResult{
			ToolID:     "obsidian_list_notes",
			Status:     "success",
			ResultData: resultData,
			ExecutedAt: time.Now().UTC(),
		}, nil
	case "read_note":
		notePath, _ := payload["note_path"].(string)
		note, err := t.ReadNote(ctx, vaultPath, notePath)
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(note)
		return &ExecutionResult{
			ToolID:     "obsidian_read_note",
			Status:     "success",
			ResultData: resultData,
			ExecutedAt: time.Now().UTC(),
		}, nil
	default:
		return nil, fmt.Errorf("unknown action: %s", action)
	}
}

// ListNotes scans the vault path for Markdown files.
func (t *ObsidianTool) ListNotes(ctx context.Context, vaultPath string) ([]string, error) {
	if vaultPath == "" {
		return nil, fmt.Errorf("vault path is required")
	}

	var notes []string
	err := filepath.WalkDir(vaultPath, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.HasSuffix(strings.ToLower(d.Name()), ".md") {
			relPath, err := filepath.Rel(vaultPath, path)
			if err != nil {
				return err
			}
			notes = append(notes, relPath)
		}
		return nil
	})

	if err != nil {
		return nil, fmt.Errorf("failed to list notes: %w", err)
	}

	return notes, nil
}

// ReadNote reads the content of a specific Markdown note.
func (t *ObsidianTool) ReadNote(ctx context.Context, vaultPath, notePath string) (*Note, error) {
	if vaultPath == "" {
		return nil, fmt.Errorf("vault path is required")
	}
	if notePath == "" {
		return nil, fmt.Errorf("note path is required")
	}

	fullPath := filepath.Join(vaultPath, notePath)
	// Security check: ensure the note is within the vault
	cleanVault := filepath.Clean(vaultPath)
	cleanFull := filepath.Clean(fullPath)
	if !strings.HasPrefix(cleanFull, cleanVault) {
		return nil, fmt.Errorf("access denied: note outside vault")
	}

	f, err := os.Open(fullPath)
	if err != nil {
		return nil, fmt.Errorf("failed to open note: %w", err)
	}
	defer f.Close()

	content, err := io.ReadAll(f)
	if err != nil {
		return nil, fmt.Errorf("failed to read note: %w", err)
	}

	return &Note{
		Path:    notePath,
		Content: string(content),
	}, nil
}
