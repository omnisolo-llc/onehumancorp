// Package config provides centralised configuration loading for the OHC
// server.  All environment variables and optional config-file keys are
// declared here so callers never use os.Getenv directly.
//
// Viper is used as the underlying configuration engine because it supports
// env-var binding, YAML/TOML/JSON config files, defaults, and hot-reload.
// At start-up the binary calls config.Load() once; all other packages call
// the typed accessor functions (Config.DatabaseURL, etc.) on the singleton
// returned by config.Get().
//
// # Environment variables
//
// The canonical variable names are shown next to each field.  Viper also
// supports a YAML config file at the path set by OHC_CONFIG_FILE.
package config

import (
	"log/slog"
	"strings"
	"sync"

	"github.com/spf13/afero"
	"github.com/spf13/viper"
)

// AppConfig holds the full, parsed runtime configuration.
type AppConfig struct {
	// Server
	ListenAddr string // OHC_LISTEN_ADDR  (default: ":8080")
	GRPCAddr   string // OHC_GRPC_ADDR    (default: ":9090")

	// Database
	DatabaseURL        string // DATABASE_URL  – postgres DSN or "sqlite://<path>"
	StandaloneMode     bool   // OHC_STANDALONE
	SQLiteEncryptionKey string // OHC_SQLITE_ENCRYPTION_KEY

	// Redis / Valkey message bus
	RedisURL string // REDIS_URL

	// Multi-tenancy
	MultiTenant bool // OHC_MULTITENANT
	Headless    bool // OHC_HEADLESS

	// AI / LLM
	MinimaxAPIKey      string // MINIMAX_API_KEY
	AnthropicAPIKey    string // ANTHROPIC_API_KEY
	OpenAIAPIKey       string // OPENAI_API_KEY
	LLMProvider        string // OHC_LLM_PROVIDER  (e.g. "anthropic","openai","ollama")
	LLMModel           string // OHC_LLM_MODEL
	LLMEndpoint        string // OHC_LOCAL_LLM_ENDPOINT
	MaxTokens          int    // OHC_MAX_TOKENS    (default: 2048)
	MaxIterations      int    // OHC_MAX_ITERATIONS
	MaxContextMessages int    // OHC_MAX_CONTEXT_MESSAGES

	// Agent authentication
	AgentToken        string // OHC_AGENT_TOKEN
	AgentAuthDisabled bool   // OHC_AGENT_AUTH_DISABLED
	AgentCertFile     string // OHC_AGENT_CERT_FILE
	AgentKeyFile      string // OHC_AGENT_KEY_FILE
	AgentCAFile       string // OHC_AGENT_CA_FILE
	AgentSPIFFEID     string // OHC_AGENT_SPIFFE_ID
	AgentAddress      string // OHC_AGENT_ADDRESS  (default: "127.0.0.1:50051")
	AgentID           string // OHC_AGENT_ID
	BuiltinAgentBinary string // OHC_BUILTIN_AGENT_BINARY

	// Cloud sync endpoints
	CloudAutoDreamEndpoint  string // OHC_CLOUD_AUTODREAM_ENDPOINT
	CloudTelemetryEndpoint  string // OHC_CLOUD_TELEMETRY_ENDPOINT
	CloudMissionsEndpoint   string // OHC_CLOUD_MISSIONS_ENDPOINT
	CloudContextEndpoint    string // OHC_CLOUD_CONTEXT_ENDPOINT
	TelemetryEnabled        bool   // OHC_TELEMETRY_ENABLED

	// Bootstrap organisation
	BootstrapOrgID     string // OHC_BOOTSTRAP_ORG_ID    (default: "bootstrap")
	BootstrapOrgName   string // OHC_BOOTSTRAP_ORG_NAME  (default: "Bootstrap Organization")
	BootstrapCEOName   string // OHC_BOOTSTRAP_CEO_NAME  (default: "Platform Admin")
	BootstrapOrgDomain string // OHC_BOOTSTRAP_ORG_DOMAIN

	// JWT
	JWTSecret string // OHC_JWT_SECRET

	// Storage (S3 / Minio)
	S3Endpoint   string // OHC_S3_ENDPOINT
	S3BucketBlobs string // OHC_S3_BUCKET_BLOBS (default: "ohc-blobs")

	// Filesystem (via Afero) — only for legitimate file access, never .agent-task
	// Fs is the Afero filesystem implementation.  Tests can substitute a MemMapFs.
	Fs afero.Fs
}

var (
	once     sync.Once
	instance *AppConfig
)

// Load initialises the global config singleton from env vars (and optionally
// a YAML/TOML/JSON config file).  Calling Load multiple times is safe – it
// runs the actual initialisation only once.
func Load() *AppConfig {
	once.Do(func() {
		instance = loadViper()
	})
	return instance
}

// Get returns the global config singleton, calling Load if not yet done.
func Get() *AppConfig {
	if instance == nil {
		return Load()
	}
	return instance
}

// Override replaces the global singleton.  Use in tests only.
func Override(cfg *AppConfig) {
	instance = cfg
}

// Reset clears the singleton so that the next call to Get or Load re-reads
// the environment.  Use in tests only.
func Reset() {
	once = sync.Once{}
	instance = nil
}

// loadViper sets up viper with all defaults and env-var bindings, then
// deserialises into AppConfig.
func loadViper() *AppConfig {
	v := viper.New()

	// Allow an optional config file; viper.ReadInConfig is a no-op if missing.
	v.SetConfigName("ohc")
	v.AddConfigPath(".")
	v.AddConfigPath("$HOME/.openclaw")
	if cfgFile := v.GetString("OHC_CONFIG_FILE"); cfgFile != "" {
		v.SetConfigFile(cfgFile)
	}
	if err := v.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			slog.Warn("config: failed to read config file", "error", err)
		}
	}

	// Map env-var names → viper keys (upper-snake → lower-dot convention).
	v.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	v.AutomaticEnv()

	// Explicit env bindings for keys that differ from the viper key.
	_ = v.BindEnv("database_url", "DATABASE_URL")
	_ = v.BindEnv("redis_url", "REDIS_URL")
	_ = v.BindEnv("minimax_api_key", "MINIMAX_API_KEY")
	_ = v.BindEnv("anthropic_api_key", "ANTHROPIC_API_KEY")
	_ = v.BindEnv("openai_api_key", "OPENAI_API_KEY")

	// Defaults
	v.SetDefault("listen_addr", ":8080")
	v.SetDefault("grpc_addr", ":9090")
	v.SetDefault("agent_address", "127.0.0.1:50051")
	v.SetDefault("max_tokens", 2048)
	v.SetDefault("s3_bucket_blobs", "ohc-blobs")
	v.SetDefault("bootstrap_org_id", "bootstrap")
	v.SetDefault("bootstrap_org_name", "Bootstrap Organization")
	v.SetDefault("bootstrap_ceo_name", "Platform Admin")

	cfg := &AppConfig{
		ListenAddr:             v.GetString("listen_addr"),
		GRPCAddr:               v.GetString("grpc_addr"),
		DatabaseURL:            v.GetString("database_url"),
		StandaloneMode:         v.GetBool("ohc_standalone"),
		SQLiteEncryptionKey:    v.GetString("ohc_sqlite_encryption_key"),
		RedisURL:               v.GetString("redis_url"),
		MultiTenant:            v.GetBool("ohc_multitenant"),
		Headless:               v.GetBool("ohc_headless"),
		MinimaxAPIKey:          v.GetString("minimax_api_key"),
		AnthropicAPIKey:        v.GetString("anthropic_api_key"),
		OpenAIAPIKey:           v.GetString("openai_api_key"),
		LLMProvider:            v.GetString("ohc_llm_provider"),
		LLMModel:               v.GetString("ohc_llm_model"),
		LLMEndpoint:            v.GetString("ohc_local_llm_endpoint"),
		MaxTokens:              v.GetInt("ohc_max_tokens"),
		MaxIterations:          v.GetInt("ohc_max_iterations"),
		MaxContextMessages:     v.GetInt("ohc_max_context_messages"),
		AgentToken:             v.GetString("ohc_agent_token"),
		AgentAuthDisabled:      v.GetBool("ohc_agent_auth_disabled"),
		AgentCertFile:          v.GetString("ohc_agent_cert_file"),
		AgentKeyFile:           v.GetString("ohc_agent_key_file"),
		AgentCAFile:            v.GetString("ohc_agent_ca_file"),
		AgentSPIFFEID:          v.GetString("ohc_agent_spiffe_id"),
		AgentAddress:           v.GetString("agent_address"),
		AgentID:                v.GetString("ohc_agent_id"),
		BuiltinAgentBinary:     v.GetString("ohc_builtin_agent_binary"),
		CloudAutoDreamEndpoint: v.GetString("ohc_cloud_autodream_endpoint"),
		CloudTelemetryEndpoint: v.GetString("ohc_cloud_telemetry_endpoint"),
		CloudMissionsEndpoint:  v.GetString("ohc_cloud_missions_endpoint"),
		CloudContextEndpoint:   v.GetString("ohc_cloud_context_endpoint"),
		TelemetryEnabled:       v.GetBool("ohc_telemetry_enabled"),
		BootstrapOrgID:         v.GetString("bootstrap_org_id"),
		BootstrapOrgName:       v.GetString("bootstrap_org_name"),
		BootstrapCEOName:       v.GetString("bootstrap_ceo_name"),
		BootstrapOrgDomain:     v.GetString("ohc_bootstrap_org_domain"),
		JWTSecret:              v.GetString("ohc_jwt_secret"),
		S3Endpoint:             v.GetString("ohc_s3_endpoint"),
		S3BucketBlobs:          v.GetString("s3_bucket_blobs"),
		Fs:                     afero.NewOsFs(),
	}

	if cfg.MaxTokens == 0 {
		cfg.MaxTokens = 2048
	}
	return cfg
}
