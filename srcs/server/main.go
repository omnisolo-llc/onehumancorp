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
	"onehumancorp/srcs/server/growth"
	"onehumancorp/srcs/server/builder"
	"onehumancorp/srcs/server/db"

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
	database, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		log.Fatalf("Failed to open database: %v", err)
	}
	defer database.Close()

	// Create memory_embeddings table
	_, err = database.Exec(`
		CREATE TABLE IF NOT EXISTS memory_embeddings (
			id TEXT PRIMARY KEY,
			content TEXT,
			vector_embedding BLOB
		)
	`)
	if err != nil {
		log.Fatalf("Failed to create memory_embeddings table: %v", err)
	}

	if db.GlobalProvider.IsSQLite() {
		// Create builder tables for sqlite testing fallback
		_, err = database.Exec(`
			CREATE TABLE IF NOT EXISTS builder_sites (
				id TEXT PRIMARY KEY,
				tenant_id TEXT,
				domain TEXT,
				custom_domain TEXT,
				status TEXT,
				created_at TEXT,
				updated_at TEXT
			);
			CREATE TABLE IF NOT EXISTS builder_pages (
				id TEXT PRIMARY KEY,
				site_id TEXT,
				tenant_id TEXT,
				path TEXT,
				title TEXT,
				created_at TEXT,
				updated_at TEXT
			);
			CREATE TABLE IF NOT EXISTS builder_blocks (
				id TEXT PRIMARY KEY,
				page_id TEXT,
				tenant_id TEXT,
				type TEXT,
				order_idx INTEGER,
				content TEXT,
				created_at TEXT,
				updated_at TEXT
			);
		`)
		if err != nil {
			log.Fatalf("Failed to create builder tables: %v", err)
		}
	} else {
		if err := db.RunBuilderMigrations(database); err != nil {
			log.Fatalf("Failed to run postgres builder migrations: %v", err)
		}
	}


	// Initialize the mock LLM client
	llmClient := &MockLLMClient{}

	// Initialize AutoDream daemon
	daemon, err := memory.NewAutoDreamDaemon(
		database,
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
	tenantStore := onboarding.NewSqliteTenantStore(database)
	taskStore := orchestration.NewSqliteTaskStore(database)
	onboardingService := onboarding.NewService(tenantStore, taskStore)
	onboardingAPI := onboarding.NewAPIHandler(onboardingService)

	builderStore := builder.NewSqlStore(database)
	builderAPI := builder.NewAPIHandler(builderStore, llmClient)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/start", onboardingAPI.HandleStartOnboarding)
	mux.HandleFunc("/api/onboarding/status", onboarding.TenantAuthMiddleware(onboardingAPI.HandleGetStatus))

	growthSvc := growth.NewGrowthService(database)
	mux.HandleFunc("/api/growth/referrals/click", growthSvc.HandleReferralClick)
	mux.HandleFunc("/api/growth/referrals/convert", growthSvc.HandleReferralConvert)
	mux.HandleFunc("/api/growth/team-invites/accept", growthSvc.HandleTeamInviteAccept)

	// Builder routes
	mux.HandleFunc("POST /api/builder/site", builder.TenantAuthMiddleware(builderAPI.HandleCreateSite)) // POST
	mux.HandleFunc("GET /api/builder/site/get", builder.TenantAuthMiddleware(builderAPI.HandleGetSite)) // GET
	mux.HandleFunc("POST /api/builder/page", builder.TenantAuthMiddleware(builderAPI.HandleCreatePage)) // POST
	mux.HandleFunc("GET /api/builder/page/get", builder.TenantAuthMiddleware(builderAPI.HandleGetPage)) // GET
	mux.HandleFunc("GET /api/builder/blocks", builder.TenantAuthMiddleware(builderAPI.HandleGetBlocks)) // GET
	mux.HandleFunc("POST /api/builder/block", builder.TenantAuthMiddleware(builderAPI.HandleCreateBlock)) // POST
	mux.HandleFunc("PUT /api/builder/block/update", builder.TenantAuthMiddleware(builderAPI.HandleUpdateBlock)) // PUT
	mux.HandleFunc("POST /api/builder/blocks/reorder", builder.TenantAuthMiddleware(builderAPI.HandleReorderBlocks)) // POST
	mux.HandleFunc("POST /api/builder/publish", builder.TenantAuthMiddleware(builderAPI.HandlePublishSite)) // POST

	go func() {
		log.Println("Listening on :8080...")
		if err := http.ListenAndServe(":8080", mux); err != nil {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	select {}
}
