package dbinspector

import (
	"context"
	"errors"
	"fmt"
	"regexp"
	"strings"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// DBInspectorMCP implements the MCP interface for database introspection.
type DBInspectorMCP struct {
	provider       db.Provider
	throttleMu     sync.Mutex
	sqliteThrottle time.Duration
}

// NewDBInspectorMCP creates a new DBInspectorMCP instance.
func NewDBInspectorMCP(provider db.Provider) *DBInspectorMCP {
	return &DBInspectorMCP{
		provider:       provider,
		sqliteThrottle: 10 * time.Millisecond,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *DBInspectorMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "inspect_schema",
			Description: "Inspects the database schema (tables, columns, types).",
			InputSchema: `{"type": "object", "properties": {"table_name": {"type": "string"}}}`,
		},
		{
			Name:        "run_query",
			Description: "Executes a READ-ONLY diagnostic query.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}, "override_safety_lock": {"type": "boolean"}}, "required": ["query"]}`,
		},
		{
			Name:        "get_stats",
			Description: "Retrieves active connections and basic stats.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *DBInspectorMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsSQLite() {
		return nil, errors.New("unauthorized: missing claims")
	}

	if m.provider.IsSQLite() {
		m.throttleMu.Lock()
		defer m.throttleMu.Unlock()
		time.Sleep(m.sqliteThrottle)
	}

	switch toolName {
	case "inspect_schema":
		tableName := ""
		if tn, ok := arguments["table_name"].(string); ok {
			tableName = tn
		}
		return m.inspectSchema(ctx, tableName)
	case "run_query":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}
		override := false
		if ov, ok := arguments["override_safety_lock"].(bool); ok {
			override = ov
		}
		return m.runQuery(ctx, claims, query, override)
	case "get_stats":
		return m.getStats(ctx)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *DBInspectorMCP) inspectSchema(ctx context.Context, tableName string) (interface{}, error) {
	if m.provider.IsSQLite() {
		query := "SELECT name, sql FROM sqlite_master WHERE type='table'"
		var args []interface{}
		if tableName != "" {
			query += " AND name = ?"
			args = append(args, tableName)
		}

		rows, err := m.provider.Query(ctx, query, args...)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		var results []map[string]interface{}
		for rows.Next() {
			var name, sql string
			if err := rows.Scan(&name, &sql); err == nil {
				results = append(results, map[string]interface{}{"table": name, "schema": sql})
			}
		}
		return results, nil
	} else {
		query := "SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_schema = current_schema()"
		var args []interface{}
		if tableName != "" {
			query += " AND table_name = $1"
			args = append(args, tableName)
		}
		rows, err := m.provider.Query(ctx, query, args...)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		var results []map[string]interface{}
		for rows.Next() {
			var tname, cname, dtype string
			if err := rows.Scan(&tname, &cname, &dtype); err == nil {
				results = append(results, map[string]interface{}{"table": tname, "column": cname, "type": dtype})
			}
		}
		return results, nil
	}
}

var (
	unsafeRegex = regexp.MustCompile(`(?i)\b(INSERT|UPDATE|DELETE|DROP|ALTER|CREATE|TRUNCATE|GRANT|REVOKE|REPLACE|UPSERT)\b`)
	orgIDRegex  = regexp.MustCompile(`^[a-zA-Z0-9_\-]+$`)
)

func (m *DBInspectorMCP) runQuery(ctx context.Context, claims *auth.Claims, query string, override bool) (interface{}, error) {
	unsafe := unsafeRegex.MatchString(query)

	if unsafe {
		if !override {
			return nil, errors.New("unsafe query detected: explicitly reject INSERT, UPDATE, DELETE, DROP, ALTER unless override_safety_lock is true")
		}

		isAdmin := false
		if claims != nil {
			for _, r := range claims.Roles {
				if r == "admin" || r == "system" {
					isAdmin = true
					break
				}
			}
		}
		if !isAdmin {
			return nil, errors.New("unauthorized: override_safety_lock requires admin claim")
		}
	}

	if !m.provider.IsSQLite() && claims != nil {
		// Postgres mode: Set local search path
		tx, err := m.provider.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		// Sanitize organization ID for search_path using simple quotes if safe
		if !orgIDRegex.MatchString(claims.OrganizationID) {
			return nil, errors.New("invalid organization ID format")
		}

		sanitizedOrgID := strings.ReplaceAll(claims.OrganizationID, "\"", "\"\"")
		// Postgres mode: Set local search path safely avoiding SQL injection
		_, err = tx.Exec(ctx, fmt.Sprintf("SET LOCAL search_path = \"%s\"", sanitizedOrgID))
		if err != nil {
			return nil, err
		}

		// Note: since we're using tx, we would ideally execute the query on tx instead of using provider.Query
		// For simplicity, we just execute on tx here.
		// Note: a true diagnostic query returning many columns could be complex, we just execute and fetch dummy rows or simply return success if no error.
		// A proper row parser would get columns and scan into dynamic slice, but that's very verbose. Let's do a basic execution.

		rows, err := tx.Query(ctx, query)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		results, err := scanRows(rows)
		if err != nil {
			return nil, err
		}
		tx.Commit(ctx)
		return map[string]interface{}{"status": "success", "results": results, "mode": "cloud"}, nil
	}

	// SQLite Mode
	rows, err := m.provider.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	results, err := scanRows(rows)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{"status": "success", "results": results, "mode": "standalone"}, nil
}

func scanRows(rows db.Rows) ([]map[string]interface{}, error) {
	columns, err := rows.Columns()
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for rows.Next() {
		columnsData := make([]interface{}, len(columns))
		columnPointers := make([]interface{}, len(columns))
		for i := range columnsData {
			columnPointers[i] = &columnsData[i]
		}

		if err := rows.Scan(columnPointers...); err != nil {
			return nil, err
		}

		rowData := make(map[string]interface{})
		for i, colName := range columns {
			val := columnsData[i]
			if b, ok := val.([]byte); ok {
				rowData[colName] = string(b)
			} else {
				rowData[colName] = val
			}
		}
		results = append(results, rowData)
	}
	return results, rows.Err()
}

func (m *DBInspectorMCP) getStats(ctx context.Context) (interface{}, error) {
	if m.provider.IsSQLite() {
		return map[string]interface{}{"status": "ok", "mode": "standalone", "connections": 1}, nil
	}

	// Postgres
	rows, err := m.provider.Query(ctx, "SELECT count(*) FROM pg_stat_activity")
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var count int
	if rows.Next() {
		_ = rows.Scan(&count)
	}
	return map[string]interface{}{"status": "ok", "mode": "cloud", "connections": count}, nil
}
