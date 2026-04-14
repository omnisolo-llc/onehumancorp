import sys

with open("srcs/server/dashboard/handlers_mcp.go", "r") as f:
    content = f.read()

blob_case = """	case "blob-mcp":
		if s.hub.Storage() == nil {
			return nil, errors.New("storage provider not configured")
		}

		inspector := blobinspector.NewBlobInspectorMCP(s.hub.Storage())
		var params map[string]interface{}
		if err := json.Unmarshal(req.Params, &params); err != nil {
			return nil, fmt.Errorf("invalid blob-mcp parameters: %w", err)
		}

		// In a real execution environment, the HTTP middleware sets context values for auth.
		// However, for MCP tool invocation inside the server loop, we recreate claims if known.
		// For simplicity we create a dummy claim just for testing out the cloud mode scoping.
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
		}, nil"""

hybrid_fs_case = """
	// ── Hybrid File System tool ───────────────────────────────────────────────
	case "hybridfs-mcp":
		// NOTE: In a real execution environment, we should check if we are in local or cloud mode
		// and instantiate the correct provider. For now, we instantiate the local provider
		// if we're in local mode, or cloud provider otherwise. We don't have direct access
		// to the global mode here so we fallback to a safe default (cloud).

		var fsProvider hybridfsmcp.FileSystemProvider
		if s.hub.Storage() != nil && s.hub.Storage().IsLocal() {
			fsProvider = hybridfsmcp.NewLocalFSProvider("./")
		} else {
			fsProvider = hybridfsmcp.NewCloudFSProvider("/tmp/ohc-cloud-fs")
		}

		inspector := hybridfsmcp.NewHybridFSMCP(fsProvider)
		var params map[string]interface{}
		if err := json.Unmarshal(req.Params, &params); err != nil {
			return nil, fmt.Errorf("invalid hybridfs-mcp parameters: %w", err)
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
		}, nil"""

if blob_case in content:
    content = content.replace(blob_case, blob_case + hybrid_fs_case)
    with open("srcs/server/dashboard/handlers_mcp.go", "w") as f:
        f.write(content)
    print("Success")
else:
    print("Failed to find blob_case")
