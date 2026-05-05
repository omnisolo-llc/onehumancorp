package blobinspector

import (
	"context"
	"errors"
	"fmt"
	"path"
	"strings"
	"time"
)

type BlobMetadata struct {
	Key          string    `json:"key"`
	Size         int64     `json:"size"`
	LastModified time.Time `json:"last_modified"`
	ContentType  string    `json:"content_type"`
}

type StorageProvider interface {
	IsLocal() bool
	ListBlobs(ctx context.Context, prefix string) ([]BlobMetadata, error)
	ReadBlobMetadata(ctx context.Context, key string) (*BlobMetadata, error)
	GetBlobURL(ctx context.Context, key string) (string, error)
}

type Claims struct {
	OrganizationID string
}

type Hub interface {
	Storage() StorageProvider
}

type BlobInspector struct {
	hub Hub
}

func NewBlobInspector(hub Hub) *BlobInspector {
	return &BlobInspector{
		hub: hub,
	}
}

func (b *BlobInspector) ListTools() []map[string]interface{} {
	return []map[string]interface{}{
		{
			"name":        "list_blobs",
			"description": "Lists blobs in the storage.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"prefix": map[string]interface{}{
						"type":        "string",
						"description": "The prefix to list blobs for.",
					},
				},
			},
		},
		{
			"name":        "read_blob_metadata",
			"description": "Reads metadata for a specific blob.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"key": map[string]interface{}{
						"type":        "string",
						"description": "The key of the blob.",
					},
				},
				"required": []string{"key"},
			},
		},
		{
			"name":        "get_blob_url",
			"description": "Gets a URL for a specific blob.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"key": map[string]interface{}{
						"type":        "string",
						"description": "The key of the blob.",
					},
				},
				"required": []string{"key"},
			},
		},
	}
}

func sanitizeKey(key string) (string, error) {
	if key == "" || key == "/" {
		return "", nil
	}

	// Prevent path traversal
	cleaned := path.Clean(key)
	if strings.HasPrefix(cleaned, "../") || cleaned == ".." {
		return "", errors.New("invalid key: path traversal not allowed")
	}
	// For object storage, strip leading slash
	cleaned = strings.TrimPrefix(cleaned, "/")

	// Preserve trailing slash for list operations if it existed originally
	if strings.HasSuffix(key, "/") && !strings.HasSuffix(cleaned, "/") {
		cleaned += "/"
	}

	return cleaned, nil
}

func (b *BlobInspector) CallTool(ctx context.Context, name string, args map[string]interface{}, claims *Claims) (interface{}, error) {
	if claims == nil {
		return nil, errors.New("unauthorized: claims are missing")
	}

	storage := b.hub.Storage()
	if storage == nil {
		return nil, errors.New("storage provider not configured")
	}

	isLocal := storage.IsLocal()

	switch name {
	case "list_blobs":
		prefix := ""
		if p, ok := args["prefix"].(string); ok {
			prefix = p
		}

		effectivePrefix, err := sanitizeKey(prefix)
		if err != nil {
			return nil, err
		}

		if !isLocal {
			if claims.OrganizationID == "" {
				return nil, errors.New("organization ID is required for cloud storage")
			}
			if effectivePrefix != "" {
				effectivePrefix = claims.OrganizationID + "/" + effectivePrefix
			} else {
				effectivePrefix = claims.OrganizationID + "/"
			}
		}

		blobs, err := storage.ListBlobs(ctx, effectivePrefix)
		if err != nil {
			return nil, fmt.Errorf("failed to list blobs: %w", err)
		}

		// Strip org prefix from results in cloud mode to keep it transparent
		if !isLocal {
			for i := range blobs {
				blobs[i].Key = strings.TrimPrefix(blobs[i].Key, claims.OrganizationID+"/")
			}
		}

		return blobs, nil

	case "read_blob_metadata":
		key, ok := args["key"].(string)
		if !ok || key == "" {
			return nil, errors.New("invalid or missing 'key' argument")
		}

		effectiveKey, err := sanitizeKey(key)
		if err != nil {
			return nil, err
		}

		if !isLocal {
			if claims.OrganizationID == "" {
				return nil, errors.New("organization ID is required for cloud storage")
			}
			effectiveKey = claims.OrganizationID + "/" + effectiveKey
		}

		metadata, err := storage.ReadBlobMetadata(ctx, effectiveKey)
		if err != nil {
			return nil, fmt.Errorf("failed to read blob metadata: %w", err)
		}

		if !isLocal {
			metadata.Key = strings.TrimPrefix(metadata.Key, claims.OrganizationID+"/")
		}

		return metadata, nil

	case "get_blob_url":
		key, ok := args["key"].(string)
		if !ok || key == "" {
			return nil, errors.New("invalid or missing 'key' argument")
		}

		effectiveKey, err := sanitizeKey(key)
		if err != nil {
			return nil, err
		}

		if !isLocal {
			if claims.OrganizationID == "" {
				return nil, errors.New("organization ID is required for cloud storage")
			}
			effectiveKey = claims.OrganizationID + "/" + effectiveKey
		}

		url, err := storage.GetBlobURL(ctx, effectiveKey)
		if err != nil {
			return nil, fmt.Errorf("failed to get blob url: %w", err)
		}

		return map[string]interface{}{
			"url": url,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
