package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/dashboard/handlers_mcp.go")
	if err != nil {
		log.Fatal(err)
	}

	searchBlock := `	// ── Hybrid File System tool ───────────────────────────────────────────────
	case "hybrid-fs-mcp":
		baseDir := "/tmp" // Or load from config/env
		isCloud := s.hub.DB() != nil && !s.hub.DB().IsSQLite()
		fsProvider := hybridfsmcp.NewProviderFactory(isCloud, baseDir)
		inspector := hybridfsmcp.NewHybridFSMCP(fsProvider)

		var params map[string]interface{}
		if err := json.Unmarshal(req.Params, &params); err != nil {
			return nil, fmt.Errorf("invalid hybrid-fs-mcp parameters: %w", err)
		}

		claims := &auth.Claims{
			OrganizationID: s.org.ID,
		}
		ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

		res, err := inspector.CallTool(ctx, req.Action, params)
		if err != nil {
			return nil, err
		}

		return map[string]any{
			"result":           res,
			"HybridEscalation": true,
		}, nil`

	replaceBlock := `	// ── Hybrid File System tool ───────────────────────────────────────────────
	case "hybrid-fs-mcp":
		baseDir := s.cfg.DataDir
		if baseDir == "" {
			baseDir = "/tmp/ohc_workspace"
		}
		isCloud := s.hub.DB() != nil && !s.hub.DB().IsSQLite()
		fsProvider := hybridfsmcp.NewProviderFactory(isCloud, baseDir)
		inspector := hybridfsmcp.NewHybridFSMCP(fsProvider)

		var params map[string]interface{}
		if err := json.Unmarshal(req.Params, &params); err != nil {
			return nil, fmt.Errorf("invalid hybrid-fs-mcp parameters: %w", err)
		}

		res, err := inspector.CallTool(r.Context(), req.Action, params)
		if err != nil {
			return nil, err
		}

		return map[string]any{
			"result":           res,
			"HybridEscalation": true,
		}, nil`

	content = bytes.Replace(content, []byte(searchBlock), []byte(replaceBlock), 1)

	searchBlobBlock := `		// In a real execution environment, the HTTP middleware sets context values for auth.
		// However, for MCP tool invocation inside the server loop, we recreate claims if known.
		// For simplicity we create a dummy claim just for testing out the cloud mode scoping.
		claims := &auth.Claims{
			OrganizationID: s.org.ID,
		}

		ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)
		res, err := inspector.CallTool(ctx, req.Action, params)`

	replaceBlobBlock := `		res, err := inspector.CallTool(r.Context(), req.Action, params)`

	content = bytes.Replace(content, []byte(searchBlobBlock), []byte(replaceBlobBlock), 1)

	if err := ioutil.WriteFile("srcs/server/dashboard/handlers_mcp.go", content, 0644); err != nil {
		log.Fatal(err)
	}
	fmt.Println("Patch applied successfully.")
}
