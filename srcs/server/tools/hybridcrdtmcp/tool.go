package hybridcrdtmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// In-memory mock database for CRDT vectors (keyed by orgID -> entityID)
var (
	mockDB      = make(map[string]map[string]map[string]interface{})
	mockDBMutex sync.RWMutex
)

// HybridCRDTMCP implements the MCP interface for CRDT state synchronization.
type HybridCRDTMCP struct {
}

// NewHybridCRDTMCP creates a new HybridCRDTMCP instance.
func NewHybridCRDTMCP() *HybridCRDTMCP {
	return &HybridCRDTMCP{}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *HybridCRDTMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "crdt_pull",
			Description: "Fetch the latest CRDT state vector for a given entity.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"entity_id": {"type": "string"}}, "required": ["entity_id"]}`),
		},
		{
			Name:        "crdt_push",
			Description: "Submit local CRDT mutations to the Cloud backend.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"entity_id": {"type": "string"}, "vector": {"type": "object"}}, "required": ["entity_id", "vector"]}`),
		},
		{
			Name:        "crdt_merge",
			Description: "Locally compute the intersection of state vectors.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"local_vector": {"type": "object"}, "remote_vector": {"type": "object"}}, "required": ["local_vector", "remote_vector"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridCRDTMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	input, err := json.Marshal(arguments)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal arguments: %w", err)
	}

	switch toolName {
	case "crdt_pull":
		return PullHandler(ctx, input)
	case "crdt_push":
		return PushHandler(ctx, input)
	case "crdt_merge":
		return MergeHandler(ctx, input)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func getOrgIDFromContext(ctx context.Context) (string, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil {
			claims, _ = ctx.Value(auth.ClaimsContextKeyForTest).(*auth.Claims)
			if claims == nil {
				return "", fmt.Errorf("missing authentication claims")
			}
		}
		if claims.OrganizationID == "" {
			return "", fmt.Errorf("missing organization_id in claims")
		}
		return claims.OrganizationID, nil
	}
	return "local-standalone-org", nil
}

func PullHandler(ctx context.Context, input json.RawMessage) (interface{}, error) {
	orgID, err := getOrgIDFromContext(ctx)
	if err != nil {
		return nil, err
	}

	var args struct {
		EntityID string `json:"entity_id"`
	}
	if err := json.Unmarshal(input, &args); err != nil {
		return nil, fmt.Errorf("invalid input: %w", err)
	}

	mockDBMutex.RLock()
	defer mockDBMutex.RUnlock()

	vector := map[string]interface{}{}
	if orgData, ok := mockDB[orgID]; ok {
		if entityVector, ok := orgData[args.EntityID]; ok {
			vector = entityVector
		}
	}

	return map[string]interface{}{
		"status": "pulled",
		"vector": vector,
	}, nil
}

func PushHandler(ctx context.Context, input json.RawMessage) (interface{}, error) {
	orgID, err := getOrgIDFromContext(ctx)
	if err != nil {
		return nil, err
	}

	var args struct {
		EntityID string                 `json:"entity_id"`
		Vector   map[string]interface{} `json:"vector"`
	}
	if err := json.Unmarshal(input, &args); err != nil {
		return nil, fmt.Errorf("invalid input: %w", err)
	}

	mockDBMutex.Lock()
	defer mockDBMutex.Unlock()

	if _, ok := mockDB[orgID]; !ok {
		mockDB[orgID] = make(map[string]map[string]interface{})
	}

	// Simulate simple merge on server by updating keys where local clock > remote clock
	currentVector := mockDB[orgID][args.EntityID]
	if currentVector == nil {
		currentVector = make(map[string]interface{})
	}

	for k, v := range args.Vector {
		if curV, ok := currentVector[k]; ok {
			if vNum, ok1 := v.(float64); ok1 {
				if curNum, ok2 := curV.(float64); ok2 && vNum > curNum {
					currentVector[k] = v
				}
			}
		} else {
			currentVector[k] = v
		}
	}
	mockDB[orgID][args.EntityID] = currentVector

	return map[string]interface{}{
		"status": "pushed",
	}, nil
}

func MergeHandler(ctx context.Context, input json.RawMessage) (interface{}, error) {
	var args struct {
		LocalVector  map[string]interface{} `json:"local_vector"`
		RemoteVector map[string]interface{} `json:"remote_vector"`
	}
	if err := json.Unmarshal(input, &args); err != nil {
		return nil, fmt.Errorf("invalid input: %w", err)
	}

	mergedVector := make(map[string]interface{})

	for k, v := range args.LocalVector {
		mergedVector[k] = v
	}

	for k, v := range args.RemoteVector {
		if curV, ok := mergedVector[k]; ok {
			if vNum, ok1 := v.(float64); ok1 {
				if curNum, ok2 := curV.(float64); ok2 && vNum > curNum {
					mergedVector[k] = v
				}
			}
		} else {
			mergedVector[k] = v
		}
	}

	return map[string]interface{}{
		"status": "merged",
		"vector": mergedVector,
	}, nil
}
