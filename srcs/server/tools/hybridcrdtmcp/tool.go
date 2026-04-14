package hybridcrdtmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// HybridCRDTMCP implements the MCP interface for CRDT state synchronization.
type HybridCRDTMCP struct {
	dbWrapper *db.DB
}

// NewHybridCRDTMCP creates a new HybridCRDTMCP instance.
func NewHybridCRDTMCP(dbWrapper *db.DB) *HybridCRDTMCP {
	return &HybridCRDTMCP{
		dbWrapper: dbWrapper,
	}
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
		return m.PullHandler(ctx, input)
	case "crdt_push":
		return m.PushHandler(ctx, input)
	case "crdt_merge":
		return m.MergeHandler(ctx, input)
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

func (m *HybridCRDTMCP) PullHandler(ctx context.Context, input json.RawMessage) (interface{}, error) {
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

	vector := map[string]interface{}{}

	if m.dbWrapper != nil {
		var crdtVector string
		query := "SELECT crdt_vector FROM shared_tasks WHERE id = $1 AND organization_id = $2"

		err := m.dbWrapper.QueryRow(ctx, query, args.EntityID, orgID).Scan(&crdtVector)
		if err == nil && crdtVector != "" {
			_ = json.Unmarshal([]byte(crdtVector), &vector)
		}
	}

	return map[string]interface{}{
		"status": "pulled",
		"vector": vector,
	}, nil
}

func (m *HybridCRDTMCP) PushHandler(ctx context.Context, input json.RawMessage) (interface{}, error) {
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

	if m.dbWrapper != nil {
		var crdtVectorStr string
		query := "SELECT crdt_vector FROM shared_tasks WHERE id = $1 AND organization_id = $2"
		err := m.dbWrapper.QueryRow(ctx, query, args.EntityID, orgID).Scan(&crdtVectorStr)

		currentVector := make(map[string]interface{})
		if err == nil && crdtVectorStr != "" {
			_ = json.Unmarshal([]byte(crdtVectorStr), &currentVector)
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

		newVectorBytes, _ := json.Marshal(currentVector)

		updateQuery := "UPDATE shared_tasks SET crdt_vector = $1 WHERE id = $2 AND organization_id = $3"
		if m.dbWrapper.IsSQLite() {
			updateQuery = "UPDATE shared_tasks SET crdt_vector = ? WHERE id = ? AND organization_id = ?"
		}
		_, _ = m.dbWrapper.Exec(ctx, updateQuery, string(newVectorBytes), args.EntityID, orgID)
	}

	return map[string]interface{}{
		"status": "pushed",
	}, nil
}

func (m *HybridCRDTMCP) MergeHandler(ctx context.Context, input json.RawMessage) (interface{}, error) {
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
