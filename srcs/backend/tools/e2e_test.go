package tools_test

import (
	"context"
	"encoding/json"
	"path/filepath"
	"strings"
	"testing"

	"onehumancorp/srcs/backend/tools/impl"
	"onehumancorp/srcs/backend/tools/registry"
)

// TestAgentToolFlowE2E tests the end-to-end flow of an agent discovering
// tools from the registry and executing them.
func TestAgentToolFlowE2E(t *testing.T) {
	// 1. Initialize the Unified Tool Registry
	reg := registry.NewUnifiedToolRegistry()

	// 2. Register tools
	shellTool := impl.NewShellTool()
	err := reg.RegisterTool(shellTool)
	if err != nil {
		t.Fatalf("failed to register shell tool: %v", err)
	}

	fileReadTool := impl.NewFileReadTool()
	err = reg.RegisterTool(fileReadTool)
	if err != nil {
		t.Fatalf("failed to register file_read tool: %v", err)
	}

	// 3. Agent discovers tools
	manifests := reg.ListTools()
	if len(manifests) != 2 {
		t.Fatalf("expected 2 tools registered, got %d", len(manifests))
	}

	// Ensure our requested tools are present
	toolNames := make(map[string]bool)
	for _, m := range manifests {
		toolNames[m.Name] = true
		// Agents would normally parse the JSON schema here to understand how to format their output
		if !json.Valid(m.InputSchema) {
			t.Errorf("tool %s has invalid JSON schema", m.Name)
		}
	}

	if !toolNames["shell"] || !toolNames["file_read"] {
		t.Errorf("missing expected tools in discovery")
	}

	ctx := context.Background()

	// 4. Agent executes a shell command to write a file
	// Pretend the agent found the shell tool and decided to use it based on a user request.
	agentSelectedToolName := "shell"
	_, exists := reg.GetTool(agentSelectedToolName)
	if !exists {
		t.Fatalf("agent selected tool %s but it was not found in registry", agentSelectedToolName)
	}

	dir := t.TempDir()
	testFilePath := filepath.Join(dir, "e2e_test.txt")
	testContent := "hello from agent"

	// Agent constructs JSON input according to the schema
	shellInput := []byte(`{"command": "echo '` + testContent + `' > ` + testFilePath + `"}`)

	// Agent executes the tool
	shellOutBytes, err := reg.ExecuteTool(ctx, agentSelectedToolName, shellInput)
	if err != nil {
		t.Fatalf("shell tool execution failed: %v", err)
	}

	var shellOutput struct {
		Error string `json:"error,omitempty"`
	}
	if err := json.Unmarshal(shellOutBytes, &shellOutput); err != nil {
		t.Fatalf("failed to unmarshal shell tool output: %v", err)
	}
	if shellOutput.Error != "" {
		t.Fatalf("shell command returned error: %s", shellOutput.Error)
	}

	// 5. Agent executes the file_read tool to verify the content
	agentSelectedToolName = "file_read"
	_, exists = reg.GetTool(agentSelectedToolName)
	if !exists {
		t.Fatalf("agent selected tool %s but it was not found in registry", agentSelectedToolName)
	}

	fileReadInput := []byte(`{"path": "` + testFilePath + `"}`)
	fileReadOutBytes, err := reg.ExecuteTool(ctx, agentSelectedToolName, fileReadInput)
	if err != nil {
		t.Fatalf("file_read tool execution failed: %v", err)
	}

	var fileReadOutput struct {
		Content string `json:"content"`
		Error   string `json:"error,omitempty"`
	}
	if err := json.Unmarshal(fileReadOutBytes, &fileReadOutput); err != nil {
		t.Fatalf("failed to unmarshal file_read tool output: %v", err)
	}

	if fileReadOutput.Error != "" {
		t.Fatalf("file_read tool returned error: %s", fileReadOutput.Error)
	}

	// We expect "hello from agent\n" from echo
	if strings.TrimSpace(fileReadOutput.Content) != testContent {
		t.Errorf("agent read incorrect file content. expected '%s', got '%s'", testContent, strings.TrimSpace(fileReadOutput.Content))
	}

	// Clean up - temp dir is automatically cleaned by test framework
	// Agent finished its job successfully!
}
