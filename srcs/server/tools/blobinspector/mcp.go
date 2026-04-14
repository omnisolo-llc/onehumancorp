package blobinspector

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/storage"
)

// BlobInspectorMCP implements the MCP interface for blob storage access.
type BlobInspectorMCP struct {
	provider storage.Provider
}

// NewBlobInspectorMCP creates a new BlobInspectorMCP instance.
func NewBlobInspectorMCP(provider storage.Provider) *BlobInspectorMCP {
	return &BlobInspectorMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *BlobInspectorMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "list_blobs",
			Description: "Lists blobs under a given prefix.",
			InputSchema: `{"type": "object", "properties": {"prefix": {"type": "string"}}}`,
		},
		{
			Name:        "read_blob_metadata",
			Description: "Retrieves metadata for a specific blob.",
			InputSchema: `{"type": "object", "properties": {"key": {"type": "string"}}, "required": ["key"]}`,
		},
		{
			Name:        "get_blob_url",
			Description: "Retrieves an accessible URL for a specific blob.",
			InputSchema: `{"type": "object", "properties": {"key": {"type": "string"}}, "required": ["key"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *BlobInspectorMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims")
	}

	switch toolName {
	case "list_blobs":
		prefix := ""
		if p, ok := arguments["prefix"].(string); ok {
			prefix = p
		}
		return m.listBlobs(ctx, claims, prefix)
	case "read_blob_metadata":
		key, ok := arguments["key"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		return m.readBlobMetadata(ctx, claims, key)
	case "get_blob_url":
		key, ok := arguments["key"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		return m.getBlobURL(ctx, claims, key)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *BlobInspectorMCP) resolveKey(claims *auth.Claims, key string) (string, error) {
	if strings.Contains(key, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	if m.provider.IsLocal() || claims == nil {
		return key, nil
	}

	// Cloud mode enforces tenant isolation
	cleanKey := filepath.Clean("/" + key)
	cleanKey = strings.TrimPrefix(cleanKey, "/")

	if cleanKey == "" {
		return claims.OrganizationID + "/", nil
	}

	// Ensure we don't prepend if it already starts with it (which would be weird, but defensive)
	if strings.HasPrefix(cleanKey, claims.OrganizationID+"/") {
		return cleanKey, nil
	}

	return fmt.Sprintf("%s/%s", claims.OrganizationID, cleanKey), nil
}

func (m *BlobInspectorMCP) listBlobs(ctx context.Context, claims *auth.Claims, prefix string) (interface{}, error) {
	scopedPrefix, err := m.resolveKey(claims, prefix)
	if err != nil {
		return nil, err
	}

	blobs, err := m.provider.ListBlobs(ctx, scopedPrefix)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, b := range blobs {
		// Strip tenant ID from output if in cloud mode
		key := b.Key
		if !m.provider.IsLocal() && claims != nil {
			key = strings.TrimPrefix(key, claims.OrganizationID+"/")
		}

		results = append(results, map[string]interface{}{
			"key":           key,
			"size":          b.Size,
			"last_modified": b.LastModified.Format(time.RFC3339),
			"content_type":  b.ContentType,
		})
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"results": results,
	}, nil
}

func (m *BlobInspectorMCP) readBlobMetadata(ctx context.Context, claims *auth.Claims, key string) (interface{}, error) {
	scopedKey, err := m.resolveKey(claims, key)
	if err != nil {
		return nil, err
	}

	b, err := m.provider.ReadBlobMetadata(ctx, scopedKey)
	if err != nil {
		return nil, err
	}

	// Strip tenant ID from output if in cloud mode
	outKey := b.Key
	if !m.provider.IsLocal() && claims != nil {
		outKey = strings.TrimPrefix(outKey, claims.OrganizationID+"/")
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":        "success",
		"mode":          mode,
		"key":           outKey,
		"size":          b.Size,
		"last_modified": b.LastModified.Format(time.RFC3339),
		"content_type":  b.ContentType,
	}, nil
}

func (m *BlobInspectorMCP) getBlobURL(ctx context.Context, claims *auth.Claims, key string) (interface{}, error) {
	scopedKey, err := m.resolveKey(claims, key)
	if err != nil {
		return nil, err
	}

	url, err := m.provider.GetBlobURL(ctx, scopedKey)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
		"url":    url,
	}, nil
}
