package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/db/provider.go"
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	content := string(contentBytes)

	funcToAdd := "	CreateTask(ctx context.Context, task *TaskRecord) error\n}"

	if !strings.Contains(content, "CreateTask") {
		content = strings.Replace(content, "	ClaimTask(ctx context.Context, taskID string) error\n}", "	ClaimTask(ctx context.Context, taskID string) error\n"+funcToAdd, 1)
		err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully patched provider.go")
	} else {
		fmt.Println("CreateTask already exists in provider.go")
	}

	pgFilePath := "srcs/server/db/postgres_provider.go"
	pgBytes, err := ioutil.ReadFile(pgFilePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	pgContent := string(pgBytes)

	pgFuncToAdd := `
func (p *PgProvider) CreateTask(ctx context.Context, task *TaskRecord) error {
	query := ` + "`" + `
		INSERT INTO tasks (id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	` + "`" + `
	_, err := p.pool.Exec(ctx, query, task.ID, task.OrganizationID, task.ParentTaskID, task.Title, task.Description, task.Status, task.AgentID, task.Dependencies)
	return err
}
`

	if !strings.Contains(pgContent, "CreateTask") {
		pgContent += pgFuncToAdd
		err = ioutil.WriteFile(pgFilePath, []byte(pgContent), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully patched postgres_provider.go")
	} else {
		fmt.Println("CreateTask already exists in postgres_provider.go")
	}

	sqliteFilePath := "srcs/server/db/sqlite_provider.go"
	sqliteBytes, err := ioutil.ReadFile(sqliteFilePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	sqliteContent := string(sqliteBytes)

	sqliteFuncToAdd := `
func (p *SqliteProvider) CreateTask(ctx context.Context, task *TaskRecord) error {
	query := ` + "`" + `
		INSERT INTO tasks (id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	` + "`" + `
	_, err := p.db.ExecContext(ctx, query, task.ID, task.OrganizationID, task.ParentTaskID, task.Title, task.Description, task.Status, task.AgentID, task.Dependencies)
	return err
}
`

	if !strings.Contains(sqliteContent, "CreateTask") {
		sqliteContent += sqliteFuncToAdd
		err = ioutil.WriteFile(sqliteFilePath, []byte(sqliteContent), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully patched sqlite_provider.go")
	} else {
		fmt.Println("CreateTask already exists in sqlite_provider.go")
	}
}
