package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
)

type ProxyDatabase interface {
	EnqueueState(ctx context.Context, toolName string, payload []byte) error
	Sync(ctx context.Context) error
}

type McpSyncProxy struct {
	DB ProxyDatabase
}

func NewMcpSyncProxy(db ProxyDatabase) *McpSyncProxy {
	return &McpSyncProxy{DB: db}
}

func (p *McpSyncProxy) BufferIntegrationState(ctx context.Context, toolName string, payload []byte) error {
	return p.DB.EnqueueState(ctx, toolName, payload)
}

func (p *McpSyncProxy) SyncPendingStates(ctx context.Context) error {
	return p.DB.Sync(ctx)
}

type WorkspaceSyncTool struct {
	Proxy *McpSyncProxy
}

func NewWorkspaceSyncTool(proxy *McpSyncProxy) *WorkspaceSyncTool {
	return &WorkspaceSyncTool{Proxy: proxy}
}

var jsonMarshal = json.Marshal

type FileData struct {
	Path    string `json:"path"`
	IsDir   bool   `json:"is_dir"`
	Size    int64  `json:"size"`
	Content string `json:"content,omitempty"` // Base64 or plain string if "full_content"
}

func (t *WorkspaceSyncTool) Execute(ctx context.Context, path string, strategy string) error {
	if strategy != "metadata_only" && strategy != "full_content" {
		return fmt.Errorf("invalid strategy: %s", strategy)
	}

	var files []FileData

	err := filepath.Walk(path, func(currentPath string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}

		fileData := FileData{
			Path:  currentPath,
			IsDir: info.IsDir(),
			Size:  info.Size(),
		}

		if !info.IsDir() && strategy == "full_content" {
			// Read up to 1MB
			if info.Size() <= 1024*1024 {
				content, readErr := os.ReadFile(currentPath)
				if readErr == nil {
					fileData.Content = string(content)
				}
			}
		}

		files = append(files, fileData)
		return nil
	})

	if err != nil {
		return fmt.Errorf("failed to walk path: %w", err)
	}

	payload := map[string]interface{}{
		"workspace_path": path,
		"strategy":       strategy,
		"files":          files,
	}

	stateBytes, err := jsonMarshal(payload)
	if err != nil {
		return fmt.Errorf("failed to marshal state: %w", err)
	}

	if err := t.Proxy.BufferIntegrationState(ctx, "hybrid_workspace_sync", stateBytes); err != nil {
		return fmt.Errorf("failed to buffer state: %w", err)
	}

	if err := t.Proxy.SyncPendingStates(ctx); err != nil {
		return fmt.Errorf("failed to sync states: %w", err)
	}

	return nil
}
