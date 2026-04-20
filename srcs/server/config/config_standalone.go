//go:build standalone

// Package config provides standalone-specific config enforcement.
//
// When the server is compiled with the `standalone` build tag (used for the
// distributed desktop binary), this file replaces environment-variable-driven
// storage configuration with compile-time constants so that end-users cannot
// override the storage backend or message bus via environment variables.
//
// Specifically:
//   - DATABASE_URL is ignored; SQLite is always used.
//   - REDIS_URL is ignored; the embedded NATS JetStream message bus is always used.
//
// This guarantees that a standalone desktop distribution behaves consistently
// regardless of what environment variables the user may have set.
package config

import "log/slog"

// standaloneEnforce is called by Load in standalone builds to override any
// user-supplied DATABASE_URL or REDIS_URL with the hardcoded standalone
// values. It logs a warning if the user tried to set these variables.
func standaloneEnforce(cfg *AppConfig) {
	if cfg.DatabaseURL != "" && cfg.DatabaseURL != standaloneDefaultDatabaseURL() {
		slog.Warn("standalone: DATABASE_URL is ignored in standalone desktop builds; using SQLite",
			"ignored_value", cfg.DatabaseURL)
	}
	if cfg.RedisURL != "" {
		slog.Warn("standalone: REDIS_URL is ignored in standalone desktop builds; using embedded NATS",
			"ignored_value", cfg.RedisURL)
	}

	// Force SQLite and standalone mode.
	cfg.DatabaseURL = standaloneDefaultDatabaseURL()
	cfg.StandaloneMode = true
	cfg.RedisURL = "" // Signals the server to use the embedded NATS bus.
	cfg.MultiTenant = false
}

// standaloneDefaultDatabaseURL returns the SQLite database URL used in
// standalone desktop distributions. The database is stored in the OS user
// data directory.
func standaloneDefaultDatabaseURL() string {
	return "sqlite://ohc-standalone.db"
}
