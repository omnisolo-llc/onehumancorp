package main

import (
	"context"
	"fmt"
	"log/slog"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"google.golang.org/grpc"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/dashboard"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/integrations/chatwoot"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/scheduler"
	"github.com/onehumancorp/mono/srcs/server/settings"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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

func newHubAndTracker(pool *db.DB) (*orchestration.Hub, *billing.Tracker) {
	if pool != nil {
		var hubRepo orchestration.HubRepository
		var taskRepo scheduler.TaskRepository
		var usageRepo billing.UsageRepository

		switch pool.Provider.(type) {
		case *db.SqliteProvider:
			hubRepo = orchestration.NewSqliteHubRepository(pool.Provider)
			taskRepo = scheduler.NewSqliteTaskRepository(pool.Provider)
			usageRepo = billing.NewSqliteUsageRepository(pool.Provider, billing.DefaultCatalog)
		case *db.PgProvider:
			hubRepo = orchestration.NewPgHubRepository(pool.Provider)
			taskRepo = scheduler.NewPgTaskRepository(pool.Provider)
			usageRepo = billing.NewPgUsageRepository(pool.Provider, billing.DefaultCatalog)
		default:
			// Fallback if neither matches directly
			hubRepo = orchestration.NewPgHubRepository(pool.Provider)
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

	hub, tracker = newHubAndTracker(pool)
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

	// Set up the SIPDB instance to connect to SQLite.
	dbPath := filepath.Join(os.Getenv("HOME"), ".openclaw", "ohc.db")
	if createdSIPDB, err := orchestration.NewSIPDB(dbPath); err == nil {
		sipdb = createdSIPDB
		hub.SetSIPDB(sipdb)

		if os.Getenv("OHC_STANDALONE") == "true" {
			telemetry.BufferMetricFunc = sipdb.BufferMetric

			// Background sync for standalone metrics to cloud
			cloudEndpoint := os.Getenv("OHC_CLOUD_METRICS_ENDPOINT")
			if cloudEndpoint != "" {
				go func() {
					ticker := time.NewTicker(5 * time.Minute)
					defer ticker.Stop()
					for {
						select {
						case <-ctx.Done():
							return
						case <-ticker.C:
							for {
								syncedCount, err := sipdb.SyncBufferedMetrics(ctx, cloudEndpoint)
								if err != nil {
									slog.Warn("Failed to sync standalone metrics", "error", err)
									break
								}
								if syncedCount > 0 {
									slog.Info("Successfully synced standalone metrics to cloud", "count", syncedCount)
								}
								if syncedCount < 500 {
									break // No more batches
								}
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
						slog.Info("successfully pruned stale agent missions")
					}
				}
			}
		}()
	} else {
		slog.Error("failed to initialize SIPDB", "path", dbPath, "error", err)
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
			tenantHub, tenantTracker := newHubAndTracker(pool)
			tenantSettings := settings.NewStore()
			_ = tenantSettings.Update(baseSettings)
			tenantHub.SetSettingsStore(tenantSettings)
			if sipdb != nil {
				tenantHub.SetSIPDB(sipdb)
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
		if _, err := hub.Scheduler().MarkRunning(task.ID); err != nil {
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
			_ = hub.Scheduler().MarkDone(task.ID, false)
		} else {
			_ = hub.Scheduler().MarkDone(task.ID, true)
		}
	})

	grpcAddress := getEnvOrDefault("GRPC_PORT", ":9090")
	httpAddress := getEnvOrDefault("PORT", defaultAddress)

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
		orchestration.RegisterHubService(s, hub)
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
