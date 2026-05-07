package main

import (
	"context"
	"database/sql"
	"time"

	"net/http"

	"onehumancorp/srcs/server/memory"
	"onehumancorp/srcs/server/onboarding"
	"onehumancorp/srcs/server/orchestration"
	"onehumancorp/srcs/server/growth"
	"fmt"
	"os"

	_ "github.com/mattn/go-sqlite3"
)

// MockLLMClient implements memory.LLMClient for demonstration purposes
type MockLLMClient struct{}

// GenerateEmbedding returns a mock embedding
func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func main() {
	fmt.Println("Starting OHC Server...")

	// Initialize SQLite database (or Postgres in a real environment)
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to open database: %v\n", err)
		os.Exit(1)
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
		fmt.Fprintf(os.Stderr, "Failed to create memory_embeddings table: %v\n", err)
		os.Exit(1)
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
		fmt.Fprintf(os.Stderr, "Failed to initialize AutoDream daemon: %v\n", err)
		os.Exit(1)
	}

	// Run AutoDream daemon in background
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go daemon.Run(ctx)


	// Block forever (or implement graceful shutdown)

	// Initialize Onboarding
	tenantStore := onboarding.NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	onboardingService := onboarding.NewService(tenantStore, taskStore)
	onboardingAPI := onboarding.NewAPIHandler(onboardingService)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/start", onboardingAPI.HandleStartOnboarding)
	mux.HandleFunc("/api/onboarding/status", onboardingAPI.HandleGetStatus)

	growthSvc := growth.NewGrowthService()
	mux.HandleFunc("/api/growth/referrals/click", growthSvc.HandleReferralClick)
	mux.HandleFunc("/api/growth/referrals/convert", growthSvc.HandleReferralConvert)
	mux.HandleFunc("/api/growth/team-invites/accept", growthSvc.HandleTeamInviteAccept)

	go func() {
		if err := http.ListenAndServe(":8080", mux); err != nil {
			fmt.Fprintf(os.Stderr, "Server failed: %v\n", err)
			os.Exit(1)
		}
	}()

	select {}
}
