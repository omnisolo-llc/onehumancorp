package db

import (
	"context"
	"database/sql"
	"fmt"
	"io/fs"
	"log/slog"
	"sort"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	_ "modernc.org/sqlite"
)

// SQLiteProvider implements the DatabaseProvider interface for SQLite.
type SQLiteProvider struct {
	db *sql.DB
}

// NewSQLite creates a new SQLite connection pool from the given DSN.
func NewSQLite(ctx context.Context, dsn string) (*SQLiteProvider, error) {
	// e.g. "file:ohc.db?cache=shared&mode=rwc"
	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to sqlite: %w", err)
	}

	if err := db.PingContext(ctx); err != nil {
		db.Close()
		return nil, fmt.Errorf("db: ping sqlite: %w", err)
	}

	// Enable WAL mode for better concurrency in SQLite
	if _, err := db.ExecContext(ctx, "PRAGMA journal_mode=WAL;"); err != nil {
		slog.Warn("db: failed to set WAL mode", "error", err)
	}

	slog.Info("db: connected to sqlite", "dsn", redactDSN(dsn))
	return &SQLiteProvider{db: db}, nil
}

// Close closes the database.
func (s *SQLiteProvider) Close() {
	if s.db != nil {
		s.db.Close()
	}
}

// Ping pings the database.
func (s *SQLiteProvider) Ping(ctx context.Context) error {
	return s.db.PingContext(ctx)
}

// SQLiteRows implements pgx.Rows to provide a unified interface.
type SQLiteRows struct {
	rows *sql.Rows
}

func (r *SQLiteRows) Close() {
	r.rows.Close()
}
func (r *SQLiteRows) Err() error {
	return r.rows.Err()
}
func (r *SQLiteRows) CommandTag() pgconn.CommandTag {
	return pgconn.CommandTag{}
}
func (r *SQLiteRows) FieldDescriptions() []pgconn.FieldDescription {
	return nil
}
func (r *SQLiteRows) Next() bool {
	return r.rows.Next()
}
func (r *SQLiteRows) Scan(dest ...any) error {
	return r.rows.Scan(dest...)
}
func (r *SQLiteRows) Values() ([]any, error) {
	cols, err := r.rows.Columns()
	if err != nil {
		return nil, err
	}

	values := make([]any, len(cols))
	pointers := make([]any, len(cols))
	for i := range values {
		pointers[i] = &values[i]
	}

	err = r.rows.Scan(pointers...)
	return values, err
}
func (r *SQLiteRows) RawValues() [][]byte {
	return nil // pgx.Rows doesn't require RawValues for most high-level usage, Values() is the critical one for iterating rows.
}
func (r *SQLiteRows) Conn() *pgx.Conn {
	return nil
}

type SQLiteRow struct {
	row *sql.Row
}
func (r *SQLiteRow) Scan(dest ...any) error {
	err := r.row.Scan(dest...)
	if err == sql.ErrNoRows {
		return pgx.ErrNoRows
	}
	return err
}


func (s *SQLiteProvider) Exec(ctx context.Context, query string, args ...any) (pgconn.CommandTag, error) {
	start := time.Now()
	query = convertBindVars(query)

	ctx, span := otel.Tracer("db").Start(ctx, "Exec")
	span.SetAttributes(attribute.String("db.system", "sqlite"), attribute.String("db.statement", query))
	defer span.End()

	res, err := s.db.ExecContext(ctx, query, args...)
	telemetry.RecordDBQuery(ctx, "Exec", "sqlite", time.Since(start).Seconds())
	if err != nil {
		span.RecordError(err)
		return pgconn.CommandTag{}, err
	}
	rowsAffected, _ := res.RowsAffected()
	tag := pgconn.NewCommandTag(fmt.Sprintf("UPDATE %d", rowsAffected))
	return tag, nil
}

func (s *SQLiteProvider) Query(ctx context.Context, query string, args ...any) (pgx.Rows, error) {
	start := time.Now()
	query = convertBindVars(query)

	ctx, span := otel.Tracer("db").Start(ctx, "Query")
	span.SetAttributes(attribute.String("db.system", "sqlite"), attribute.String("db.statement", query))
	defer span.End()

	rows, err := s.db.QueryContext(ctx, query, args...)
	telemetry.RecordDBQuery(ctx, "Query", "sqlite", time.Since(start).Seconds())
	if err != nil {
		span.RecordError(err)
		return nil, err
	}
	return &SQLiteRows{rows: rows}, nil
}

func (s *SQLiteProvider) QueryRow(ctx context.Context, query string, args ...any) pgx.Row {
	start := time.Now()
	query = convertBindVars(query)

	ctx, span := otel.Tracer("db").Start(ctx, "QueryRow")
	span.SetAttributes(attribute.String("db.system", "sqlite"), attribute.String("db.statement", query))
	defer span.End()

	row := s.db.QueryRowContext(ctx, query, args...)
	telemetry.RecordDBQuery(ctx, "QueryRow", "sqlite", time.Since(start).Seconds())
	return &SQLiteRow{row: row}
}


// SQLiteTx implements pgx.Tx
type SQLiteTx struct {
	tx *sql.Tx
}

func (tx *SQLiteTx) Commit(ctx context.Context) error {
	return tx.tx.Commit()
}
func (tx *SQLiteTx) Rollback(ctx context.Context) error {
	return tx.tx.Rollback()
}
func (tx *SQLiteTx) Exec(ctx context.Context, query string, args ...any) (pgconn.CommandTag, error) {
	start := time.Now()
	query = convertBindVars(query)

	ctx, span := otel.Tracer("db").Start(ctx, "TxExec")
	span.SetAttributes(attribute.String("db.system", "sqlite"), attribute.String("db.statement", query))
	defer span.End()

	res, err := tx.tx.ExecContext(ctx, query, args...)
	telemetry.RecordDBQuery(ctx, "TxExec", "sqlite", time.Since(start).Seconds())
	if err != nil {
		span.RecordError(err)
		return pgconn.CommandTag{}, err
	}
	rowsAffected, _ := res.RowsAffected()
	return pgconn.NewCommandTag(fmt.Sprintf("UPDATE %d", rowsAffected)), nil
}
func (tx *SQLiteTx) Query(ctx context.Context, query string, args ...any) (pgx.Rows, error) {
	start := time.Now()
	query = convertBindVars(query)

	ctx, span := otel.Tracer("db").Start(ctx, "TxQuery")
	span.SetAttributes(attribute.String("db.system", "sqlite"), attribute.String("db.statement", query))
	defer span.End()

	rows, err := tx.tx.QueryContext(ctx, query, args...)
	telemetry.RecordDBQuery(ctx, "TxQuery", "sqlite", time.Since(start).Seconds())
	if err != nil {
		span.RecordError(err)
		return nil, err
	}
	return &SQLiteRows{rows: rows}, nil
}
func (tx *SQLiteTx) QueryRow(ctx context.Context, query string, args ...any) pgx.Row {
	start := time.Now()
	query = convertBindVars(query)

	ctx, span := otel.Tracer("db").Start(ctx, "TxQueryRow")
	span.SetAttributes(attribute.String("db.system", "sqlite"), attribute.String("db.statement", query))
	defer span.End()

	row := tx.tx.QueryRowContext(ctx, query, args...)
	telemetry.RecordDBQuery(ctx, "TxQueryRow", "sqlite", time.Since(start).Seconds())
	return &SQLiteRow{row: row}
}

func (tx *SQLiteTx) Begin(ctx context.Context) (pgx.Tx, error) { return nil, fmt.Errorf("nested transactions not supported in SQLite") }
func (tx *SQLiteTx) CopyFrom(ctx context.Context, tableName pgx.Identifier, columnNames []string, rowSrc pgx.CopyFromSource) (int64, error) { return 0, nil }
func (tx *SQLiteTx) SendBatch(ctx context.Context, b *pgx.Batch) pgx.BatchResults { return nil }
func (tx *SQLiteTx) LargeObjects() pgx.LargeObjects { return pgx.LargeObjects{} }
func (tx *SQLiteTx) Prepare(ctx context.Context, name, sql string) (*pgconn.StatementDescription, error) { return nil, nil }
func (tx *SQLiteTx) Conn() *pgx.Conn { return nil }

func (s *SQLiteProvider) Begin(ctx context.Context) (pgx.Tx, error) {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	return &SQLiteTx{tx: tx}, nil
}

// RunMigrations applies embedded migrations to the SQLite database.
func (s *SQLiteProvider) RunMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	if _, err := s.db.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`); err != nil {
		return fmt.Errorf("db: create schema_migrations: %w", err)
	}

	entries, err := fs.ReadDir(migrationsFS, "migrations")
	if err != nil {
		return fmt.Errorf("db: read embedded migrations: %w", err)
	}

	var files []string
	for _, e := range entries {
		if !e.IsDir() && strings.HasSuffix(e.Name(), ".sql") {
			files = append(files, e.Name())
		}
	}
	sort.Strings(files)

	for _, f := range files {
		// Check if already applied.
		var count int
		if err := s.db.QueryRowContext(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = ?", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sqlBytes, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		// replace some pg syntax with sqlite
		sqlStr := string(sqlBytes)
		sqlStr = strings.ReplaceAll(sqlStr, "TIMESTAMPTZ", "DATETIME")
		sqlStr = strings.ReplaceAll(sqlStr, "BIGSERIAL PRIMARY KEY", "INTEGER PRIMARY KEY AUTOINCREMENT")
		sqlStr = strings.ReplaceAll(sqlStr, "SERIAL PRIMARY KEY", "INTEGER PRIMARY KEY AUTOINCREMENT")
		sqlStr = strings.ReplaceAll(sqlStr, "DOUBLE PRECISION", "REAL")
		sqlStr = strings.ReplaceAll(sqlStr, "JSONB", "TEXT")
		sqlStr = strings.ReplaceAll(sqlStr, "BYTEA", "BLOB")
		sqlStr = strings.ReplaceAll(sqlStr, "TEXT[]", "TEXT")
		sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['*']", "'[\"*\"]'")
		sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read', 'write']", "'[\"read\", \"write\"]'")
		sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read']", "'[\"read\"]'")
		sqlStr = strings.ReplaceAll(sqlStr, "NOW()", "CURRENT_TIMESTAMP")




		// Remove ON CONFLICT (id) DO NOTHING for simple test insertions
		// or replace it if necessary. Actually SQLite supports ON CONFLICT (id) DO NOTHING from 3.24+
		// but the previous syntax error was around "ARRAY".

		tx, err := s.db.BeginTx(ctx, nil)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, sqlStr); err != nil {
			_ = tx.Rollback()
			return fmt.Errorf("db: exec migration %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, "INSERT INTO schema_migrations (filename) VALUES (?)", f); err != nil {
			_ = tx.Rollback()
			return fmt.Errorf("db: record migration %s: %w", f, err)
		}

		if err := tx.Commit(); err != nil {
			return fmt.Errorf("db: commit migration %s: %w", f, err)
		}

		slog.Info("db: applied migration", "file", f)
	}

	return nil
}

// Ensure SQLiteProvider implements DatabaseProvider
var _ DatabaseProvider = (*SQLiteProvider)(nil)

// convertBindVars replaces Postgres-style $1, $2, etc., with SQLite-style ?, ?, etc.
// Very basic implementation: just replaces $n with ?.
// In a robust solution, one would use regex to ensure it's not inside a string literal,
// but for standard SQL generation it will suffice here.
func convertBindVars(query string) string {
	var sb strings.Builder
	inString := false
	for i := 0; i < len(query); i++ {
		if query[i] == '\'' {
			inString = !inString
		}
		if !inString && query[i] == '$' {
			sb.WriteByte('?')
		} else {
			sb.WriteByte(query[i])
		}
	}
	res := sb.String()
	res = strings.ReplaceAll(res, "FOR UPDATE SKIP LOCKED", "")

	// Quick heuristic for RETURNING clauses: SQLite supports RETURNING only in 3.35.0+, but
	// for maximum compatibility with tests we might need it. Actually modernc.org/sqlite
	// is up to date and supports RETURNING.

	// Another issue is ON CONFLICT DO NOTHING without conflict target on PostgreSQL vs SQLite
	// But in Postgres ON CONFLICT DO NOTHING requires a target if you use ON CONFLICT (col),
	// SQLite supports ON CONFLICT (col) DO NOTHING.
	return res
}
