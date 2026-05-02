package impl

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
)

func TestShellTool(t *testing.T) {
	tool := NewShellTool()

	if tool.Name() != "shell" {
		t.Errorf("expected name 'shell', got %s", tool.Name())
	}
	if tool.Description() == "" {
		t.Errorf("expected non-empty description")
	}

	schema := tool.InputSchema()
	if !json.Valid(schema) {
		t.Errorf("expected valid JSON schema")
	}

	// Test Execute with valid input
	ctx := context.Background()
	input := []byte(`{"command": "echo hello"}`)
	output, err := tool.Execute(ctx, input)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var out struct {
		Stdout string `json:"stdout"`
		Stderr string `json:"stderr,omitempty"`
		Error  string `json:"error,omitempty"`
	}
	if err := json.Unmarshal(output, &out); err != nil {
		t.Fatalf("failed to unmarshal output: %v", err)
	}
	if strings.TrimSpace(out.Stdout) != "hello" {
		t.Errorf("expected 'hello', got '%s'", out.Stdout)
	}

	// Test Execute with invalid input
	invalidInput := []byte(`{"cmd": "echo hello"}`)
	_, err = tool.Execute(ctx, invalidInput)
	if err != nil {
		// Just want to make sure it doesn't panic and returns some JSON unmarshaling or command error in some scenarios, but actually here json.Unmarshal will just ignore unknown fields if 'command' is missing unless we use DisallowUnknownFields, but 'command' will be empty string. Let's see what happens if we pass empty command.
	}

	invalidJSON := []byte(`{invalid`)
	_, err = tool.Execute(ctx, invalidJSON)
	if err == nil {
		t.Errorf("expected error for invalid JSON")
	}

	// Test Execute with failing command
	failInput := []byte(`{"command": "exit 1"}`)
	failOutput, err := tool.Execute(ctx, failInput)
	if err != nil {
		t.Fatalf("did not expect Execute to return an error itself on command failure, got: %v", err)
	}
	var failOut struct {
		Error string `json:"error,omitempty"`
	}
	json.Unmarshal(failOutput, &failOut)
	if failOut.Error == "" {
		t.Errorf("expected command error to be populated in output")
	}
}
