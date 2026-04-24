1. **Fix duplicate declarations in test files.**
   - Execute the following python script via `run_in_bash_session` to fix duplicate mock declarations and undefined methods:
   ```python
   import re
   with open("srcs/server/orchestration/autodream_test.go", "r") as f:
       c = f.read()
   c = c.replace("type mockPgProvider struct {", "type mockPgProviderAutoDreamTest struct {")
   c = c.replace("mockPgProvider) IsSQLite", "mockPgProviderAutoDreamTest) IsSQLite")
   c = c.replace("mockPgProvider{}", "mockPgProviderAutoDreamTest{}")
   with open("srcs/server/orchestration/autodream_test.go", "w") as f:
       f.write(c)

   with open("srcs/server/orchestration/tasks_db_test.go", "r") as f:
       c = f.read()
   c = c.replace("type mockPGProvider struct {", "type mockPGProviderTasksDBTest struct {")
   c = c.replace("mockPGProvider) IsSQLite", "mockPGProviderTasksDBTest) IsSQLite")
   c = c.replace("mockPGProvider{}", "mockPGProviderTasksDBTest{}")
   with open("srcs/server/orchestration/tasks_db_test.go", "w") as f:
       f.write(c)

   with open("srcs/server/orchestration/service.go", "r") as f:
       c = f.read()
   c = re.sub(r'func \(s \*HubServiceServer\) PublishTeammateMeshEvent[\s\S]*?nil\n\}', '', c)
   c = re.sub(r'func \(s \*HubServiceServer\) StreamTeammateMesh[\s\S]*?\}\n\t\}\n\}', '', c)
   with open("srcs/server/orchestration/service.go", "w") as f:
       f.write(c)
   ```

2. **Integrate StateMachine into tasks packages.**
   - Execute the following python script via `run_in_bash_session` to replace hardcoded `UPDATE shared_tasks SET status` with state machine transitions:
   ```python
   import re

   with open("srcs/server/orchestration/tasks.go", "r") as f:
       content = f.read()

   old_update = """	query := `
		UPDATE shared_tasks
		SET title = $1, priority = $2, agent_id = $3, payload = $4, locked_until = $5, updated_at = CURRENT_TIMESTAMP
		WHERE id = $6
	`
	_, err = tx.Exec(ctx, query, task.Title, task.Priority, task.AssignedAgentID, task.Payload, task.LockedUntil, task.ID)
	if err != nil {
		return fmt.Errorf("failed to update task: %w", err)
	}

	broadcastFunc, err := tm.stateMachine.TransitionWithTx(ctx, tx, task.ID, "SHARED_TASK", task.Status, task.AssignedAgentID, "Task updated via UpdateTask")
	if err != nil {
		return fmt.Errorf("failed to transition state: %w", err)
	}"""
   new_update = """	query := `
		UPDATE shared_tasks
		SET title = $1, priority = $2, payload = $3, locked_until = $4
		WHERE id = $5
	`
	_, err = tx.Exec(ctx, query, task.Title, task.Priority, task.Payload, task.LockedUntil, task.ID)
	if err != nil {
		return fmt.Errorf("failed to update task: %w", err)
	}

	broadcastFunc, err := tm.stateMachine.TransitionWithTx(ctx, tx, task.ID, "SHARED_TASK", task.Status, task.AssignedAgentID, "Task updated via UpdateTask")
	if err != nil {
		return fmt.Errorf("failed to transition state: %w", err)
	}"""
   content = content.replace(old_update, new_update)
   with open("srcs/server/orchestration/tasks.go", "w") as f:
       f.write(content)

   with open("srcs/server/orchestration/tasks_db.go", "r") as f:
       content = f.read()
   content = re.sub(
       r'_, err = tx\.Exec\(ctx, "UPDATE shared_tasks SET status = \'IN_PROGRESS\', assigned_agent_id = \$1, updated_at = CURRENT_TIMESTAMP WHERE id = \$2 AND organization_id = \$3", agentID, id, orgID\)',
       r'// Status updated via transition\n\t_, err = tx.Exec(ctx, "UPDATE shared_tasks SET assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND organization_id = $3", agentID, id, orgID)',
       content
   )
   content = re.sub(
       r'if _, err := tx\.Exec\(ctx, "UPDATE shared_tasks SET status = \$1, updated_at = CURRENT_TIMESTAMP WHERE id = \$2", toState, taskID\); err != nil {',
       r'// Status handled by state machine\n\tif _, err := tx.Exec(ctx, "UPDATE shared_tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = $1", taskID); err != nil {',
       content
   )
   with open("srcs/server/orchestration/tasks_db.go", "w") as f:
       f.write(content)

   with open("srcs/server/orchestration/sub_agent.go", "r") as f:
       content = f.read()
   content = re.sub(
       r'`UPDATE shared_tasks SET status = \$1, updated_at = CURRENT_TIMESTAMP WHERE id = \$2`,',
       r'`UPDATE shared_tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = $2`, // Status updated via state machine transition',
       content
   )
   content = re.sub(
       r'_, err := s\.db\.Exec\(context\.Background\(\), "UPDATE shared_tasks SET status = \'FAILED\', updated_at = CURRENT_TIMESTAMP WHERE id = \$1", task\.ID\)',
       r'_, err := s.db.Exec(context.Background(), "UPDATE shared_tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = $1", task.ID)',
       content
   )
   content = re.sub(
       r'_, err := s\.db\.Exec\(context\.Background\(\), "UPDATE shared_tasks SET status = \'COMPLETED\', updated_at = CURRENT_TIMESTAMP WHERE id = \$1", task\.ID\)',
       r'_, err := s.db.Exec(context.Background(), "UPDATE shared_tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = $1", task.ID)',
       content
   )
   with open("srcs/server/orchestration/sub_agent.go", "w") as f:
       f.write(content)

   with open("srcs/server/orchestration/queue/kairos_queue.go", "r") as f:
       content = f.read()
   content = re.sub(
       r'`\n\t\tUPDATE shared_tasks\n\t\tSET status = \'IN_PROGRESS\', assigned_agent = \$1\n\t\tWHERE id = \(',
       r'`\n\t\tUPDATE shared_tasks\n\t\tSET assigned_agent = $1\n\t\tWHERE id = (',
       content
   )
   content = re.sub(
       r'`\n\t\tUPDATE shared_tasks\n\t\tSET status = \'IN_PROGRESS\', assigned_agent = \?\n\t\tWHERE id = \(',
       r'`\n\t\tUPDATE shared_tasks\n\t\tSET assigned_agent = ?\n\t\tWHERE id = (',
       content
   )
   with open("srcs/server/orchestration/queue/kairos_queue.go", "w") as f:
       f.write(content)

   with open("srcs/server/orchestration/autodream/kairos_autodream.go", "r") as f:
       content = f.read()
   content = re.sub(
       r'updateQuery := "UPDATE shared_tasks SET status = \'CONSOLIDATED\' WHERE id = \$1"',
       r'updateQuery := "UPDATE shared_tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = $1"',
       content
   )
   content = re.sub(
       r'updateQuery = "UPDATE shared_tasks SET status = \'CONSOLIDATED\' WHERE id = \?"',
       r'updateQuery = "UPDATE shared_tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = ?"',
       content
   )
   with open("srcs/server/orchestration/autodream/kairos_autodream.go", "w") as f:
       f.write(content)
   ```

3. **Verify refactoring**
   - Run `go build ./srcs/server/orchestration/...` via `run_in_bash_session` to confirm compilation.

4. **Test execution**
   - Run `./bazelisk test //srcs/server/orchestration/...` via `run_in_bash_session` to confirm functionality.

5. **Complete pre-commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit the change.**
   - Once all tests pass, submit the change with a descriptive commit message with the issue tracking id `3997`.
