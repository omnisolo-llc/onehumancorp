import re

with open("srcs/server/agents/kairos/task_decomposer.go", "r") as f:
    content = f.read()

# Make checkCircularDependencies verify against the database
new_check = """// checkCircularDependencies performs a topological sort to detect cycles.
// It checks the new tasks and their existing dependencies in the database.
func (td *TaskDecomposer) checkCircularDependencies(ctx context.Context, tasks []*Task) error {
	inDegree := make(map[string]int)
	graph := make(map[string][]string)
	nodes := make(map[string]struct{})

	// Gather all dependencies to query their dependencies from DB
	var depIDs []string

	for _, t := range tasks {
		nodes[t.ID] = struct{}{}
		inDegree[t.ID] = 0
		for _, dep := range t.Dependencies {
			depIDs = append(depIDs, dep)
		}
	}

	// For simplicity, we just fetch all tasks for the org.
	// A more robust solution would recurse or use a CTE for only related tasks.
	if len(tasks) > 0 {
		orgID := tasks[0].OrganizationID
		query := `SELECT id, dependencies FROM shared_tasks_decomposition WHERE organization_id = $1`
		rows, err := td.provider.Query(ctx, query, orgID)
		if err == nil {
			defer rows.Close()
			for rows.Next() {
				var dbID string
				var dbDepsJSON string
				if err := rows.Scan(&dbID, &dbDepsJSON); err == nil {
					var dbDeps []string
					if err := json.Unmarshal([]byte(dbDepsJSON), &dbDeps); err == nil {
						if _, exists := nodes[dbID]; !exists {
							nodes[dbID] = struct{}{}
							inDegree[dbID] = 0
						}
						for _, dep := range dbDeps {
							graph[dep] = append(graph[dep], dbID)
							inDegree[dbID]++
							if _, exists := nodes[dep]; !exists {
								nodes[dep] = struct{}{}
								inDegree[dep] = 0
							}
						}
					}
				}
			}
		}
	}

	// Add new tasks to graph
	for _, t := range tasks {
		for _, dep := range t.Dependencies {
			// t depends on dep: dep -> t
			graph[dep] = append(graph[dep], t.ID)
			inDegree[t.ID]++
			if _, exists := nodes[dep]; !exists {
				nodes[dep] = struct{}{}
				inDegree[dep] = 0
			}
		}
	}

	var queue []string
	for node, degree := range inDegree {
		if degree == 0 {
			queue = append(queue, node)
		}
	}

	visitedCount := 0
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		visitedCount++

		for _, v := range graph[u] {
			inDegree[v]--
			if inDegree[v] == 0 {
				queue = append(queue, v)
			}
		}
	}

	if visitedCount != len(nodes) {
		return ErrCircularDependency
	}

	return nil
}"""

match = re.search(r'// checkCircularDependencies', content)
start_idx = match.start()

new_content = content[:start_idx] + new_check + "\n"

# also update CreateTasks to pass ctx
new_content = new_content.replace("td.checkCircularDependencies(tasks)", "td.checkCircularDependencies(ctx, tasks)")

with open("srcs/server/agents/kairos/task_decomposer.go", "w") as f:
    f.write(new_content)
