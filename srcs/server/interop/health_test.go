package interop

import (
	"context"
	"testing"
	"time"
)

func TestHealthMonitor(t *testing.T) {
	baseMesh := NewTeammateMeshWithClient(nil)
	monitor, err := NewHealthMonitor(baseMesh)
	if err != nil {
		t.Fatalf("failed to create health monitor: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	agentID := "agent-123"

	// Start responder
	err = monitor.StartResponder(ctx, agentID)
	if err != nil {
		t.Fatalf("failed to start responder: %v", err)
	}

	// Wait a tiny bit for subscriber to be ready
	time.Sleep(50 * time.Millisecond)

	// Ping
	status, err := monitor.Ping(ctx, agentID)
	if err != nil {
		t.Fatalf("ping failed: %v", err)
	}

	if status.AgentId != agentID {
		t.Fatalf("expected agent %s, got %s", agentID, status.AgentId)
	}
	if status.Status != "healthy" {
		t.Fatalf("expected healthy, got %s", status.Status)
	}
}

func TestHealthMonitor_Timeout(t *testing.T) {
	baseMesh := NewTeammateMeshWithClient(nil)
	monitor, err := NewHealthMonitor(baseMesh)
	if err != nil {
		t.Fatalf("failed to create health monitor: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 500*time.Millisecond) // Short timeout
	defer cancel()

	// Ping non-existent agent
	_, err = monitor.Ping(ctx, "ghost-agent")
	if err == nil {
		t.Fatalf("expected ping to fail with timeout")
	}
}
