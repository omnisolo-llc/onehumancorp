package hybridcrdtmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type CRDTTool struct {
	db db.Provider
}

func NewCRDTTool(database db.Provider) *CRDTTool {
	return &CRDTTool{db: database}
}

func (t *CRDTTool) Register(s *server.MCPServer) {
	s.AddTool(t.createPullTool(), t.handlePull)
	s.AddTool(t.createPushTool(), t.handlePush)
	s.AddTool(t.createMergeTool(), t.handleMerge)
}

func (t *CRDTTool) createPullTool() mcp.Tool {
	return mcp.Tool{
		Name:        "crdt_pull",
		Description: "Fetch the latest CRDT state vector for a given entity from the Cloud backend (or return local if standalone).",
		InputSchema: mcp.ToolInputSchema{
			Type: "object",
			Properties: map[string]interface{}{
				"entity_id": map[string]interface{}{
					"type":        "string",
					"description": "The ID of the entity to pull the CRDT vector for.",
				},
			},
			Required: []string{"entity_id"},
		},
	}
}

func (t *CRDTTool) createPushTool() mcp.Tool {
	return mcp.Tool{
		Name:        "crdt_push",
		Description: "Submit local CRDT mutations to the Cloud backend.",
		InputSchema: mcp.ToolInputSchema{
			Type: "object",
			Properties: map[string]interface{}{
				"entity_id": map[string]interface{}{
					"type":        "string",
					"description": "The ID of the entity.",
				},
				"crdt_vector": map[string]interface{}{
					"type":        "string",
					"description": "The JSON serialized CRDT vector.",
				},
			},
			Required: []string{"entity_id", "crdt_vector"},
		},
	}
}

func (t *CRDTTool) createMergeTool() mcp.Tool {
	return mcp.Tool{
		Name:        "crdt_merge",
		Description: "Locally compute the intersection of state vectors.",
		InputSchema: mcp.ToolInputSchema{
			Type: "object",
			Properties: map[string]interface{}{
				"vector_a": map[string]interface{}{
					"type":        "string",
					"description": "First CRDT vector (JSON).",
				},
				"vector_b": map[string]interface{}{
					"type":        "string",
					"description": "Second CRDT vector (JSON).",
				},
			},
			Required: []string{"vector_a", "vector_b"},
		},
	}
}

func (t *CRDTTool) checkAuth(ctx context.Context) (string, error) {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"
	if isMultiTenant {
		orgID := auth.OrganizationIDFromContext(ctx)
		if orgID == "" {
			return "", errors.New("organization_id is required in multi-tenant mode")
		}
		return orgID, nil
	}
	return "", nil
}

func (t *CRDTTool) handlePull(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	orgID, err := t.checkAuth(ctx)
	if err != nil {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: err.Error()}}}, nil
	}

	args, ok := request.Params.Arguments.(map[string]interface{})
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid arguments format"}}}, nil
	}
	entityID, ok := args["entity_id"].(string)
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid entity_id"}}}, nil
	}

	var vectorStr string
	if orgID != "" {
		err = t.db.QueryRow(ctx, "SELECT COALESCE(crdt_vector::text, '{}') FROM shared_tasks WHERE id = $1 AND organization_id = $2", entityID, orgID).Scan(&vectorStr)
	} else {
		err = t.db.QueryRow(ctx, "SELECT COALESCE(crdt_vector, '{}') FROM shared_tasks WHERE id = $1", entityID).Scan(&vectorStr)
	}

	if err != nil {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "entity not found or error fetching vector: " + err.Error()}}}, nil
	}

	return &mcp.CallToolResult{
		Content: []mcp.Content{mcp.TextContent{Type: "text", Text: vectorStr}},
	}, nil
}

func (t *CRDTTool) handlePush(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	orgID, err := t.checkAuth(ctx)
	if err != nil {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: err.Error()}}}, nil
	}

	args, ok := request.Params.Arguments.(map[string]interface{})
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid arguments format"}}}, nil
	}
	entityID, ok := args["entity_id"].(string)
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid entity_id"}}}, nil
	}

	vectorStr, ok := args["crdt_vector"].(string)
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid crdt_vector"}}}, nil
	}

    if !json.Valid([]byte(vectorStr)) {
        return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "crdt_vector must be valid json"}}}, nil
    }

	if orgID != "" {
		_, err = t.db.Exec(ctx, "UPDATE shared_tasks SET crdt_vector = $1 WHERE id = $2 AND organization_id = $3", vectorStr, entityID, orgID)
	} else {
		_, err = t.db.Exec(ctx, "UPDATE shared_tasks SET crdt_vector = $1 WHERE id = $2", vectorStr, entityID)
	}

	if err != nil {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "error updating vector: " + err.Error()}}}, nil
	}

	return &mcp.CallToolResult{
		Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "success"}},
	}, nil
}

func (t *CRDTTool) handleMerge(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args, ok := request.Params.Arguments.(map[string]interface{})
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid arguments format"}}}, nil
	}
	vectorAStr, ok := args["vector_a"].(string)
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid vector_a"}}}, nil
	}

	vectorBStr, ok := args["vector_b"].(string)
	if !ok {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "invalid vector_b"}}}, nil
	}

	var mapA, mapB map[string]int
	if err := json.Unmarshal([]byte(vectorAStr), &mapA); err != nil {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "error parsing vector_a: " + err.Error()}}}, nil
	}
	if err := json.Unmarshal([]byte(vectorBStr), &mapB); err != nil {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "error parsing vector_b: " + err.Error()}}}, nil
	}

	merged := make(map[string]int)
	for k, v := range mapA {
		merged[k] = v
	}
	for k, v := range mapB {
		if existing, ok := merged[k]; ok {
			if v > existing {
				merged[k] = v
			}
		} else {
			merged[k] = v
		}
	}

	mergedBytes, err := json.Marshal(merged)
	if err != nil {
		return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{mcp.TextContent{Type: "text", Text: "error serializing merged vector: " + err.Error()}}}, nil
	}

	return &mcp.CallToolResult{
		Content: []mcp.Content{mcp.TextContent{Type: "text", Text: string(mergedBytes)}},
	}, nil
}

func (t *CRDTTool) CallTool(ctx context.Context, action string, params map[string]interface{}) (interface{}, error) {
	req := mcp.CallToolRequest{}
	req.Params.Arguments = params

	switch action {
	case "crdt_pull":
		res, err := t.handlePull(ctx, req)
		if err != nil {
			return nil, err
		}
		if res.IsError {
			return nil, errors.New(fmt.Sprint(res.Content))
		}
		return res.Content, nil
	case "crdt_push":
		res, err := t.handlePush(ctx, req)
		if err != nil {
			return nil, err
		}
		if res.IsError {
			return nil, errors.New(fmt.Sprint(res.Content))
		}
		return res.Content, nil
	case "crdt_merge":
		res, err := t.handleMerge(ctx, req)
		if err != nil {
			return nil, err
		}
		if res.IsError {
			return nil, errors.New(fmt.Sprint(res.Content))
		}
		return res.Content, nil
	default:
		return nil, errors.New("unknown action: " + action)
	}
}
