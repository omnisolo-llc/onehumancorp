package main

import (
	"context"
	"database/sql"
	"log"
	"time"

	"net/http"

	"onehumancorp/srcs/server/memory"
	"onehumancorp/srcs/server/onboarding"
	"onehumancorp/srcs/server/orchestration"

	_ "github.com/mattn/go-sqlite3"
)

// MockLLMClient implements memory.LLMClient for demonstration purposes
type MockLLMClient struct{}

// GenerateEmbedding returns a mock embedding
func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

// Mock auth middleware to satisfy the prompt's `auth.Middleware` requirement locally
func mockAuthMiddleware(next http.HandlerFunc) http.HandlerFunc {
    return func(w http.ResponseWriter, r *http.Request) {
        next.ServeHTTP(w, r)
    }
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

	// Orchestration Autodream
	autodreamWorker := orchestration.NewAutoDreamWorker(db)
	go autodreamWorker.Start(ctx, ".agent-task/memory", 5*time.Second)

	log.Println("OHC Server is running. AutoDream daemon started.")

	// Block forever (or implement graceful shutdown)

	// Initialize Onboarding
	tenantStore := onboarding.NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	onboardingService := onboarding.NewService(tenantStore, taskStore)
	onboardingAPI := onboarding.NewAPIHandler(onboardingService)

	// Mesh API
	meshHub := orchestration.NewLocalTeammateMesh()
	meshAPI := orchestration.NewMeshAPI(meshHub)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/start", onboardingAPI.HandleStartOnboarding)
	mux.HandleFunc("/api/onboarding/status", onboardingAPI.HandleGetStatus)
	mux.HandleFunc("/api/mesh/broadcast", mockAuthMiddleware(meshAPI.HandleBroadcast))

	go func() {
		log.Println("Listening on :8080...")
		if err := http.ListenAndServe(":8080", mux); err != nil {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	select {}
}
