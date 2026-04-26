//go:build !standalone

package config

// standaloneEnforce is a no-op in non-standalone builds.
// In standalone builds (see config_standalone.go), this function overrides
// DATABASE_URL and REDIS_URL to enforce SQLite + embedded NATS usage.
func standaloneEnforce(_ *AppConfig) {}
