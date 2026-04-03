#!/bin/bash
# Apply patches to srcs/server/orchestration/tasks.go

sed -i 's/Status          string \/\/ PENDING, IN_PROGRESS, COMPLETED, FAILED/Status          string \/\/ PENDING, IN_PROGRESS, REVIEW, COMPLETED, FAILED/' srcs/server/orchestration/tasks.go

# Add ReviewTask method after CompleteTask
sed -i '/^\/\/ CompleteTask marks a task as completed./i \
// ReviewTask marks a task as ready for review.\n\
func (tm *TaskManager) ReviewTask(ctx context.Context, taskID, agentID string) error {\n\
	query := `\n\
		UPDATE swarm_tasks\n\
		SET status = '"'REVIEW'"', updated_at = CURRENT_TIMESTAMP\n\
		WHERE id = $1 AND assigned_agent_id = $2 AND status = '"'IN_PROGRESS'"'\n\
	`\n\
	res, err := tm.db.Exec(ctx, query, taskID, agentID)\n\
	if err != nil {\n\
		return fmt.Errorf("failed to review task: %w", err)\n\
	}\n\
\n\
	if res == 0 {\n\
		return errors.New("task not found or not assigned to agent in IN_PROGRESS state")\n\
	}\n\
\n\
	// Broadcast task review\n\
	if tm.hub != nil {\n\
		go func() {\n\
			tm.hub.PublishTaskBroadcast(taskID, map[string]interface{}{\n\
				"action":   "REVIEW",\n\
				"agent_id": agentID,\n\
				"status":   "REVIEW",\n\
			})\n\
		}()\n\
	}\n\
\n\
	var missionID string\n\
	err = tm.db.QueryRow(ctx, "SELECT mission_id FROM swarm_tasks WHERE id = $1", taskID).Scan(&missionID)\n\
	if err == nil {\n\
		telemetry.RecordSwarmTaskReviewed(ctx, missionID)\n\
	}\n\
\n\
	return nil\n\
}\n\
' srcs/server/orchestration/tasks.go
