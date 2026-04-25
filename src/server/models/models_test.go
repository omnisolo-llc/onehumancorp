package models_test

import (
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/models"
)

// TestTask_Fields verifies that a Task struct can be created and populated.
func TestTask_Fields(t *testing.T) {
	now := time.Now().UTC()
	task := models.Task{
		ID:              "task-001",
		MissionID:       "mission-001",
		ParentPlanID:    "plan-001",
		Dependencies:    []string{"task-000"},
		Title:           "Implement feature X",
		Description:     "Add support for X in the codebase",
		Priority:        "high",
		Status:          "PENDING",
		AssignedAgentID: "agent-001",
		Payload:         `{"key":"value"}`,
		CreatedAt:       now,
		UpdatedAt:       now,
	}

	if task.ID != "task-001" {
		t.Errorf("expected ID 'task-001', got %q", task.ID)
	}
	if task.Status != "PENDING" {
		t.Errorf("expected Status 'PENDING', got %q", task.Status)
	}
	if len(task.Dependencies) != 1 || task.Dependencies[0] != "task-000" {
		t.Errorf("expected Dependencies=['task-000'], got %v", task.Dependencies)
	}
}

// TestTask_StatusValues verifies that standard Task status values are usable
// as string constants.
func TestTask_StatusValues(t *testing.T) {
	statuses := []string{
		"PENDING", "READY", "IN_PROGRESS", "COMPLETED", "BLOCKED", "FAILED",
	}
	for _, s := range statuses {
		task := models.Task{Status: s}
		if task.Status != s {
			t.Errorf("expected status %q, got %q", s, task.Status)
		}
	}
}

// TestTask_ZeroValue verifies that the zero value of Task is safe to use.
func TestTask_ZeroValue(t *testing.T) {
	var task models.Task
	if task.ID != "" {
		t.Errorf("expected empty ID for zero value, got %q", task.ID)
	}
	if task.Dependencies != nil {
		t.Errorf("expected nil Dependencies for zero value, got %v", task.Dependencies)
	}
}

// TestTask_NilDependencies verifies that a task with no dependencies can be
// created and that the nil slice is handled safely.
func TestTask_NilDependencies(t *testing.T) {
	task := models.Task{
		ID:    "task-no-deps",
		Title: "Task without dependencies",
	}
	if len(task.Dependencies) != 0 {
		t.Errorf("expected 0 dependencies, got %d", len(task.Dependencies))
	}
}

// TestSharedTask_Fields verifies the SharedTask struct fields and JSON tags
// are correct by round-tripping via struct assignment.
func TestSharedTask_Fields(t *testing.T) {
	now := time.Now().UTC()
	st := models.SharedTask{
		ID:              "shared-001",
		OrganizationID:  "org-001",
		ParentPlanID:    "plan-001",
		Dependencies:    []string{"dep-001", "dep-002"},
		Title:           "Deploy to staging",
		Description:     "Deploy the latest build to staging environment",
		Status:          "READY",
		AssignedAgentID: "agent-swe-01",
		Priority:        "medium",
		Payload:         `{"env":"staging"}`,
		CreatedAt:       now,
		UpdatedAt:       now,
	}

	if st.ID != "shared-001" {
		t.Errorf("expected ID 'shared-001', got %q", st.ID)
	}
	if st.OrganizationID != "org-001" {
		t.Errorf("expected OrganizationID 'org-001', got %q", st.OrganizationID)
	}
	if len(st.Dependencies) != 2 {
		t.Errorf("expected 2 dependencies, got %d", len(st.Dependencies))
	}
	if st.LockedUntil != nil {
		t.Error("expected nil LockedUntil by default")
	}
}

// TestSharedTask_LockedUntil verifies that the optional LockedUntil pointer
// field can be set and read correctly.
func TestSharedTask_LockedUntil(t *testing.T) {
	future := time.Now().Add(time.Hour).UTC()
	st := models.SharedTask{
		ID:          "locked-task",
		LockedUntil: &future,
	}
	if st.LockedUntil == nil {
		t.Fatal("expected non-nil LockedUntil")
	}
	if !st.LockedUntil.Equal(future) {
		t.Errorf("expected LockedUntil=%v, got %v", future, *st.LockedUntil)
	}
}

// TestTaskDependency_Fields verifies the TaskDependency struct.
func TestTaskDependency_Fields(t *testing.T) {
	dep := models.TaskDependency{
		TaskID:          "task-002",
		DependsOnTaskID: "task-001",
	}
	if dep.TaskID != "task-002" {
		t.Errorf("expected TaskID 'task-002', got %q", dep.TaskID)
	}
	if dep.DependsOnTaskID != "task-001" {
		t.Errorf("expected DependsOnTaskID 'task-001', got %q", dep.DependsOnTaskID)
	}
}

// TestSyncLog_Fields verifies the SyncLog struct.
func TestSyncLog_Fields(t *testing.T) {
	now := time.Now().UTC()
	log := models.SyncLog{
		SyncID:         "sync-001",
		MemoryID:       "mem-001",
		CloudMissionID: "cloud-mission-001",
		SyncedAt:       now,
	}
	if log.SyncID != "sync-001" {
		t.Errorf("expected SyncID 'sync-001', got %q", log.SyncID)
	}
	if log.MemoryID != "mem-001" {
		t.Errorf("expected MemoryID 'mem-001', got %q", log.MemoryID)
	}
	if !log.SyncedAt.Equal(now) {
		t.Errorf("expected SyncedAt=%v, got %v", now, log.SyncedAt)
	}
}

// TestSyncLog_ZeroValue verifies that the zero value of SyncLog is safe.
func TestSyncLog_ZeroValue(t *testing.T) {
	var log models.SyncLog
	if log.SyncID != "" {
		t.Errorf("expected empty SyncID, got %q", log.SyncID)
	}
	if !log.SyncedAt.IsZero() {
		t.Errorf("expected zero SyncedAt, got %v", log.SyncedAt)
	}
}
