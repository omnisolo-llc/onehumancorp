package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/orchestration/tasks_db.go")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent := string(content)

	if !strings.Contains(strContent, "\"encoding/json\"") {
		strContent = strings.Replace(strContent, "import (", "import (\n\t\"encoding/json\"\n", 1)
	}

	newResolve := `// ResolveTaskDependencies checks the status of all dependencies of a task.
// Returns true if all dependencies are COMPLETED.
func (to *SharedTaskOrchestrator) ResolveTaskDependencies(ctx context.Context, taskID string) (bool, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return false, errors.New("unauthorized: missing claims")
	}

	query := ` + "`" + `SELECT dependencies FROM shared_tasks_v2 WHERE id = $1` + "`" + `
	var depsJSON *string
	err := to.dbProvider.QueryRow(ctx, query, taskID).Scan(&depsJSON)
	if err != nil {
		return false, fmt.Errorf("failed to get task dependencies: %w", err)
	}

	if depsJSON == nil {
		return true, nil
	}

	var deps []string
	if err := json.Unmarshal([]byte(*depsJSON), &deps); err != nil {
		return false, fmt.Errorf("failed to parse dependencies: %w", err)
	}

	if len(deps) == 0 {
		return true, nil
	}

	for _, depID := range deps {
		var status string
		err := to.dbProvider.QueryRow(ctx, "SELECT status FROM shared_tasks_v2 WHERE id = $1", depID).Scan(&status)
		if err != nil || status != "COMPLETED" {
			return false, nil
		}
	}

	return true, nil
}`

	// Regex-like replacement is tricky in Go string replacement without regexp, so let's just rewrite the end of the file.
	// Actually we know exactly what is there from the previous patch.
	oldResolvePart := `// ResolveTaskDependencies checks the status of all dependencies of a task.
// Returns true if all dependencies are COMPLETED.
func (to *SharedTaskOrchestrator) ResolveTaskDependencies(ctx context.Context, taskID string) (bool, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return false, errors.New("unauthorized: missing claims")
	}

	query := ` + "`" + `SELECT dependencies FROM shared_tasks_v2 WHERE id = $1` + "`" + `
	var depsJSON *string
	err := to.dbProvider.QueryRow(ctx, query, taskID).Scan(&depsJSON)
	if err != nil {
		return false, fmt.Errorf("failed to get task dependencies: %w", err)
	}

	if depsJSON == nil {
		return true, nil
	}

	var deps []string
	importJson := true // dummy to force json import logic if needed, we'll just add json to imports
	if importJson {
		importJson = false
	}

	// We'll rely on the orchestrator logic
	return true, nil
}`

	strContent = strings.Replace(strContent, oldResolvePart, newResolve, 1)

	err = ioutil.WriteFile("srcs/server/orchestration/tasks_db.go", []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}
	fmt.Println("tasks_db.go final resolve patched successfully")
}
