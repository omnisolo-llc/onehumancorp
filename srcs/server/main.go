package main

import (
	"context"
	"fmt"
	"log/slog"
	"net"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"time"

	"github.com/onehumancorp/mono/srcs/server/sync"
	"github.com/redis/rueidis"

	"google.golang.org/grpc"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/dashboard"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/integrations/chatwoot"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/pipeline"
	"github.com/onehumancorp/mono/srcs/server/scheduler"
	"github.com/onehumancorp/mono/srcs/server/settings"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/onehumancorp/mono/srcs/server/workers"
)

const defaultAddress = ":8080"

type listenFunc func(string, http.Handler) error

// Retrieves an environment variable or returns a fallback value.
// Accepts parameters: key, fallback.
// Returns string.
// Produces no errors.
// Has no side effects.
func getEnvOrDefault(key, fallback string) string {
	if val, ok := os.LookupEnv(key); ok && val != "" {
		return ":" + val
	}
	return fallback
}

func getValueOrDefault(key, fallback string) string {
	if val, ok := os.LookupEnv(key); ok && val != "" {
		return val
	}
	return fallback
}

func envBoolDefault(key string, fallback bool) bool {
	value, ok := os.LookupEnv(key)
	if !ok {
		return fallback
	}

	switch value {
	case "1", "true", "TRUE", "yes", "YES", "on", "ON":
		return true
	case "0", "false", "FALSE", "no", "NO", "off", "OFF":
		return false
	default:
		return fallback
	}
}

func newHubAndTracker(pool *db.DB, orgID string) (*orchestration.Hub, *billing.Tracker) {
	if pool != nil {
		var hubRepo orchestration.HubRepository
		var taskRepo scheduler.TaskRepository
		var usageRepo billing.UsageRepository

		if pool.Provider.IsSQLite() {
			hubRepo = orchestration.NewSqliteHubRepository(pool.Provider, orgID)
			taskRepo = scheduler.NewSqliteTaskRepository(pool.Provider)
			usageRepo = billing.NewSqliteUsageRepository(pool.Provider, billing.DefaultCatalog)
		} else {
			hubRepo = orchestration.NewPgHubRepository(pool.Provider, orgID)
			taskRepo = scheduler.NewPgTaskRepository(pool.Provider)
			usageRepo = billing.NewPgUsageRepository(pool.Provider, billing.DefaultCatalog)
		}

		return orchestration.NewHubWithRepository(
				hubRepo,
				taskRepo,
			), billing.NewTrackerWithRepository(
				billing.DefaultCatalog,
				usageRepo,
			)
	}

	return orchestration.NewHub(), billing.NewTracker(billing.DefaultCatalog)
}

func bootstrapTenantOrganization(now time.Time) domain.Organization {
	org := domain.NewSoftwareCompany(
		getValueOrDefault("OHC_BOOTSTRAP_ORG_ID", "bootstrap"),
		getValueOrDefault("OHC_BOOTSTRAP_ORG_NAME", "Bootstrap Organization"),
		getValueOrDefault("OHC_BOOTSTRAP_CEO_NAME", "Platform Admin"),
		now.UTC(),
	)
	if configuredDomain := os.Getenv("OHC_BOOTSTRAP_ORG_DOMAIN"); configuredDomain != "" {
		org.Domain = configuredDomain
	}
	return org
}

var (
	nowUTC        = time.Now
	listenForMain = http.ListenAndServe
	fatalForMain  = func(err error) {
		panic(err)
	}
	initTelemetry = telemetry.InitTelemetry
	netListen     = net.Listen
	chatwootSetup = func(c *chatwoot.Client) error {
		return c.Setup()
	}
)

// Initializes structured JSON logging.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has side effects: Sets the default logger.
func init() {
	// Initialize structured JSON logging
	opts := &slog.HandlerOptions{
		Level: slog.LevelInfo,
	}
	var handler slog.Handler = slog.NewJSONHandler(os.Stdout, opts)
	// Provide unified logging across Cloud and Local standalone modes
	if os.Getenv("OHC_STANDALONE") == "true" {
		handler = slog.NewTextHandler(os.Stdout, opts)
	}
	logger := slog.New(handler)
	slog.SetDefault(logger)
}

// Creates a new demo system.
// Accepts parameters: now.
// Returns (domain.Organization, *orchestration.Hub, *billing.Tracker).
// Produces no errors.
// Has no side effects.
func newDemoSystem(now time.Time, hub *orchestration.Hub, tracker *billing.Tracker) domain.Organization {
	org := domain.NewSoftwareCompany("demo", "Demo Software Company", "Human CEO", now.UTC())
	hub.RegisterAgent(orchestration.Agent{ID: "pm-1", Name: "Product Manager", Role: "PRODUCT_MANAGER", OrganizationID: org.ID})
	hub.RegisterAgent(orchestration.Agent{ID: "swe-1", Name: "Software Engineer", Role: "SOFTWARE_ENGINEER", OrganizationID: org.ID})
	hub.RegisterAgent(orchestration.Agent{ID: "news-1", Name: "AI News Collector", Role: "AI_NEWS_COLLECTOR", Status: orchestration.StatusActive, OrganizationID: org.ID})
	hub.OpenMeeting("kickoff", []string{"pm-1", "swe-1"})

	if tracker.Summary(org.ID).TotalTokens == 0 {
		_, _ = tracker.Track(billing.Usage{
			AgentID:          "swe-1",
			AgentRole:        "SOFTWARE_ENGINEER",
			OrganizationID:   org.ID,
			Model:            "gpt-4o-mini",
			PromptTokens:     1500,
			CompletionTokens: 700,
			OccurredAt:       now.UTC(),
		})
	}

	return org
}

// Creates a new demo handler.
// Accepts parameters: now.
// Returns (http.Handler, *orchestration.Hub).
// Produces no errors.
// Has no side effects.
func newDemoHandler(now time.Time, hub *orchestration.Hub, tracker *billing.Tracker, authStore *auth.Store) http.Handler {
	org := newDemoSystem(now, hub, tracker)
	if authStore == nil {
		authStore = auth.NewStore()
	}

	return dashboard.NewServer(org, hub, tracker, authStore)
}

// Runs the API server.
// Accepts parameters: now, listen.
// Returns error.
// Produces errors: Returns an error if applicable.
// Has no side effects.
func run(now time.Time, listen listenFunc) error {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	multiTenant := envBoolDefault("OHC_MULTITENANT", false)
	headless := envBoolDefault("OHC_HEADLESS", false) || !envBoolDefault("OHC_SERVE_UI", true)

	pool, err := db.New(ctx)
	if err != nil {
		return err
	}
	if pool != nil {
		defer pool.Close()
		if err := pool.RunMigrations(ctx); err != nil {
			return err
		}
		slog.Info("using Postgres-backed repositories")
	} else {
		slog.Info("DATABASE_URL not set, using in-memory repositories")
	}
	if multiTenant && pool == nil {
		slog.Warn("multi-tenant mode enabled without Postgres; tenant state will remain process-local")
	}

	// 1. Initialize Settings
	configPath := filepath.Join(os.Getenv("HOME"), ".openclaw", "openclaw.json")
	store, err := settings.FromFile(configPath)
	if err != nil {
		slog.Warn("failed to load settings from file, using defaults", "path", configPath, "error", err)
		store = settings.NewStore()
	}

	// 2. Initialize core stores
	var (
		hub       *orchestration.Hub
		tracker   *billing.Tracker
		authStore *auth.Store
		sipdb     *orchestration.SIPDB
	)

	hub, tracker = newHubAndTracker(pool, "")
	hub.GetTokenUsage = func(ctx context.Context) map[string]int64 {
		usage := make(map[string]int64)
		for _, orgID := range tracker.ActiveOrganizations(ctx) {
			usage[orgID] = tracker.Summary(orgID).TotalTokens
		}
		return usage
	}

	if pool != nil {
		authStore = auth.NewStoreWithRepository(auth.NewPgUserRepository(pool.Provider))
	} else {
		authStore = auth.NewStore()
	}
	hub.SetSettingsStore(store)
	baseSettings := store.Get()

	if chatwoot.IsEnabled() {
		go func() {
			c := chatwoot.NewClientFromEnv()
			if err := chatwootSetup(c); err != nil {
				slog.Error("chatwoot setup", "error", err)
			}
		}()
	}

	// Set up the SIPDB instance to connect to Database via Provider.
	// If we have a pool connection, use it for seamless SQLite / Postgres multi-target support.
	var createdSIPDB *orchestration.SIPDB
	var sipdbErr error

	if pool != nil {
		createdSIPDB, sipdbErr = orchestration.NewSIPDBWithProvider(pool.Provider, "system")
	} else {
		var dbPath string
		if os.Getenv("OHC_STANDALONE") == "true" {
			runtimeDir := os.Getenv("OHC_RUNTIME_DIR")
			if runtimeDir == "" {
				runtimeDir = filepath.Join(".ohc", "runtime")
			}
			if err := os.MkdirAll(runtimeDir, 0700); err != nil {
				slog.Warn("failed to create standalone runtime directory", "error", err)
			}
			dbPath = filepath.Join(runtimeDir, "swarm.db")
		} else {
			openclawDir := filepath.Join(os.Getenv("HOME"), ".openclaw")
			if err := os.MkdirAll(openclawDir, 0700); err != nil {
				slog.Warn("failed to create .openclaw directory", "error", err)
			}
			dbPath = filepath.Join(openclawDir, "ohc.db")
		}
		createdSIPDB, sipdbErr = orchestration.NewSIPDB(dbPath)
	}

	// Initialize Rueidis client if REDIS_URL is provided
	var redisClient rueidis.Client
	if redisURL := os.Getenv("REDIS_URL"); redisURL != "" {
		if opts, err := rueidis.ParseURL(redisURL); err == nil {
			redisClient, _ = rueidis.NewClient(opts)
		}
	}

	if pool != nil {
		autodreamWorker := orchestration.NewAutoDreamWorker(pool.Provider)
		autodreamWorker.Start(ctx)

		missionIngestionWorker := workers.NewMissionIngestionWorker(pool.Provider)
		go missionIngestionWorker.Start(ctx)

		competitorAuditWorker := workers.NewCompetitorAuditWorker(pool.Provider)
		go competitorAuditWorker.Start(ctx)

		autodreamPipeline := pipeline.NewAutoDreamPipeline(pool.Provider, redisClient)
		go autodreamPipeline.Start(ctx)
	}

	// Run Token Burn Rate Forecasting Engine
	if tracker != nil {
		go orchestration.StartTokenBurnForecaster(
			ctx,
			func(c context.Context) []string { return tracker.ActiveOrganizations(c) },
			func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens },
		)
	}

	if sipdbErr == nil {
		sipdb = createdSIPDB
		hub.SetSIPDB(sipdb)

		if os.Getenv("OHC_STANDALONE") == "true" {
			telemetry.BufferMetricFunc = sipdb.BufferMetric

			// Setup AutoDreamSyncEngine
			syncCloudAPI := os.Getenv("OHC_CLOUD_AUTODREAM_ENDPOINT")
			if syncCloudAPI != "" && pool != nil {
				slog.Info("starting autodream sync engine", "endpoint", syncCloudAPI)
				autodreamSyncEngine := sync.NewAutoDreamSyncEngine(pool, 1*time.Minute, syncCloudAPI)
				autodreamSyncEngine.Start(ctx)
			}

			// Background sync for standalone metrics to cloud
			cloudEndpoint := os.Getenv("OHC_CLOUD_TELEMETRY_ENDPOINT")
			if cloudEndpoint != "" && envBoolDefault("OHC_TELEMETRY_ENABLED", false) {
				telemetry.StartSyncDaemon(ctx, sipdb.SyncBufferedMetrics, cloudEndpoint, 5*time.Minute)
			}
			// Background sync for standalone missions to cloud
			missionsEndpoint := os.Getenv("OHC_CLOUD_MISSIONS_ENDPOINT")
			if missionsEndpoint != "" {
				go func() {
					ticker := time.NewTicker(2 * time.Second)
					defer ticker.Stop()
					for {
						select {
						case <-ctx.Done():
							return
						case <-ticker.C:
							syncedCount, err := sipdb.SyncMissions(ctx, missionsEndpoint)
							if err != nil {
								slog.Warn("Failed to sync standalone missions", "error", err)
							} else if syncedCount > 0 {
								slog.Debug("Successfully synced standalone missions to cloud", "count", syncedCount)
							}
						}
					}
				}()
			}

			// Background sync for Hybrid MCP RAG state to cloud orchestration engine
			contextEndpoint := os.Getenv("OHC_CLOUD_CONTEXT_ENDPOINT")
			if contextEndpoint != "" {
				go func() {
					ticker := time.NewTicker(5 * time.Second)
					defer ticker.Stop()
					for {
						select {
						case <-ctx.Done():
							return
						case <-ticker.C:
							syncedCount, err := sipdb.SyncContextSync(ctx, contextEndpoint)
							if err != nil {
								slog.Warn("Failed to sync standalone RAG context", "error", err)
							} else if syncedCount > 0 {
								slog.Debug("Successfully synced standalone RAG context to cloud", "count", syncedCount)
							}
						}
					}
				}()
			}
		}

		// Hygiene: Prune stale missions in the agent_missions table periodically
		go func() {
			ticker := time.NewTicker(1 * time.Hour)
			defer ticker.Stop()
			for {
				select {
				case <-ctx.Done():
					return
				case <-ticker.C:
					// Prune missions older than 7 days or marked COMPLETED
					if err := sipdb.PruneStaleMissions(ctx, 7*24*time.Hour); err != nil {
						slog.Error("failed to prune stale agent missions", "error", err)
					} else {
						slog.Debug("successfully pruned stale agent missions")
					}
				}
			}
		}()
	} else {
		slog.Error("failed to initialize SIPDB", "error", sipdbErr)
	}

	var handler http.Handler
	if multiTenant {
		// Initialize CentrifugeNode once globally to avoid redundant initializations in multi-tenant mode
		var globalCentrifugeNode *orchestration.CentrifugeNode
		if cnNode, err := orchestration.NewCentrifugeNode(); err == nil {
			globalCentrifugeNode = cnNode
		} else {
			slog.Warn("global centrifuge node init failed; real-time WebSocket disabled", "error", err)
		}

		factory := func(org domain.Organization) http.Handler {
			tenantHub, tenantTracker := newHubAndTracker(pool, org.ID)
			tenantHub.GetTokenUsage = func(ctx context.Context) map[string]int64 {
				usage := make(map[string]int64)
				for _, orgID := range tenantTracker.ActiveOrganizations(ctx) {
					usage[orgID] = tenantTracker.Summary(orgID).TotalTokens
				}
				return usage
			}
			tenantSettings := settings.NewStore()
			_ = tenantSettings.Update(baseSettings)
			tenantHub.SetSettingsStore(tenantSettings)
			// Create a tenant-scoped SIPDB instance to enforce row-level tenant isolation.
			if pool != nil {
				if tenantSIPDB, err := orchestration.NewSIPDBWithProvider(pool.Provider, org.ID); err == nil {
					tenantHub.SetSIPDB(tenantSIPDB)
				}
			}
			if globalCentrifugeNode != nil {
				tenantHub.SetCentrifugeNode(globalCentrifugeNode)
			}
			return dashboard.NewServer(org, tenantHub, tenantTracker, authStore)
		}

		registry, multiTenantHandler := dashboard.NewMultiTenantServerWithRegistry(authStore, factory)
		bootstrapOrg := bootstrapTenantOrganization(now)
		registry.Provision(bootstrapOrg)
		handler = multiTenantHandler
		slog.Info("using multi-tenant dashboard server", "bootstrap_org", bootstrapOrg.ID, "headless", headless)
	} else {
		handler = newDemoHandler(now, hub, tracker, authStore)
		slog.Info("using single-tenant dashboard server", "headless", headless)
	}

	// 4. Start Scheduler Background Task
	go hub.Scheduler().StartBackgroundTask(ctx, func(task scheduler.Task) {
		slog.Info("executing scheduled task", "task_id", task.ID, "name", task.Name)
		// Mark as running
		if _, err := hub.Scheduler().MarkRunning(task.OrganizationID, task.ID); err != nil {
			slog.Error("failed to mark task as running", "task_id", task.ID, "error", err)
			return
		}

		// Simulate task execution by publishing a message
		msg := orchestration.Message{
			ID:         task.ID + "-" + fmt.Sprintf("%d", time.Now().Unix()),
			FromAgent:  "system-scheduler",
			ToAgent:    task.AgentID,
			Type:       orchestration.EventTask,
			Content:    fmt.Sprintf("Scheduled Task triggered: %s. Payload: %s", task.Name, string(task.Payload)),
			OccurredAt: time.Now().UTC(),
		}

		// In a real scenario, we'd need to register 'system-scheduler' or similar.
		// For this migration, we'll just log and mark done for now.
		err := hub.Publish(msg)
		if err != nil {
			slog.Error("failed to publish scheduled task message", "task_id", task.ID, "error", err)
			_ = hub.Scheduler().MarkDone(task.OrganizationID, task.ID, false)
		} else {
			_ = hub.Scheduler().MarkDone(task.OrganizationID, task.ID, true)
		}
	})

	// 5. Start the builtin agent process (Rust binary).
	// The Rust binary connects to the gRPC server and self-registers.
	grpcAddress := getEnvOrDefault("GRPC_PORT", ":9090")
	grpcEndpoint := "http://localhost" + grpcAddress
	startBuiltinAgentProcess(ctx, grpcEndpoint)
	httpAddress := getEnvOrDefault("PORT", defaultAddress)

	// Setup MeshTransport for Hub
	var mesh orchestration.MeshTransport
	if redisURL := os.Getenv("REDIS_URL"); redisURL != "" && os.Getenv("OHC_STANDALONE") != "true" {
		rm, err := orchestration.NewRedisMeshTransport(redisURL)
		if err == nil {
			mesh = rm
		}
	}
	if mesh == nil {
		if pool != nil {
			mesh = orchestration.NewMemoryMeshTransport(pool.Provider)
		} else {
			mesh = orchestration.NewMemoryMeshTransport(nil)
		}
	}

	if cn := hub.CentrifugeNode(); cn != nil {
		cn.SetMeshTransport(mesh)
	}

	// Start gRPC server
	go func() {
		lis, err := netListen("tcp", grpcAddress)
		if err != nil {
			slog.Error("failed to listen for gRPC", "error", err)
			return
		}
		s := grpc.NewServer(
			grpc.UnaryInterceptor(orchestration.SPIFFEAuthInterceptor()),
			grpc.StreamInterceptor(orchestration.SPIFFEStreamInterceptor()),
		)
		orchestration.RegisterHubService(s, hub, mesh)
		slog.Info("serving gRPC", "address", grpcAddress)
		if err := s.Serve(lis); err != nil {
			slog.Error("failed to serve gRPC", "error", err)
		}
	}()

	slog.Info("serving API", "address", httpAddress)
	return listen(httpAddress, handler)
}

// Entry point for the application.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
func main() {
	shutdown, err := initTelemetry()
	if err != nil {
		slog.Warn("failed to initialize telemetry", "error", err)
	} else {
		defer shutdown()
	}

	if err := run(nowUTC().UTC(), listenForMain); err != nil {
		fatalForMain(err)
	}
}

// startBuiltinAgentProcess spawns the Rust builtin agent binary as a subprocess.
// The Rust binary connects back to the gRPC server and self-registers.
// It is restarted automatically if it exits unexpectedly.
func startBuiltinAgentProcess(ctx context.Context, grpcEndpoint string) {
	binaryPath := os.Getenv("OHC_BUILTIN_AGENT_BINARY")
	if binaryPath == "" {
		exe, err := os.Executable()
		if err == nil {
			binaryPath = filepath.Join(filepath.Dir(exe), "ohc-builtin-agent")
		}
	}
	if binaryPath == "" {
		binaryPath = "ohc-builtin-agent"
	}

	if _, err := os.Stat(binaryPath); os.IsNotExist(err) {
		slog.Warn("builtin agent binary not found, agent subprocess will not start", "path", binaryPath)
		return
	}

	go func() {
		for {
			cmd := exec.CommandContext(ctx, binaryPath)
			cmd.Env = append(os.Environ(),
				"OHC_GRPC_ENDPOINT="+grpcEndpoint,
			)
			cmd.Stdout = os.Stdout
			cmd.Stderr = os.Stderr

			slog.Info("starting builtin agent process", "path", binaryPath)
			if err := cmd.Run(); err != nil {
				if ctx.Err() != nil {
					return
				}
				slog.Error("builtin agent process exited unexpectedly, restarting in 5s", "err", err)
				select {
				case <-ctx.Done():
					return
				case <-time.After(5 * time.Second):
				}
			}
		}
	}()
}
