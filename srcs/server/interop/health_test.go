package interop

import (
	"context"
	"os"
	"testing"
	"time"
	"github.com/redis/rueidis"
)

func TestMemoryHealthMonitor_ReportAndCheck(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	monitor, err := NewHealthMonitor()
	if err != nil {
		t.Fatalf("Failed to create memory health monitor: %v", err)
	}

	ctx := context.Background()
	agentID := "agent_123"
	status := "ALIVE"

	err = monitor.ReportHealth(ctx, agentID, status)
	if err != nil {
		t.Fatalf("ReportHealth failed: %v", err)
	}

	checkedStatus, err := monitor.CheckHealth(ctx, agentID)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if checkedStatus != status {
		t.Errorf("Expected status %s, got %s", status, checkedStatus)
	}
}

func TestMemoryHealthMonitor_NotFound(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	monitor, _ := NewHealthMonitor()
	ctx := context.Background()

	_, err := monitor.CheckHealth(ctx, "unknown_agent")
	if err == nil {
		t.Error("Expected error when checking unknown agent")
	}
}

func TestMemoryHealthMonitor_Dead(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	monitor, _ := NewHealthMonitor()
	memMonitor := monitor.(*MemoryHealthMonitor)
	ctx := context.Background()

	agentID := "agent_dead"
	memMonitor.statuses[agentID] = healthEntry{
		status:    "ALIVE",
		timestamp: time.Now().Add(-6 * time.Minute), // More than 5 mins ago
	}

	status, err := memMonitor.CheckHealth(ctx, agentID)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}
	if status != "DEAD" {
		t.Errorf("Expected status DEAD for expired health check, got %s", status)
	}
}

func TestCloudHealthMonitor_Fallback(t *testing.T) {
	os.Setenv("REDIS_URL", "invalid_url")
	os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("REDIS_URL")

	monitor, err := NewHealthMonitor()
	if err != nil {
		t.Fatalf("Expected fallback to succeed, got error: %v", err)
	}

	if _, ok := monitor.(*MemoryHealthMonitor); !ok {
		t.Errorf("Expected fallback to MemoryHealthMonitor, got %T", monitor)
	}
}

func TestCloudHealthMonitor_ReportAndCheck(t *testing.T) {
	mockClient, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{"127.0.0.1:6379"}})
	if err != nil {
		t.Skip("Skipping cloud test because redis is not available locally")
	}
	defer mockClient.Close()

	err = mockClient.Do(context.Background(), mockClient.B().Ping().Build()).Error()
	if err != nil {
		t.Skip("Skipping cloud test because redis is not responding")
	}

	monitor := &CloudHealthMonitor{client: mockClient}
	ctx := context.Background()
	agentID := "agent_cloud_1"
	expectedStatus := "ALIVE"

	err = monitor.ReportHealth(ctx, agentID, expectedStatus)
	if err != nil {
		t.Fatalf("ReportHealth failed: %v", err)
	}

	status, err := monitor.CheckHealth(ctx, agentID)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if status != expectedStatus {
		t.Errorf("Expected status %s, got %s", expectedStatus, status)
	}

	// Test checking a non-existent/expired key
	deadStatus, err := monitor.CheckHealth(ctx, "non_existent_cloud_agent")
	if err != nil {
		t.Fatalf("CheckHealth failed for non-existent agent: %v", err)
	}
	if deadStatus != "DEAD" {
		t.Errorf("Expected DEAD status for non-existent agent, got %s", deadStatus)
	}
}
