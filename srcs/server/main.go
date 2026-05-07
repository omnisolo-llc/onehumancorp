package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"net/url"
	"os"
	"time"

	"net/http"

	"onehumancorp/srcs/server/memory"
	"onehumancorp/srcs/server/onboarding"
	"onehumancorp/srcs/server/orchestration"

	_ "github.com/mutecomm/go-sqlcipher/v4"
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
	key := os.Getenv("OHC_SQLITE_KEY")
	if key == "" {
		log.Fatalf("CRITICAL SECURITY ERROR: OHC_SQLITE_KEY must be set in Standalone Mode to ensure secure, encrypted SQLite storage.")
	}

	dsn := fmt.Sprintf("file::memory:?_pragma_key=%s&_pragma_cipher=sqlcipher", url.QueryEscape(key))
	db, err := sql.Open("sqlite3", dsn)
	if err != nil {
		log.Fatalf("Failed to open encrypted database: %v", err)
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

	// Initialize Onboarding
	tenantStore := onboarding.NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	onboardingService := onboarding.NewService(tenantStore, taskStore)
	onboardingAPI := onboarding.NewAPIHandler(onboardingService)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/start", onboardingAPI.HandleStartOnboarding)
	mux.HandleFunc("/api/onboarding/status", onboardingAPI.HandleGetStatus)

	go func() {
		log.Println("Listening on :8080...")
		if err := http.ListenAndServe(":8080", mux); err != nil {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	select {}
}
