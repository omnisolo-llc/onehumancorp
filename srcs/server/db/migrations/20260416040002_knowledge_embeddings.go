package migrations

import (
	"context"
	"database/sql"
	"log/slog"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upKnowledgeEmbeddings, downKnowledgeEmbeddings)
}

func upKnowledgeEmbeddings(ctx context.Context, tx *sql.Tx) error {
	dialect := goose.GetDialect()

	if dialect == "sqlite3" || dialect == "sqlite" {
		_, err := tx.Exec(`
			CREATE TABLE IF NOT EXISTS knowledge_embeddings (
				id TEXT PRIMARY KEY,
				content TEXT,
				embedding TEXT
			);
		`)
		return err
	}

	// PostgreSQL path
	_, err := tx.Exec(`CREATE EXTENSION IF NOT EXISTS vector;`)
	if err != nil {
		slog.Error("failed to create vector extension", "error", err)
		return err
	}
	_, err = tx.Exec(`
		CREATE TABLE IF NOT EXISTS knowledge_embeddings (
			id UUID PRIMARY KEY,
			content TEXT,
			embedding VECTOR(1536)
		);
	`)
	if err != nil {
		return err
	}
	_, err = tx.Exec(`CREATE INDEX IF NOT EXISTS knowledge_embeddings_idx ON knowledge_embeddings USING ivfflat (embedding vector_cosine_ops);`)
	return err
}

func downKnowledgeEmbeddings(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.Exec(`DROP TABLE IF EXISTS knowledge_embeddings;`)
	return err
}
