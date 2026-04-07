package statesyncmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

type Server struct {
	provider StateSyncProvider
}

func NewServer(provider StateSyncProvider) *Server {
	return &Server{
		provider: provider,
	}
}

func (s *Server) ListTools(ctx context.Context) ([]Tool, error) {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronize local state to the cloud.",
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Synchronize cloud state to the local database.",
		},
		{
			Name:        "get_sync_status",
			Description: "Get the current synchronization status.",
		},
	}, nil
}

func (s *Server) CallTool(ctx context.Context, name string, args map[string]interface{}) (string, error) {
	switch name {
	case "sync_local_to_cloud":
		res, err := s.provider.SyncUp(ctx)
		if err != nil {
			return "", err
		}
		b, err := json.Marshal(res)
		return string(b), err
	case "sync_cloud_to_local":
		res, err := s.provider.SyncDown(ctx)
		if err != nil {
			return "", err
		}
		b, err := json.Marshal(res)
		return string(b), err
	case "get_sync_status":
		res, err := s.provider.GetStatus(ctx)
		if err != nil {
			return "", err
		}
		b, err := json.Marshal(res)
		return string(b), err
	default:
		return "", fmt.Errorf("unknown tool: %s", name)
	}
}
