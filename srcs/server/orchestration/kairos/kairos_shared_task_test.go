package kairos

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/db"
	"testing"
	"time"
)

func TestKairosSharedTaskRepo(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create the table just like the other tests do, in case migrations drop it.
	_, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME,
            action_risk TEXT,
            approval_status TEXT
        );
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewSharedTaskRepo(provider)
	task := &SharedTask{
		ID:             "test-uuid",
		AgentID:        "agent-1",
		Status:         "PENDING",
		Payload:        []byte(`{"hello":"world"}`),
		ActionRisk:     "HIGH",
		ApprovalStatus: "PENDING",
		CreatedAt:      time.Now().Truncate(time.Second).UTC(),
	}

	if err := repo.Insert(ctx, task); err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	fetched, err := repo.Get(ctx, task.ID)
	if err != nil {
		t.Fatalf("failed to get: %v", err)
	}

	if fetched.ID != task.ID || fetched.AgentID != task.AgentID || fetched.Status != task.Status {
		t.Errorf("mismatch: %+v != %+v", fetched, task)
	}
	if string(fetched.Payload) != string(task.Payload) {
		t.Errorf("payload mismatch: %s != %s", string(fetched.Payload), string(task.Payload))
	}
	if fetched.ActionRisk != task.ActionRisk {
		t.Errorf("action_risk mismatch: %s != %s", fetched.ActionRisk, task.ActionRisk)
	}
	if fetched.ApprovalStatus != task.ApprovalStatus {
		t.Errorf("approval_status mismatch: %s != %s", fetched.ApprovalStatus, task.ApprovalStatus)
	}
	if !fetched.CreatedAt.Equal(task.CreatedAt) {
		t.Errorf("created_at mismatch: %v != %v", fetched.CreatedAt, task.CreatedAt)
	}
}

func TestUpdateApprovalStatus(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME,
            action_risk TEXT,
            approval_status TEXT
        );
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewSharedTaskRepo(provider)
	task := &SharedTask{
		ID:             "test-uuid-2",
		AgentID:        "agent-1",
		Status:         "PENDING",
		ActionRisk:     "HIGH",
		ApprovalStatus: "PENDING",
		Payload:        []byte(`{"hello":"world"}`),
		CreatedAt:      time.Now().Truncate(time.Second).UTC(),
	}

	if err := repo.Insert(ctx, task); err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	if err := repo.UpdateApprovalStatus(ctx, task.ID, "APPROVED"); err != nil {
		t.Fatalf("failed to update approval status: %v", err)
	}

	fetched, err := repo.Get(ctx, task.ID)
	if err != nil {
		t.Fatalf("failed to get: %v", err)
	}

	if fetched.ApprovalStatus != "APPROVED" {
		t.Errorf("expected approval_status APPROVED, got %s", fetched.ApprovalStatus)
	}
}
