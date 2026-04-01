package db

import (
	"bytes"
	"context"
	"database/sql"
	"strings"
	"time"

	_ "modernc.org/sqlite"
)

// convertBindVars translates PostgreSQL-style $1, $2 placeholders to SQLite ?1, ?2,
// correctly ignoring placeholders inside string literals, and strips natively unsupported
// clauses like 'FOR UPDATE SKIP LOCKED'.
func convertBindVars(query string) string {
	var buf bytes.Buffer
	inString := false
	inIdentifier := false

	// Fast path check to avoid allocation if possible
	if !strings.Contains(query, "$") && !strings.Contains(query, "FOR UPDATE") {
		return query
	}

	// We only want to strip FOR UPDATE when it's part of the query syntax, not inside a string.
	// Since FOR UPDATE comes at the very end of queries, we can check for it outside strings.
	// Actually, just ignore FOR UPDATE logic here and strip it by checking if it's not in string.
	// We'll build the new query character by character.

	// A simpler way to strip "FOR UPDATE SKIP LOCKED" is just to look for it, but to avoid replacing
	// inside string literals, we must be careful.
	// Wait, if it's "FOR UPDATE SKIP LOCKED", it's usually at the end.

	// Let's implement it carefully.
	for i := 0; i < len(query); i++ {
		c := query[i]

		if c == '\'' && !inIdentifier {
			inString = !inString
		} else if c == '"' && !inString {
			inIdentifier = !inIdentifier
		}

		if c == '$' && !inString && !inIdentifier {
			buf.WriteByte('?')
		} else {
			// Basic check for FOR UPDATE
			if !inString && !inIdentifier && c == 'F' && i+10 <= len(query) && query[i:i+10] == "FOR UPDATE" {
				if i+22 <= len(query) && query[i:i+22] == "FOR UPDATE SKIP LOCKED" {
					i += 21
					continue
				} else {
					i += 9
					continue
				}
			}
			buf.WriteByte(c)
		}
	}

	return buf.String()
}

// SqliteProvider implements the Provider interface using database/sql with modernc.org/sqlite.
type SqliteProvider struct {
	db *sql.DB
}

func NewSqliteProvider(db *sql.DB) *SqliteProvider {
	return &SqliteProvider{db: db}
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

func (p *SqliteProvider) IsSQLite() bool {
	return true
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
