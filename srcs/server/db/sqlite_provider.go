package db

import (
	"context"
	"database/sql"
	"time"

	_ "modernc.org/sqlite"
)

import (
	"regexp"
	"strings"
)

// SqliteProvider implements the Provider interface using database/sql with modernc.org/sqlite.
type SqliteProvider struct {
	db *sql.DB
}

func NewSqliteProvider(db *sql.DB) *SqliteProvider {
	return &SqliteProvider{db: db}
}

var jsonPathRegex = regexp.MustCompile(`([a-zA-Z0-9_]+)::json->>'([a-zA-Z0-9_]+)'`)

// convertBindVars translates PostgreSQL syntax to SQLite syntax.
// It maps positional parameters (e.g., $1) to SQLite numbered variables (?1),
// dynamically strips unsupported clauses like FOR UPDATE SKIP LOCKED,
// and maps JSON extraction syntax.
func convertBindVars(sql string) string {
	var out strings.Builder
	var inQuote bool
	var inDoubleQuote bool
	var inEscape bool

	out.Grow(len(sql))

	// Some basic syntax translations needed for Postgres to SQLite:
	sql = strings.ReplaceAll(sql, "FOR UPDATE SKIP LOCKED", "")

	for i := 0; i < len(sql); i++ {
		c := sql[i]
		if inEscape {
			out.WriteByte(c)
			inEscape = false
			continue
		}
		if c == '\\' {
			inEscape = true
			out.WriteByte(c)
			continue
		}
		if c == '\'' && !inDoubleQuote {
			inQuote = !inQuote
			out.WriteByte(c)
			continue
		}
		if c == '"' && !inQuote {
			inDoubleQuote = !inDoubleQuote
			out.WriteByte(c)
			continue
		}
		if !inQuote && !inDoubleQuote {
			if c == '$' && i+1 < len(sql) && sql[i+1] >= '0' && sql[i+1] <= '9' {
				// Parse the number
				start := i + 1
				end := start
				for end < len(sql) && sql[end] >= '0' && sql[end] <= '9' {
					end++
				}
				paramStr := sql[start:end]
				out.WriteString("?" + paramStr)
				i = end - 1
				continue
			}
		}
		out.WriteByte(c)
	}

	res := out.String()

	// Dynamic JSON path mapping
	res = jsonPathRegex.ReplaceAllString(res, `json_extract($1, '$$.$2')`)

	return res
}

func (p *SqliteProvider) Exec(ctx context.Context, sqlQuery string, arguments ...any) (int64, error) {
	start := time.Now()
	sqlQuery = convertBindVars(sqlQuery)
	res, err := p.db.ExecContext(ctx, sqlQuery, arguments...)
	trackQuery(ctx, "Exec", err, time.Since(start))
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (p *SqliteProvider) Query(ctx context.Context, sqlQuery string, optionsAndArgs ...any) (Rows, error) {
	start := time.Now()
	sqlQuery = convertBindVars(sqlQuery)
	rows, err := p.db.QueryContext(ctx, sqlQuery, optionsAndArgs...)
	trackQuery(ctx, "Query", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &SqliteRows{rows: rows}, nil
}

func (p *SqliteProvider) QueryRow(ctx context.Context, sqlQuery string, optionsAndArgs ...any) Row {
	start := time.Now()
	sqlQuery = convertBindVars(sqlQuery)
	row := p.db.QueryRowContext(ctx, sqlQuery, optionsAndArgs...)
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
	sqlQuery = convertBindVars(sqlQuery)
	res, err := t.tx.ExecContext(ctx, sqlQuery, arguments...)
	trackQuery(ctx, "Tx.Exec", err, time.Since(start))
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (t *SqliteTx) Query(ctx context.Context, sqlQuery string, optionsAndArgs ...any) (Rows, error) {
	start := time.Now()
	sqlQuery = convertBindVars(sqlQuery)
	rows, err := t.tx.QueryContext(ctx, sqlQuery, optionsAndArgs...)
	trackQuery(ctx, "Tx.Query", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &SqliteRows{rows: rows}, nil
}

func (t *SqliteTx) QueryRow(ctx context.Context, sqlQuery string, optionsAndArgs ...any) Row {
	start := time.Now()
	sqlQuery = convertBindVars(sqlQuery)
	row := t.tx.QueryRowContext(ctx, sqlQuery, optionsAndArgs...)
	trackQuery(ctx, "Tx.QueryRow", nil, time.Since(start))
	return &SqliteRow{row: row}
}

func (t *SqliteTx) Commit(ctx context.Context) error {
	return t.tx.Commit()
}

func (t *SqliteTx) Rollback(ctx context.Context) error {
	return t.tx.Rollback()
}
