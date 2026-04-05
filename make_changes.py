import os
import re

# Update tasks.go
with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

content = re.sub(
    r'func \(tm \*TaskManager\) ClaimTask\(ctx context\.Context, taskID, agentID string\) \(\*SharedTask, error\) \{',
    r'func (tm *TaskManager) ClaimTask(ctx context.Context, organizationID, taskID, agentID string) (*SharedTask, error) {',
    content
)
content = re.sub(
    r'WHERE id = \$1 AND status = \'PENDING\'',
    r'WHERE id = $1 AND organization_id = $2 AND status = \'PENDING\'',
    content
)
content = re.sub(
    r'errQuery = tx\.QueryRow\(ctx, query, taskID\)\.Scan',
    r'errQuery = tx.QueryRow(ctx, query, taskID, organizationID).Scan',
    content
)
content = re.sub(
    r'WHERE id = \$2 AND status = \'PENDING\'',
    r'WHERE id = $2 AND organization_id = $3 AND status = \'PENDING\'',
    content
)
content = re.sub(
    r'rowsAffected, err := tx\.Exec\(ctx, updateQuery, agentID, task\.ID\)',
    r'rowsAffected, err := tx.Exec(ctx, updateQuery, agentID, task.ID, organizationID)',
    content
)

# ReviewTask
content = re.sub(
    r'func \(tm \*TaskManager\) ReviewTask\(ctx context\.Context, taskID, agentID string\) error \{',
    r'func (tm *TaskManager) ReviewTask(ctx context.Context, organizationID, taskID, agentID string) error {',
    content
)
content = re.sub(
    r'WHERE id = \$1 AND agent_id = \$2 AND status = \'IN_PROGRESS\'',
    r'WHERE id = $1 AND agent_id = $2 AND organization_id = $3 AND status = \'IN_PROGRESS\'',
    content
)
content = re.sub(
    r'rowsAffected, err := tm\.db\.Exec\(ctx, query, taskID, agentID\)',
    r'rowsAffected, err := tm.db.Exec(ctx, query, taskID, agentID, organizationID)',
    content
)

# CompleteTask
content = re.sub(
    r'func \(tm \*TaskManager\) CompleteTask\(ctx context\.Context, taskID, agentID string\) error \{',
    r'func (tm *TaskManager) CompleteTask(ctx context.Context, organizationID, taskID, agentID string) error {',
    content
)
content = re.sub(
    r'err := tm\.db\.QueryRow\(ctx, "SELECT created_at FROM shared_tasks WHERE id = \$1", taskID\)\.Scan\(&createdAt\)',
    r'err := tm.db.QueryRow(ctx, "SELECT created_at FROM shared_tasks WHERE id = $1 AND organization_id = $2", taskID, organizationID).Scan(&createdAt)',
    content
)
content = re.sub(
    r'WHERE id = \$1 AND agent_id = \$2 AND status IN \(\'IN_PROGRESS\', \'REVIEW\'\)',
    r'WHERE id = $1 AND agent_id = $2 AND organization_id = $3 AND status IN (\'IN_PROGRESS\', \'REVIEW\')',
    content
)
content = re.sub(
    r'rowsAffected, err := tm\.db\.Exec\(ctx, query, taskID, agentID\)',
    r'rowsAffected, err := tm.db.Exec(ctx, query, taskID, agentID, organizationID)',
    content
)

# PeekTasks
content = re.sub(
    r'func \(tm \*TaskManager\) PeekTasks\(ctx context\.Context, limit int\) \(\[\]\*SharedTask, error\) \{',
    r'func (tm *TaskManager) PeekTasks(ctx context.Context, organizationID string, limit int) ([]*SharedTask, error) {',
    content
)
content = re.sub(
    r'WHERE status = \'PENDING\' AND \(locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP\)',
    r'WHERE organization_id = $1 AND status = \'PENDING\' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)',
    content
)
content = re.sub(
    r'query \+= fmt\.Sprintf\(" LIMIT \%d", limit\)',
    r'query += fmt.Sprintf(" LIMIT %d", limit)',
    content
)
content = re.sub(
    r'rows, err := tm\.db\.Query\(ctx, query\)',
    r'rows, err := tm.db.Query(ctx, query, organizationID)',
    content
)

# PollTasks
content = re.sub(
    r'func \(tm \*TaskManager\) PollTasks\(ctx context\.Context, agentID string, limit int\) \(\[\]\*SharedTask, error\) \{',
    r'func (tm *TaskManager) PollTasks(ctx context.Context, organizationID, agentID string, limit int) ([]*SharedTask, error) {',
    content
)
content = re.sub(
    r'WHERE status = \'PENDING\' AND \(locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP\)\n(.*?)LIMIT \$1',
    r'WHERE organization_id = $1 AND status = \'PENDING\' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)\n\1LIMIT $2',
    content
)
content = re.sub(
    r'rows, err := tx\.Query\(ctx, query, fetchLimit\)',
    r'rows, err := tx.Query(ctx, query, organizationID, fetchLimit)',
    content
)
content = re.sub(
    r'WHERE id = \$2 AND status = \'PENDING\'',
    r'WHERE id = $2 AND organization_id = $3 AND status = \'PENDING\'',
    content
)
content = re.sub(
    r'rowsAffected, err := tx\.Exec\(ctx, `\n\t\t\tUPDATE shared_tasks\n\t\t\tSET status = \'IN_PROGRESS\', agent_id = \$1, updated_at = CURRENT_TIMESTAMP\n\t\t\tWHERE id = \$2 AND status = \'PENDING\'\n\t\t`, agentID, task\.ID\)',
    r'rowsAffected, err := tx.Exec(ctx, `\n\t\t\tUPDATE shared_tasks\n\t\t\tSET status = \'IN_PROGRESS\', agent_id = $1, updated_at = CURRENT_TIMESTAMP\n\t\t\tWHERE id = $2 AND organization_id = $3 AND status = \'PENDING\'\n\t\t`, agentID, task.ID, organizationID)',
    content
)

content = content.replace(
    'tasks, err := tm.PeekTasks(ctx, 1)',
    'tasks, err := tm.PeekTasks(ctx, "default_org", 1)'
)
content = content.replace("status = \\'PENDING\\'", "status = 'PENDING'")
content = content.replace("status = \\'IN_PROGRESS\\'", "status = 'IN_PROGRESS'")
content = content.replace("status IN (\\'IN_PROGRESS\\', \\'REVIEW\\')", "status IN ('IN_PROGRESS', 'REVIEW')")

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(content)

# Service
with open("srcs/server/orchestration/service.go", "r") as f:
    service = f.read()

service = re.sub(
    r'tasks,\s*err\s*:=\s*tm\.PollTasks\(r\.Context\(\),\s*agentID,\s*limit\)',
    r'tasks, err := tm.PollTasks(r.Context(), claims.OrganizationID, agentID, limit)',
    service
)
service = re.sub(
    r'err\s*=\s*tm\.ReviewTask\(r\.Context\(\),\s*taskID,\s*req\.AgentID\)',
    r'err = tm.ReviewTask(r.Context(), claims.OrganizationID, taskID, req.AgentID)',
    service
)
service = re.sub(
    r'err\s*=\s*tm\.CompleteTask\(r\.Context\(\),\s*taskID,\s*req\.AgentID\)',
    r'err = tm.CompleteTask(r.Context(), claims.OrganizationID, taskID, req.AgentID)',
    service
)

imports = re.search(r'import \((.*?)\)', service, re.DOTALL)
if imports:
    imp = imports.group(1)
    if '"github.com/onehumancorp/mono/srcs/server/auth"' not in imp:
        new_imp = imp + '\t"github.com/onehumancorp/mono/srcs/server/auth"\n'
        service = service.replace(imp, new_imp)

if "claims := auth.ClaimsFromContext" not in service:
    lines = service.split("\n")
    for i, line in enumerate(lines):
        if "func handlePollTasks" in line:
            lines.insert(i+1, '\tclaims := auth.ClaimsFromContext(r.Context())\n\tif claims == nil {\n\t\thttp.Error(w, "unauthorized", http.StatusUnauthorized)\n\t\treturn\n\t}')
            break
    for i, line in enumerate(lines):
        if "func handleUpdateTaskStatus" in line:
            lines.insert(i+1, '\tclaims := auth.ClaimsFromContext(r.Context())\n\tif claims == nil {\n\t\thttp.Error(w, "unauthorized", http.StatusUnauthorized)\n\t\treturn\n\t}')
            break
    service = "\n".join(lines)

with open("srcs/server/orchestration/service.go", "w") as f:
    f.write(service)

# tests
with open("srcs/server/orchestration/tasks_test.go", "r") as f:
    test = f.read()

setup_db_patch = """
	// Create tables
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			mission_id TEXT,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			locked_until DATETIME,
			payload TEXT NOT NULL DEFAULT '{}',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)
"""

if "task_dependencies" not in test:
    test = re.sub(
        r'// Create tables\s*_, err := prov\.Exec\(context\.Background\(\), `[\s\S]*?`\)',
        setup_db_patch.strip(),
        test
    )

    test = re.sub(r'tm\.ClaimTask\(ctx, "non-existent-task-id", "agent-1"\)', r'tm.ClaimTask(ctx, "default_org", "non-existent-task-id", "agent-1")', test)
    test = re.sub(r'tm\.ClaimTask\(ctx, createdTask\.ID, "agent-1"\)', r'tm.ClaimTask(ctx, "default_org", createdTask.ID, "agent-1")', test)
    test = re.sub(r'tm\.ClaimTask\(ctx, "another-non-existent-id", "agent-2"\)', r'tm.ClaimTask(ctx, "default_org", "another-non-existent-id", "agent-2")', test)
    test = re.sub(r'tm\.ClaimTask\(ctx, task\.ID, "agent-1"\)', r'tm.ClaimTask(ctx, "default_org", task.ID, "agent-1")', test)
    test = re.sub(r'tm\.CreateTask\(ctx, "mission-[0-9]+",', r'tm.CreateTask(ctx, "default_org",', test)
    test = re.sub(r'tm\.PollTasks\(ctx, "agent-1",', r'tm.PollTasks(ctx, "default_org", "agent-1",', test)
    test = re.sub(r'tm\.PollTasks\(ctx, "agent-2",', r'tm.PollTasks(ctx, "default_org", "agent-2",', test)
    test = re.sub(r'tm\.CompleteTask\(ctx, claimedTask\.ID, "agent-1"\)', r'tm.CompleteTask(ctx, "default_org", claimedTask.ID, "agent-1")', test)
    test = re.sub(r'tm\.CompleteTask\(ctx, "non-existent", "agent-1"\)', r'tm.CompleteTask(ctx, "default_org", "non-existent", "agent-1")', test)

    test += """
func TestTaskManager_DAGDependencies(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tm, cleanup := setupTestDB(t)
	defer cleanup()

	ctx := context.Background()

	// Create parent task
	parentTask, _ := tm.CreateTask(ctx, "default_org", "Parent Task", "Desc", "P1")

	// Create dependent task
	depTask, _ := tm.CreateTaskWithPlan(ctx, "default_org", []string{parentTask.ID}, "Dependent Task", "Desc", "P1")

	// Try to poll
	tasks, err := tm.PollTasks(ctx, "default_org", "agent-1", 5)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Should only poll parentTask since depTask is blocked
	if len(tasks) != 1 {
		t.Fatalf("expected 1 task polled, got %d", len(tasks))
	}
	if tasks[0].ID != parentTask.ID {
		t.Fatalf("expected to poll parent task, got %v", tasks[0].Title)
	}

	// Complete parent task
	err = tm.CompleteTask(ctx, "default_org", parentTask.ID, "agent-1")
	if err != nil {
		t.Fatalf("expected no error completing parent, got %v", err)
	}

	// Now poll again, depTask should be available
	tasks2, err := tm.PollTasks(ctx, "default_org", "agent-2", 5)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(tasks2) != 1 {
		t.Fatalf("expected 1 task polled, got %d", len(tasks2))
	}
	if tasks2[0].ID != depTask.ID {
		t.Fatalf("expected to poll dependent task, got %v", tasks2[0].Title)
	}
}
"""

with open("srcs/server/orchestration/tasks_test.go", "w") as f:
    f.write(test)

# DB changes
with open("srcs/server/db/migrations/021_kairos_orchestration.sql", "r") as f:
    migration = f.read()
if "mission_id" not in migration:
    migration = migration.replace(
        "organization_id VARCHAR NOT NULL,",
        "organization_id VARCHAR NOT NULL,\n    mission_id TEXT,\n    parent_plan_id TEXT,"
    )
    with open("srcs/server/db/migrations/021_kairos_orchestration.sql", "w") as f:
        f.write(migration)

with open("srcs/server/orchestration/BUILD.bazel", "r") as f:
    build = f.read()
if '"//srcs/server/auth",' not in build:
    build = build.replace(
        '"//srcs/server/db",',
        '"//srcs/server/auth",\n        "//srcs/server/db",'
    )
    with open("srcs/server/orchestration/BUILD.bazel", "w") as f:
        f.write(build)

# UI changes
with open("srcs/app/lib/screens/swarm_memory_screen.dart", "r") as f:
    ui = f.read()
if "ImageFilter.compose" not in ui:
    ui = ui.replace(
        """      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(""",
        """      child: BackdropFilter(
        filter: ImageFilter.compose(outer: ColorFilter.matrix([1.168, -0.153, -0.015, 0, 0, -0.046, 1.061, -0.015, 0, 0, -0.046, -0.152, 1.198, 0, 0, 0, 0, 0, 1, 0]), inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)),
        child: Container("""
    )
    with open("srcs/app/lib/screens/swarm_memory_screen.dart", "w") as f:
        f.write(ui)
