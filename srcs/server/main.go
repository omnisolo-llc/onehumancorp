package main

import (
	"context"
	"database/sql"
	"log"
	"net/http"
	"os"
	"time"

	"onehumancorp/srcs/server/memory"
	"onehumancorp/srcs/server/onboarding"
	"onehumancorp/srcs/server/orchestration"
	"onehumancorp/srcs/server/telemetry"
	"onehumancorp/srcs/server/growth"
	"onehumancorp/srcs/server/dashboard"
	"onehumancorp/srcs/server/tiers"
	"onehumancorp/srcs/server/services/sync"

	"go.opentelemetry.io/otel"
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

	// Initialize SQLite database
	dbPath := ":memory:"
	if os.Getenv("OHC_STANDALONE") == "true" {
		dbPath = "ohc_standalone.db"
	}

    // Pass key via DSN query parameter for sqlcipher
    dsn := dbPath
    if dbPath != ":memory:" {
        dbKey := os.Getenv("OHC_LOCAL_DB_KEY")
        if dbKey == "" {
            log.Fatalf("OHC_LOCAL_DB_KEY environment variable is required for local standalone mode encryption")
        }
        dsn = dbPath + "?_pragma_key=" + dbKey
    }

	db, err := sql.Open("sqlite3", dsn)
	if err != nil {
		log.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	if dbPath != ":memory:" {
		// Enforce secure file permissions for local standalone mode
		if err := os.Chmod(dbPath, 0600); err != nil {
			log.Printf("Warning: Failed to set secure permissions on %s: %v", dbPath, err)
		}
	}

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

	// Initialize telemetry sync engine
	telemetryEngine := telemetry.NewTelemetrySyncEngine(db, "https://api.onehumancorp.com/telemetry")
	telemetry.InitGlobalSyncEngine(telemetryEngine)
	go telemetryEngine.StartSyncDaemon(ctx, 30*time.Second)

	// Initialize Sync Escalator Daemon
	meter := otel.Meter("onehumancorp/sync-escalator")
	escalator, err := sync.InitWithMeter(db, meter)
	if err != nil {
		log.Fatalf("Failed to initialize sync escalator: %v", err)
	}
	go escalator.Start(ctx, 15*time.Second)

	log.Println("OHC Server is running. AutoDream daemon started.")

	// Block forever (or implement graceful shutdown)

	// Initialize Onboarding
	tenantStore := onboarding.NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	onboardingService := onboarding.NewService(tenantStore, taskStore)
	onboardingAPI := onboarding.NewAPIHandler(onboardingService)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/start", onboardingAPI.HandleStartOnboarding)
	mux.HandleFunc("/api/onboarding/status", onboarding.TenantAuthMiddleware(onboardingAPI.HandleGetStatus))
	mux.HandleFunc("/api/onboarding/state", onboarding.TenantAuthMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			onboardingAPI.HandleSaveState(w, r)
		} else if r.Method == http.MethodGet {
			onboardingAPI.HandleGetState(w, r)
		} else {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		}
	}))

	growthSvc := growth.NewGrowthService(db)
	mux.HandleFunc("/api/growth/referrals/click", growthSvc.HandleReferralClick)
	mux.HandleFunc("/api/growth/referrals/convert", growthSvc.HandleReferralConvert)
	mux.HandleFunc("/api/growth/team-invites/accept", growthSvc.HandleTeamInviteAccept)

	tierSvc := tiers.NewTierService(db)
	tierAPI := tiers.NewAPIHandler(tierSvc)
	mux.HandleFunc("/api/tiers/check", tierAPI.HandleCheckLimit)

	mux.HandleFunc("/api/dashboard/onboarding/metrics", dashboard.HandleOnboardingMetrics)

	syncAPI := sync.NewAPIHandler(escalator)
	mux.HandleFunc("/api/v1/orchestration/escalate", syncAPI.HandleEscalate)

	mux.HandleFunc("/api/v1/stream", dashboard.HandleStream)
	mux.HandleFunc("/api/v1/autodream/sync", dashboard.HandleAutoDreamSync)
	mux.HandleFunc("/api/v1/autodream/query", dashboard.HandleAutoDreamQuery)
	mux.HandleFunc("/api/mesh/broadcast", dashboard.HandleMeshBroadcast)

	go func() {
		log.Println("Listening on :8080...")
		if err := http.ListenAndServe(":8080", mux); err != nil {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	select {}
}
