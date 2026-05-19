package db

import (
	"database/sql"
	"log"
	"os"
	"strings"

	_ "github.com/mattn/go-sqlite3"
)

func InitDB() *sql.DB {
	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		dbURL = "sqlite://file::memory:?mode=memory"
	}

	var db *sql.DB
	var err error

	if strings.HasPrefix(dbURL, "sqlite") {
		path := strings.TrimPrefix(dbURL, "sqlite://")
		if path == dbURL {
			path = strings.TrimPrefix(dbURL, "sqlite:")
		}
		if path == "" {
			path = ":memory:"
		}

		db, err = sql.Open("sqlite3", path)
		if err != nil {
			log.Fatalf("failed to open sqlite: %v", err)
		}

		// Replace VECTOR(1536) fields with BLOB for SQLite vector embedding parity
		_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS embedding_cache (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			synced_to_cloud BOOLEAN DEFAULT false
		);
		`)
		if err != nil {
			log.Fatalf("failed to create table: %v", err)
		}
	} else {
		// Just a placeholder for other databases
	}

	return db
}
