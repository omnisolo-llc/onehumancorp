package statesyncmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// SyncStatus represents the result of a sync operation
type SyncStatus string

const (
	SyncStatusSuccess SyncStatus = "SUCCESS"
	SyncStatusError   SyncStatus = "ERROR"
	SyncStatusPending SyncStatus = "PENDING"
)

// StateSyncProvider defines the interface for local-to-cloud state synchronization
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims, payload json.RawMessage) (*SyncResult, error)
	SyncDown(ctx context.Context, claims *auth.Claims) (*SyncResult, error)
	GetStatus(ctx context.Context, claims *auth.Claims) (*SyncStatusResponse, error)
}

// SyncResult is returned after a sync operation
type SyncResult struct {
	SyncedRecords int        `json:"synced_records"`
	Status        SyncStatus `json:"status"`
	Message       string     `json:"message"`
	Timestamp     time.Time  `json:"timestamp"`
}

// SyncStatusResponse is returned when querying sync status
type SyncStatusResponse struct {
	LastSyncUp   time.Time  `json:"last_sync_up"`
	LastSyncDown time.Time  `json:"last_sync_down"`
	Status       SyncStatus `json:"status"`
	PendingCount int        `json:"pending_count"`
}

// MCPServer implements the Model Context Protocol server for state sync
type MCPServer struct {
	provider StateSyncProvider
}

// NewMCPServer creates a new StateSync MCP Server
func NewMCPServer(provider StateSyncProvider) *MCPServer {
	return &MCPServer{
		provider: provider,
	}
}

// Tool represents an MCP tool definition
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

// ListTools returns the tools exposed by this MCP server
func (s *MCPServer) ListTools(ctx context.Context) ([]Tool, error) {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronizes local state to the cloud backend. Requires payload with state changes.",
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetches state from the cloud and updates the local database.",
		},
		{
			Name:        "get_sync_status",
			Description: "Gets the current synchronization status and pending record count.",
		},
	}, nil
}

// CallTool executes a specific tool
func (s *MCPServer) CallTool(ctx context.Context, name string, args json.RawMessage) (json.RawMessage, error) {
	// Extract claims from context
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, fmt.Errorf("unauthorized: missing auth claims")
	}

	switch name {
	case "sync_local_to_cloud":
		result, err := s.provider.SyncUp(ctx, claims, args)
		if err != nil {
			return nil, err
		}
		b, err := json.Marshal(result)
		if err != nil {
			return nil, err
		}
		return b, nil

	case "sync_cloud_to_local":
		result, err := s.provider.SyncDown(ctx, claims)
		if err != nil {
			return nil, err
		}
		b, err := json.Marshal(result)
		if err != nil {
			return nil, err
		}
		return b, nil

	case "get_sync_status":
		result, err := s.provider.GetStatus(ctx, claims)
		if err != nil {
			return nil, err
		}
		b, err := json.Marshal(result)
		if err != nil {
			return nil, err
		}
		return b, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

// MockProvider is a fallback provider for testing or when running in pure cloud mode without local DB
type MockProvider struct {
	LastUp   time.Time
	LastDown time.Time
}

// NewMockProvider creates a new MockProvider
func NewMockProvider() *MockProvider {
	return &MockProvider{}
}

// SyncUp mocks syncing up to the cloud
func (m *MockProvider) SyncUp(ctx context.Context, claims *auth.Claims, payload json.RawMessage) (*SyncResult, error) {
	m.LastUp = time.Now()
	return &SyncResult{
		SyncedRecords: 1,
		Status:        SyncStatusSuccess,
		Message:       "Mock sync up successful",
		Timestamp:     m.LastUp,
	}, nil
}

// SyncDown mocks syncing down from the cloud
func (m *MockProvider) SyncDown(ctx context.Context, claims *auth.Claims) (*SyncResult, error) {
	m.LastDown = time.Now()
	return &SyncResult{
		SyncedRecords: 0,
		Status:        SyncStatusSuccess,
		Message:       "Mock sync down successful",
		Timestamp:     m.LastDown,
	}, nil
}

// GetStatus mocks getting sync status
func (m *MockProvider) GetStatus(ctx context.Context, claims *auth.Claims) (*SyncStatusResponse, error) {
	return &SyncStatusResponse{
		LastSyncUp:   m.LastUp,
		LastSyncDown: m.LastDown,
		Status:       SyncStatusSuccess,
		PendingCount: 0,
	}, nil
}
