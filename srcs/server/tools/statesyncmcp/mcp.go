package statesyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

// Server implements the MCP interface for state sync
type Server struct {
	provider StateSyncProvider
}

// NewServer creates a new StateSync MCP server
func NewServer(provider StateSyncProvider) *Server {
	return &Server{
		provider: provider,
	}
}

// ListTools returns the available MCP tools
func (s *Server) ListTools(ctx context.Context) ([]Tool, error) {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Pushes local state transitions to the cloud backend",
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetches state updates from the cloud backend to the local database",
		},
		{
			Name:        "get_sync_status",
			Description: "Returns the current sync status",
		},
	}, nil
}

// CallTool executes the specified MCP tool
func (s *Server) CallTool(ctx context.Context, name string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	switch name {
	case "sync_local_to_cloud":
		err := s.provider.SyncUp(ctx, claims)
		if err != nil {
			return nil, fmt.Errorf("sync_local_to_cloud failed: %w", err)
		}
		return map[string]string{"status": "success"}, nil

	case "sync_cloud_to_local":
		err := s.provider.SyncDown(ctx, claims)
		if err != nil {
			return nil, fmt.Errorf("sync_cloud_to_local failed: %w", err)
		}
		return map[string]string{"status": "success"}, nil

	case "get_sync_status":
		status, err := s.provider.GetStatus(ctx, claims)
		if err != nil {
			return nil, fmt.Errorf("get_sync_status failed: %w", err)
		}
		return status, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
