<<<<<<< SEARCH
// CompleteTask marks a task as completed.
func (m *TaskManager) CompleteTask(ctx context.Context, taskID string, missionID string) error {
	query := `
		UPDATE swarm_tasks
		SET status = 'COMPLETED', updated_at = NOW()
		WHERE id = $1
	`
=======
// CompleteTask marks a task as completed.
func (m *TaskManager) CompleteTask(ctx context.Context, taskID string, missionID string) error {
	query := `
		UPDATE swarm_tasks
		SET status = 'COMPLETED', updated_at = NOW()
		WHERE id = $1
	`
>>>>>>> REPLACE
