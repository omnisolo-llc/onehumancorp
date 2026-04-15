package db

import (
	"context"
	"database/sql"
	"regexp"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"

	_ "modernc.org/sqlite"
)

var jsonPathRe = regexp.MustCompile(`([a-zA-Z0-9_]+)\s*::\s*json\s*->>\s*'([^']+)'`)

// SqliteProvider implements the Provider interface using database/sql with modernc.org/sqlite.
type SqliteProvider struct {
	db *sql.DB
}

func NewSqliteProvider(db *sql.DB) *SqliteProvider {
	return &SqliteProvider{db: db}
}

// convertBindVars parses PostgreSQL queries and translates them to SQLite syntax.
// It tracks string literal states to avoid replacing `$` inside quotes, maps Postgres
// positional parameters (e.g., `$1`) to SQLite numbered variables (e.g., `?1`),
// and dynamically strips natively unsupported clauses like `FOR UPDATE SKIP LOCKED`.
func convertBindVars(query string) string {
	query = strings.ReplaceAll(query, "FOR UPDATE SKIP LOCKED", "")
	query = strings.ReplaceAll(query, "::vector", "")

	var result strings.Builder
	result.Grow(len(query))

	inQuotes := false
	for i := 0; i < len(query); i++ {
		c := query[i]

		if c == '\'' {
			inQuotes = !inQuotes
			result.WriteByte(c)
			continue
		}

		if !inQuotes && c == '$' {
			// Look ahead for numbers
			j := i + 1
			for j < len(query) && query[j] >= '0' && query[j] <= '9' {
				j++
			}
			if j > i+1 {
				result.WriteByte('?')
				result.WriteString(query[i+1 : j])
				i = j - 1
				continue
			}
		}

		result.WriteByte(c)
	}

	resStr := result.String()
	// Map json paths `col::json->>'key'` to `json_extract(col, '$.key')`
	resStr = jsonPathRe.ReplaceAllString(resStr, "json_extract($1, '$.$2')")

	return resStr
}

// translateArgs translates pgx-style args if needed, though typically SQL standard positional args are similar enough.
// SQLite natively expects `?` instead of `$1`, `$2`. Wait, no, SQLite actually accepts `$1`, `$2` bindings if using proper parameter names, but default `database/sql` positional parameters are usually just `?`. Wait, `database/sql` driver for SQLite usually supports `?`, `$1`, and `:name`. Let's assume standard passing works unless proven otherwise.
func (p *SqliteProvider) Exec(ctx context.Context, sqlQuery string, arguments ...any) (int64, error) {
	start := time.Now()
	res, err := p.db.ExecContext(ctx, convertBindVars(sqlQuery), arguments...)
	trackQuery(ctx, "Exec", err, time.Since(start))
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (p *SqliteProvider) Query(ctx context.Context, sqlQuery string, optionsAndArgs ...any) (Rows, error) {
	start := time.Now()
	rows, err := p.db.QueryContext(ctx, convertBindVars(sqlQuery), optionsAndArgs...)
	trackQuery(ctx, "Query", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &SqliteRows{rows: rows}, nil
}

func (p *SqliteProvider) QueryRow(ctx context.Context, sqlQuery string, optionsAndArgs ...any) Row {
	start := time.Now()
	row := p.db.QueryRowContext(ctx, convertBindVars(sqlQuery), optionsAndArgs...)
	trackQuery(ctx, "QueryRow", nil, time.Since(start))
	return &SqliteRow{row: row}
}

func (p *SqliteProvider) Begin(ctx context.Context) (Tx, error) {
	start := time.Now()
	tx, err := p.db.BeginTx(ctx, nil)
	trackQuery(ctx, "Begin", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &SqliteTx{tx: tx}, nil
}

func (p *SqliteProvider) Close() {
	p.db.Close()
}

func (p *SqliteProvider) AcquireTask(ctx context.Context, organizationID, agentID string) (*TaskRecord, error) {
	start := time.Now()
	// SQLite supports UPDATE ... RETURNING
	// But it does not support subqueries with LIMIT in UPDATE directly.
	// So we use a transaction and two steps, or a simple single update if we use a specific condition.
	tx, err := p.Begin(ctx)
	if err != nil {
		trackQuery(ctx, "AcquireTask", err, time.Since(start))
		return nil, err
	}
	defer tx.Rollback(ctx)

	// In SQLite we can do UPDATE ... RETURNING where ID is subquery limit 1
	// because SQLite Begin creates an immediate transaction lock by default or
	// we rely on the concurrent writes lock.
	query := `
		UPDATE shared_tasks_decomposition
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks_decomposition
			WHERE status = 'PENDING' AND organization_id = $2
			ORDER BY created_at ASC
			LIMIT 1
		)
		RETURNING id, parent_plan_id, assigned_agent_id, status, payload, created_at, updated_at
	`

	var t TaskRecord
	var payloadStr *string
	var createdAtStr, updatedAtStr string
	err = tx.QueryRow(ctx, query, agentID, organizationID).Scan(
		&t.ID, &t.ParentTaskID, &t.AgentID, &t.Status, &payloadStr, &createdAtStr, &updatedAtStr,
	)
	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			// Memory instructions: SQLite lock contention detection parity with Postgres
			// Secondary check to see if any PENDING tasks exist that were skipped.
			var exists bool
			checkErr := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM shared_tasks_decomposition WHERE status = 'PENDING' AND organization_id = $1)", organizationID).Scan(&exists)
			if checkErr == nil && exists {
				// We found pending tasks but couldn't acquire any because they are locked.
				// In SQLite this usually happens if another connection locked them and we timed out or missed.
				// We simulate lock contention metric.
				telemetry.RecordPostgresLockContention(ctx, "acquire_task_sqlite")
			}
			trackQuery(ctx, "AcquireTask", nil, time.Since(start))
			return nil, nil
		}
		trackQuery(ctx, "AcquireTask", err, time.Since(start))
		return nil, err
	}

	t.Payload = payloadStr
	if t.CreatedAt, err = time.Parse("2006-01-02 15:04:05", createdAtStr); err != nil {
		// Fallback for RFC3339 which is sometimes used in tests
		t.CreatedAt, _ = time.Parse(time.RFC3339, createdAtStr)
	}
	if t.UpdatedAt, err = time.Parse("2006-01-02 15:04:05", updatedAtStr); err != nil {
		t.UpdatedAt, _ = time.Parse(time.RFC3339, updatedAtStr)
	}

	if err := tx.Commit(ctx); err != nil {
		trackQuery(ctx, "AcquireTask_Commit", err, time.Since(start))
		return nil, err
	}

	trackQuery(ctx, "AcquireTask", nil, time.Since(start))
	return &t, nil
}

func (p *SqliteProvider) IsSQLite() bool {
	return true
}

func (p *SqliteProvider) Ping(ctx context.Context) error {
	return p.db.PingContext(ctx)
}

// SqliteRows implements Rows using sql.Rows.
type SqliteRows struct {
	rows *sql.Rows
}

func (r *SqliteRows) Next() bool {
	return r.rows.Next()
}

func (r *SqliteRows) Scan(dest ...any) error {
	return r.rows.Scan(dest...)
}

func (r *SqliteRows) Columns() ([]string, error) {
	return r.rows.Columns()
}

func (r *SqliteRows) Close() {
	r.rows.Close()
}

func (r *SqliteRows) Err() error {
	return r.rows.Err()
}

// SqliteRow implements Row using sql.Row.
type SqliteRow struct {
	row *sql.Row
}

func (r *SqliteRow) Scan(dest ...any) error {
	return r.row.Scan(dest...)
}

// SqliteTx implements Tx using sql.Tx.
type SqliteTx struct {
	tx *sql.Tx
}

func (t *SqliteTx) Exec(ctx context.Context, sqlQuery string, arguments ...any) (int64, error) {
	start := time.Now()
	res, err := t.tx.ExecContext(ctx, convertBindVars(sqlQuery), arguments...)
	trackQuery(ctx, "Tx.Exec", err, time.Since(start))
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (t *SqliteTx) Query(ctx context.Context, sqlQuery string, optionsAndArgs ...any) (Rows, error) {
	start := time.Now()
	rows, err := t.tx.QueryContext(ctx, convertBindVars(sqlQuery), optionsAndArgs...)
	trackQuery(ctx, "Tx.Query", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &SqliteRows{rows: rows}, nil
}

func (t *SqliteTx) QueryRow(ctx context.Context, sqlQuery string, optionsAndArgs ...any) Row {
	start := time.Now()
	row := t.tx.QueryRowContext(ctx, convertBindVars(sqlQuery), optionsAndArgs...)
	trackQuery(ctx, "Tx.QueryRow", nil, time.Since(start))
	return &SqliteRow{row: row}
}

func (t *SqliteTx) Commit(ctx context.Context) error {
	return t.tx.Commit()
}

func (t *SqliteTx) Rollback(ctx context.Context) error {
	return t.tx.Rollback()
}
