package db

import (
	"context"
	"database/sql"
	"regexp"
	"strings"
	"time"

	_ "modernc.org/sqlite"
)

// SqliteProvider implements the Provider interface using database/sql with modernc.org/sqlite.
type SqliteProvider struct {
	db *sql.DB
}

func NewSqliteProvider(db *sql.DB) *SqliteProvider {
	return &SqliteProvider{db: db}
}

var jsonPathRegex = regexp.MustCompile(`([a-zA-Z0-9_]+)(?:::json)?->>'([a-zA-Z0-9_]+)'`)

// convertBindVars parses PostgreSQL queries and translates them to SQLite syntax.
// It tracks string literal states to avoid replacing `$` inside quotes, maps Postgres
// positional parameters (e.g., `$1`) to SQLite numbered variables (e.g., `?1`),
// and dynamically strips natively unsupported clauses like `FOR UPDATE SKIP LOCKED`.
func convertBindVars(query string) string {
	query = strings.ReplaceAll(query, "FOR UPDATE SKIP LOCKED", "")

	// map JSON paths: col::json->>'key' OR col->>'key' => json_extract(col, '$.key')
	query = jsonPathRegex.ReplaceAllString(query, "json_extract($1, '$$.$2')")

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

	return result.String()
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

func (p *SqliteProvider) IsSQLite() bool {
	return true
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
