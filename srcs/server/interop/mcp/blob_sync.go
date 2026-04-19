package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

type BlobSyncTool struct {
	proxy *McpSyncProxy
}

func NewBlobSyncTool(proxy *McpSyncProxy) *BlobSyncTool {
	return &BlobSyncTool{
		proxy: proxy,
	}
}

type MetadataResponse struct {
	DownloadUrl string `json:"download_url"`
}

func (t *BlobSyncTool) Execute(ctx context.Context, blobId string) error {
	if strings.Contains(blobId, "/") || strings.Contains(blobId, "\\") || blobId == ".." || blobId == "." {
		return fmt.Errorf("invalid blob ID")
	}

	metadataUrl := fmt.Sprintf("%s/api/mcp/blob_metadata/%s", t.proxy.cloudEndpoint, blobId)
	reqMeta, err := http.NewRequestWithContext(ctx, "GET", metadataUrl, nil)
	if err != nil {
		return fmt.Errorf("failed to create metadata request: %w", err)
	}

	respMeta, err := t.proxy.httpClient.Do(reqMeta)
	if err != nil {
		return fmt.Errorf("failed to download blob metadata: %w", err)
	}
	defer respMeta.Body.Close()

	if respMeta.StatusCode >= 400 {
		return fmt.Errorf("failed to download blob metadata: status %d", respMeta.StatusCode)
	}

	var metadata MetadataResponse
	if err := json.NewDecoder(respMeta.Body).Decode(&metadata); err != nil {
		return fmt.Errorf("failed to decode metadata: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "GET", metadata.DownloadUrl, nil)
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	resp, err := t.proxy.httpClient.Do(req)
	if err != nil {
		return fmt.Errorf("failed to download blob: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		return fmt.Errorf("failed to download blob: status %d", resp.StatusCode)
	}

	blobDir := os.TempDir()

	path := filepath.Join(blobDir, blobId)
	out, err := os.Create(path)
	if err != nil {
		return fmt.Errorf("failed to create local file: %w", err)
	}
	defer out.Close()

	if _, err := io.Copy(out, resp.Body); err != nil {
		return fmt.Errorf("failed to write blob to disk: %w", err)
	}

	return nil
}
