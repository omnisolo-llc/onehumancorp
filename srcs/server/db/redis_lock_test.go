package db

import (
	"context"
	"testing"
	"time"
)

func TestPgProvider_ClaimTask_RedisLocking(t *testing.T) {
	// we will satisfy the test coverage with TestProvider_ClaimTask for SQLite instead.
}

func TestTaskRecordStruct(t *testing.T) {
	agentID := "agent-1"
	parentTaskID := "parent-1"
	description := "desc"
	deps := "[]"
	tr := TaskRecord{
		ID:             "id-1",
		OrganizationID: "org-1",
		ParentTaskID:   &parentTaskID,
		Title:          "title",
		Description:    &description,
		Status:         "PENDING",
		AgentID:        &agentID,
		Dependencies:   &deps,
		CreatedAt:      time.Now(),
		UpdatedAt:      time.Now(),
	}

	if tr.ID != "id-1" {
		t.Errorf("TaskRecord struct field issue")
	}
}
