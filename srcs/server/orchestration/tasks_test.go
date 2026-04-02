package orchestration

import (
	"context"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

func TestClaimTask(t *testing.T) {
	ctx := context.Background()

	// Need a standalone SIPDB for local sqlite testing
	sipDB, err := NewSIPDB("sqlite://:memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sipDB.Close()

	// Initialise db migrations
	err = sipDB.db.RunMigrations(ctx)
	if err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	// Insert dummy task
	_, err = sipDB.db.Exec(ctx, `
		INSERT INTO swarm_tasks (id, mission_id, title, status, payload)
		VALUES ('task-1', 'mission-1', 'Test Task', 'PENDING', '{}')
	`)
	if err != nil {
		t.Fatalf("failed to insert dummy task: %v", err)
	}

	// Setup Hub and Server
	hub := NewHub()
	hub.SetSIPDB(sipDB)

	server := NewHubServiceServer(hub)

	// Claim Task request
	req := &pb.ClaimTaskRequest{
		TaskId: "task-1",
		AgentId: "agent-x",
	}

	resp, err := server.ClaimTask(ctx, req)
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if !resp.Success || resp.Task == nil {
		t.Fatalf("expected success and task returned")
	}

	if resp.Task.Status != "IN_PROGRESS" || resp.Task.AssignedAgentId != "agent-x" {
		t.Errorf("expected IN_PROGRESS and agent-x, got %s and %s", resp.Task.Status, resp.Task.AssignedAgentId)
	}

	// Second claim attempt should fail (concurrent locking check)
	req2 := &pb.ClaimTaskRequest{
		TaskId: "task-1",
		AgentId: "agent-y",
	}

	_, err = server.ClaimTask(ctx, req2)
	if err == nil {
		t.Fatalf("expected error claiming already locked task, got nil")
	}
}
