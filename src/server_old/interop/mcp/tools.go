package mcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
)

type WorkspaceSyncTool struct {
	proxy *McpSyncProxy

	// Add test hooks to allow 100% coverage
	walkDirFunc func(root string, fn fs.WalkDirFunc) error
	readFileFunc func(name string) ([]byte, error)
}

func NewWorkspaceSyncTool(proxy *McpSyncProxy) *WorkspaceSyncTool {
	return &WorkspaceSyncTool{
		proxy: proxy,
		walkDirFunc: filepath.WalkDir,
		readFileFunc: os.ReadFile,
	}
}

func (t *WorkspaceSyncTool) Execute(ctx context.Context, path string, strategy string) error {
	if strategy != "metadata_only" && strategy != "full_content" {
		return fmt.Errorf("invalid sync strategy: %s", strategy)
	}

	files := make(map[string]interface{})

	err := t.walkDirFunc(path, func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if d.IsDir() {
			return nil
		}

		relPath, err := filepath.Rel(path, p)
		if err != nil {
			return err
		}

		info, err := d.Info()
		if err != nil {
			return err
		}

		fileData := map[string]interface{}{
			"size": info.Size(),
			"mode": info.Mode().String(),
		}

		if strategy == "full_content" {
			if info.Size() < 1024*1024 { // Less than 1MB
				content, err := t.readFileFunc(p)
				if err != nil {
					return err
				}
				fileData["content"] = string(content)
			} else {
				fileData["content_skipped_size"] = true
			}
		}

		files[relPath] = fileData
		return nil
	})

	if err != nil {
		return fmt.Errorf("failed to walk workspace: %w", err)
	}

	payload := map[string]interface{}{
		"workspace_path": path,
		"strategy":       strategy,
		"files":          files,
	}

	_, err = t.proxy.BufferIntegrationState(ctx, "hybrid_workspace_sync", payload)
	if err != nil {
		return fmt.Errorf("failed to buffer state: %w", err)
	}

	err = t.proxy.SyncPendingStates(ctx)
	if err != nil {
		return fmt.Errorf("failed to sync pending states: %w", err)
	}

	return nil
}
