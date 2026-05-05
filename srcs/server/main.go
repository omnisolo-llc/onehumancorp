package main

import (
	"context"
	"database/sql"
	"log"
	"time"

	"onehumancorp/srcs/server/memory"

	_ "github.com/mattn/go-sqlite3"
)

// MockLLMClient implements memory.LLMClient for demonstration purposes
type MockLLMClient struct{}

// GenerateEmbedding returns a mock embedding
func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func main() {
	log.Println("Starting OHC Server...")

	// Initialize SQLite database (or Postgres in a real environment)
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		log.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	// Create memory_embeddings table
	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS memory_embeddings (
			id TEXT PRIMARY KEY,
			content TEXT,
			vector_embedding BLOB
		)
	`)
	if err != nil {
		log.Fatalf("Failed to create memory_embeddings table: %v", err)
	}

	// Initialize the mock LLM client
	llmClient := &MockLLMClient{}

	// Initialize AutoDream daemon
	daemon, err := memory.NewAutoDreamDaemon(
		db,
		llmClient,
		".agent-task/memory",
		".agent-task/missions",
		5*time.Second,
	)
	if err != nil {
		log.Fatalf("Failed to initialize AutoDream daemon: %v", err)
	}

	// Run AutoDream daemon in background
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go daemon.Run(ctx)

	log.Println("OHC Server is running. AutoDream daemon started.")

	// Block forever (or implement graceful shutdown)
	select {}
}
