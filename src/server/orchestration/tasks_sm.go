package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"time"
)

// TaskManager coordinates state transitions and queue/mesh interactions
type TaskManager struct {
	db    *sql.DB
	mesh  TeammateMesh
	queue SubAgentQueue
}

func NewTaskManager(db *sql.DB, mesh TeammateMesh, queue SubAgentQueue) *TaskManager {
	return &TaskManager{
		db:    db,
		mesh:  mesh,
		queue: queue,
	}
}

func (tm *TaskManager) TransitionTask(ctx context.Context, taskID string, fromState, toState string, payload []byte) error {
	// 1. Durably update state
	if tm.db != nil {
		_, err := tm.db.ExecContext(ctx, "INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, transitioned_at) VALUES (?, ?, ?, ?, ?)",
			fmt.Sprintf("tx-%d", time.Now().UnixNano()), taskID, fromState, toState, time.Now().Format(time.RFC3339))
		if err != nil {
			return fmt.Errorf("failed to persist transition: %w", err)
		}
	}

	fmt.Printf("Transitioning task %s from %s to %s\n", taskID, fromState, toState)

	// 2. Broadcast over new V2 Mesh
	msg := MeshMessage{
		AgentID:   "system",
		Channel:   fmt.Sprintf("task:%s:transitions", taskID),
		EventType: "STATE_CHANGED",
		Data:      payload,
	}

	if err := tm.mesh.Publish(ctx, msg); err != nil {
		return fmt.Errorf("failed to broadcast transition: %w", err)
	}

	return nil
}

func (tm *TaskManager) PollTasks(ctx context.Context) error {
	job, err := tm.queue.Dequeue(ctx)
	if err != nil {
		return err
	}
	if job == nil {
		return nil
	}

	// State Machine: PENDING -> IN_PROGRESS
	err = tm.TransitionTask(ctx, job.ParentTaskID, "PENDING", "IN_PROGRESS", job.Payload)
	if err != nil {
		return err
	}

	return nil
}
