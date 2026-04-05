package dbinspector

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"regexp"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/agents/local"
)

// ListTools returns the definitions for the dbinspector tools.
func ListTools() []local.ToolDefinition {
	return []local.ToolDefinition{
		{
			Name:        "inspect_schema",
			Description: "Introspects database schemas for the current environment.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{},
			},
		},
		{
			Name:        "run_query",
			Description: "Executes diagnostic queries. Strictly READ-ONLY unless override_safety_lock is provided by an admin.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"query": map[string]interface{}{
						"type":        "string",
						"description": "The SQL query to run.",
					},
					"override_safety_lock": map[string]interface{}{
						"type":        "boolean",
						"description": "Optional flag to bypass READ-ONLY protections. Requires admin claim.",
					},
				},
				"required": []interface{}{"query"},
			},
		},
		{
			Name:        "get_stats",
			Description: "Retrieves diagnostic database statistics.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{},
			},
		},
	}
}

// Ensure interface compatibility
var _ local.Tool = (*InspectSchemaTool)(nil)
var _ local.Tool = (*RunQueryTool)(nil)
var _ local.Tool = (*GetStatsTool)(nil)

type InspectSchemaTool struct {
	DB db.Provider
}

func (t *InspectSchemaTool) Definition() local.ToolDefinition {
	return ListTools()[0]
}

func (t *InspectSchemaTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	if t.DB == nil {
		return "", errors.New("database provider is not initialized")
	}

	var query string
	if t.DB.IsSQLite() {
		query = "SELECT type, name, sql FROM sqlite_master WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%';"
	} else {
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil {
			return "", errors.New("unauthorized: missing claims for cloud mode schema inspection")
		}
		// In a real multi-tenant we might set local search path, but here we just show the public schema or the specific tenant's schema if applicable.
		// `runDBQuery` already handles execution context.
		query = "SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_schema = 'public';"
	}

	return runDBQuery(ctx, t.DB, query)
}

type RunQueryTool struct {
	DB db.Provider
}

func (t *RunQueryTool) Definition() local.ToolDefinition {
	return ListTools()[1]
}

var mutatingPattern = regexp.MustCompile(`(?i)\b(INSERT|UPDATE|DELETE|DROP|ALTER|CREATE|TRUNCATE|GRANT|REVOKE)\b`)

func (t *RunQueryTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	if t.DB == nil {
		return "", errors.New("database provider is not initialized")
	}

	queryRaw, ok := input["query"]
	if !ok {
		return "", errors.New("missing required argument 'query'")
	}
	query, ok := queryRaw.(string)
	if !ok {
		return "", errors.New("argument 'query' must be a string")
	}

	overrideRaw, _ := input["override_safety_lock"]
	overrideSafetyLock, _ := overrideRaw.(bool)

	if mutatingPattern.MatchString(query) {
		if !overrideSafetyLock {
			return "", errors.New("mutating queries are not allowed in READ-ONLY mode")
		}
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil {
			return "", errors.New("unauthorized: missing claims")
		}
		isAdmin := false
		for _, role := range claims.Roles {
			if role == "admin" {
				isAdmin = true
				break
			}
		}
		if !isAdmin {
			return "", errors.New("unauthorized: only admins can use override_safety_lock for mutating queries")
		}
	}

	return runDBQuery(ctx, t.DB, query)
}

type GetStatsTool struct {
	DB db.Provider
}

func (t *GetStatsTool) Definition() local.ToolDefinition {
	return ListTools()[2]
}

func (t *GetStatsTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	if t.DB == nil {
		return "", errors.New("database provider is not initialized")
	}

	var query string
	if t.DB.IsSQLite() {
		query = "PRAGMA compile_options;"
	} else {
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil {
			return "", errors.New("unauthorized: missing claims for cloud mode stats inspection")
		}
		query = "SELECT datname, numbackends, xact_commit, xact_rollback FROM pg_stat_database WHERE datname = current_database();"
	}

	return runDBQuery(ctx, t.DB, query)
}

func runDBQuery(ctx context.Context, dbProvider db.Provider, query string) (string, error) {
	if mutatingPattern.MatchString(query) {
		res, err := dbProvider.Exec(ctx, query)
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("Rows affected: %d", res), nil
	}

	rows, err := dbProvider.Query(ctx, query)
	if err != nil {
		return "", err
	}
	defer rows.Close()

	cols, err := rows.Columns()
	if err != nil {
		return "", fmt.Errorf("failed to get columns: %w", err)
	}

	var results []map[string]interface{}

	for rows.Next() {
		columns := make([]interface{}, len(cols))
		columnPointers := make([]interface{}, len(cols))
		for i := range columns {
			columnPointers[i] = &columns[i]
		}

		if err := rows.Scan(columnPointers...); err != nil {
			return "", err
		}

		m := make(map[string]interface{})
		for i, colName := range cols {
			val := columnPointers[i].(*interface{})
			// Convert []byte to string for easier JSON serialization
			if b, ok := (*val).([]byte); ok {
				m[colName] = string(b)
			} else {
				m[colName] = *val
			}
		}
		results = append(results, m)
	}

	if err := rows.Err(); err != nil {
		return "", err
	}

	b, err := json.Marshal(results)
	if err != nil {
		return "", err
	}

	return string(b), nil
}
