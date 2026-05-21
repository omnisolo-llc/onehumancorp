package sandbox

import (
	"context"
	"fmt"
	"net"
	"os"
	"path/filepath"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"
)

var tracer = otel.Tracer("sandbox")

// SandboxConfig holds configuration for the SandboxManager.
type SandboxConfig struct {
	AllowedReadPaths  []string
	AllowedWritePaths []string
	AllowedHosts      []string
	DangerouslyDisable bool
}

// SandboxManager handles filesystem read/write restrictions and network host filtering.
type SandboxManager struct {
	config SandboxConfig
}

// NewSandboxManager creates a new SandboxManager with the given configuration.
func NewSandboxManager(config SandboxConfig) *SandboxManager {
	return &SandboxManager{
		config: config,
	}
}

// CheckRead ensures the given path is allowed to be read.
func (sm *SandboxManager) CheckRead(ctx context.Context, path string) error {
	ctx, span := tracer.Start(ctx, "CheckRead", trace.WithAttributes(attribute.String("path", path)))
	defer span.End()

	if sm.config.DangerouslyDisable {
		span.AddEvent("Sandbox dangerously disabled, allowing read")
		return nil
	}

	cleanPath := filepath.Clean(path)
	for _, allowed := range sm.config.AllowedReadPaths {
		cleanAllowed := filepath.Clean(allowed)
		if cleanPath == cleanAllowed || strings.HasPrefix(cleanPath, cleanAllowed+string(filepath.Separator)) {
			return nil
		}
	}

	err := fmt.Errorf("read access denied for path: %s", path)
	span.RecordError(err)
	return err
}

// CheckWrite ensures the given path is allowed to be written to.
func (sm *SandboxManager) CheckWrite(ctx context.Context, path string) error {
	ctx, span := tracer.Start(ctx, "CheckWrite", trace.WithAttributes(attribute.String("path", path)))
	defer span.End()

	if sm.config.DangerouslyDisable {
		span.AddEvent("Sandbox dangerously disabled, allowing write")
		return nil
	}

	cleanPath := filepath.Clean(path)
	for _, allowed := range sm.config.AllowedWritePaths {
		cleanAllowed := filepath.Clean(allowed)
		if cleanPath == cleanAllowed || strings.HasPrefix(cleanPath, cleanAllowed+string(filepath.Separator)) {
			return nil
		}
	}

	err := fmt.Errorf("write access denied for path: %s", path)
	span.RecordError(err)
	return err
}

// CheckNetwork ensures the given host is allowed to be accessed.
func (sm *SandboxManager) CheckNetwork(ctx context.Context, host string) error {
	ctx, span := tracer.Start(ctx, "CheckNetwork", trace.WithAttributes(attribute.String("host", host)))
	defer span.End()

	if sm.config.DangerouslyDisable {
		span.AddEvent("Sandbox dangerously disabled, allowing network")
		return nil
	}

	// Handle case where host includes port (e.g., "example.com:80")
	hostname := host
	if h, _, err := net.SplitHostPort(host); err == nil {
		hostname = h
	}

	for _, allowed := range sm.config.AllowedHosts {
		if hostname == allowed {
			return nil
		}
	}

	err := fmt.Errorf("network access denied for host: %s", host)
	span.RecordError(err)
	return err
}

// ReadFile is a sandboxed wrapper for os.ReadFile
func (sm *SandboxManager) ReadFile(ctx context.Context, filename string) ([]byte, error) {
	if err := sm.CheckRead(ctx, filename); err != nil {
		return nil, err
	}
	return os.ReadFile(filename)
}
