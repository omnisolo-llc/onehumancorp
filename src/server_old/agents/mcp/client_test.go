package mcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"

	"github.com/onehumancorp/mono/src/server_old/telemetry"
)

func TestHybridContextTool(t *testing.T) {
	tool := &HybridContextTool{}
	ctx := context.Background()

	metricCalled := false
	var recordedMetric string
	var recordedPayload string
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		metricCalled = true
		recordedMetric = metricType
		recordedPayload = payload
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	payload := map[string]interface{}{
		"action": "click",
		"widget": "button",
	}

	res, err := tool.Execute(ctx, payload)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if !metricCalled {
		t.Errorf("Expected telemetry.BufferMetricFunc to be called")
	}
	if recordedMetric != "hybrid_ui_context" {
		t.Errorf("Expected metric 'hybrid_ui_context', got %s", recordedMetric)
	}

	expectedPayload, _ := json.Marshal(payload)
	if string(expectedPayload) != recordedPayload {
		t.Errorf("Expected payload %s, got %s", string(expectedPayload), recordedPayload)
	}

	if res == nil {
		t.Fatalf("Expected execution result, got nil")
	}
	if res.ToolID != "hybrid_context" {
		t.Errorf("Expected ToolID 'hybrid_context', got %s", res.ToolID)
	}
	if res.Status != "success" {
		t.Errorf("Expected status 'success', got %s", res.Status)
	}
	if res.HybridEscalation != false {
		t.Errorf("Expected HybridEscalation to be false, got %v", res.HybridEscalation)
	}
}

func TestLocalFSSyncTool(t *testing.T) {
	tool := &LocalFSSyncTool{}
	ctx := context.Background()

	metricCalled := false
	var recordedMetric string
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		metricCalled = true
		recordedMetric = metricType
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	err := os.MkdirAll(".agent-task/test-sandbox", 0755)
	if err != nil {
		t.Fatalf("Failed to create test dir: %v", err)
	}
	defer os.RemoveAll(".agent-task/test-sandbox")

	payloadWrite := map[string]interface{}{
		"Action":  "write",
		"Path":    ".agent-task/test-sandbox/test.txt",
		"Content": "hello world",
	}
	resWrite, err := tool.Execute(ctx, payloadWrite)
	if err != nil {
		t.Fatalf("Expected no error for write, got %v", err)
	}
	if resWrite.Status != "success" {
		t.Errorf("Expected write status success, got %s", resWrite.Status)
	}
	if !metricCalled || recordedMetric != "local_fs_sync" {
		t.Errorf("Expected telemetry.BufferMetricFunc to be called with local_fs_sync")
	}

	payloadRead := map[string]interface{}{
		"Action": "read",
		"Path":   ".agent-task/test-sandbox/test.txt",
	}
	resRead, err := tool.Execute(ctx, payloadRead)
	if err != nil {
		t.Fatalf("Expected no error for read, got %v", err)
	}
	if string(resRead.ResultData) != "hello world" {
		t.Errorf("Expected read content hello world, got %s", string(resRead.ResultData))
	}

	payloadSync := map[string]interface{}{
		"Action": "sync",
		"Path":   ".agent-task/test-sandbox/test.txt",
	}
	resSync, err := tool.Execute(ctx, payloadSync)
	if err != nil {
		t.Fatalf("Expected no error for sync, got %v", err)
	}
	if resSync.Status != "success" {
		t.Errorf("Expected sync status success, got %s", resSync.Status)
	}

	payloadInvalid := map[string]interface{}{
		"Action": "delete",
		"Path":   ".agent-task/test-sandbox/test.txt",
	}
	_, err = tool.Execute(ctx, payloadInvalid)
	if err == nil {
		t.Errorf("Expected error for invalid action")
	}

	payloadInvalidPath := map[string]interface{}{
		"Action": "read",
		"Path":   "outside/test.txt",
	}
	_, err = tool.Execute(ctx, payloadInvalidPath)
	if err == nil {
		t.Errorf("Expected error for invalid path")
	}

	payloadReadErr := map[string]interface{}{
		"Action": "read",
		"Path":   ".agent-task/test-sandbox/not_exist.txt",
	}
	_, err = tool.Execute(ctx, payloadReadErr)
	if err == nil {
		t.Errorf("Expected error for read non-exist")
	}

	payloadSyncErr := map[string]interface{}{
		"Action": "sync",
		"Path":   ".agent-task/test-sandbox/not_exist.txt",
	}
	_, err = tool.Execute(ctx, payloadSyncErr)
	if err == nil {
		t.Errorf("Expected error for sync non-exist")
	}

	err = os.MkdirAll(".agent-task/test-sandbox/dir", 0755)
	if err == nil {
		payloadWriteErr := map[string]interface{}{
			"Action":  "write",
			"Path":    ".agent-task/test-sandbox/dir",
			"Content": "hello world",
		}
		_, err = tool.Execute(ctx, payloadWriteErr)
		if err == nil {
			t.Errorf("Expected error for write on dir")
		}
	}

	// test path traversal
	payloadTraversal := map[string]interface{}{
		"Action": "read",
		"Path":   ".agent-task/../etc/passwd",
	}
	_, err = tool.Execute(ctx, payloadTraversal)
	if err == nil {
		t.Errorf("Expected error for path traversal")
	}

	payloadTraversal2 := map[string]interface{}{
		"Action": "read",
		"Path":   ".agent-task/../../outside.txt",
	}
	_, err = tool.Execute(ctx, payloadTraversal2)
	if err == nil {
		t.Errorf("Expected error for path traversal 2")
	}
}
