package config_test

import (
	"testing"

	"github.com/spf13/afero"
	"github.com/onehumancorp/mono/srcs/server/config"
)

func TestLoad_Defaults(t *testing.T) {
	config.Reset()
	t.Setenv("DATABASE_URL", "")
	t.Setenv("OHC_STANDALONE", "")
	t.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")

	cfg := config.Load()
	if cfg == nil {
		t.Fatal("expected non-nil config")
	}
	if cfg.ListenAddr != ":8080" {
		t.Errorf("expected default ListenAddr=:8080, got %q", cfg.ListenAddr)
	}
	if cfg.MaxTokens != 2048 {
		t.Errorf("expected default MaxTokens=2048, got %d", cfg.MaxTokens)
	}
	if cfg.S3BucketBlobs != "ohc-blobs" {
		t.Errorf("expected default S3BucketBlobs=ohc-blobs, got %q", cfg.S3BucketBlobs)
	}
	if cfg.Fs == nil {
		t.Error("expected non-nil Fs")
	}
	config.Reset()
}

func TestLoad_EnvVars(t *testing.T) {
	config.Reset()
	t.Setenv("DATABASE_URL", "postgres://localhost/testdb")
	t.Setenv("REDIS_URL", "redis://localhost:6379")
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")

	cfg := config.Load()
	if cfg.DatabaseURL != "postgres://localhost/testdb" {
		t.Errorf("expected DatabaseURL from env, got %q", cfg.DatabaseURL)
	}
	if cfg.RedisURL != "redis://localhost:6379" {
		t.Errorf("expected RedisURL from env, got %q", cfg.RedisURL)
	}
	if !cfg.StandaloneMode {
		t.Error("expected StandaloneMode=true")
	}
	config.Reset()
}

func TestOverride(t *testing.T) {
	config.Reset()
	memFs := afero.NewMemMapFs()
	cfg := &config.AppConfig{
		ListenAddr: ":9999",
		Fs:         memFs,
	}
	config.Override(cfg)

	got := config.Get()
	if got.ListenAddr != ":9999" {
		t.Errorf("expected overridden ListenAddr=:9999, got %q", got.ListenAddr)
	}
	config.Reset()
}
