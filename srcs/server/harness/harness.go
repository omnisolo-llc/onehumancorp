package harness

import (
	"context"
	"net/http"
	"os/exec"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// SandboxConfig holds the configuration for the bwrap harness.
type SandboxConfig struct {
	AllowedDomains []string
	DeniedDomains  []string
	ReadPaths      []string
	WritePaths     []string
	EnableSeccomp  bool
}

// Harness wraps agent execution using Bubblewrap.
type Harness struct {
	config *SandboxConfig
}

// NewHarness creates a new Harness.
func NewHarness(config *SandboxConfig) *Harness {
	return &Harness{
		config: config,
	}
}

// ServeHTTP acts as an integrated HTTP proxy to intercept and filter network requests.
func (h *Harness) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	// Simple proxy intercept
	host := r.Host
	if !strings.Contains(host, ":") {
		host = host + ":80" // default port
	}

	for _, denied := range h.config.DeniedDomains {
		if strings.Contains(host, denied) {
			http.Error(w, "Access Denied by Harness Proxy", http.StatusForbidden)
			return
		}
	}

	// If allowlist is populated, deny anything not in the allowlist
	if len(h.config.AllowedDomains) > 0 {
		allowed := false
		for _, allow := range h.config.AllowedDomains {
			if strings.Contains(host, allow) {
				allowed = true
				break
			}
		}
		if !allowed {
			http.Error(w, "Access Denied by Harness Proxy", http.StatusForbidden)
			return
		}
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Proxy Allow"))
}

// Run executes the command via bwrap, enforcing the read/write paths defined in the config.
func (h *Harness) Run(ctx context.Context, cmd string, args []string) error {
	bwrapArgs := []string{}

	bwrapArgs = append(bwrapArgs, "--ro-bind", "/", "/")

	for _, path := range h.config.ReadPaths {
		bwrapArgs = append(bwrapArgs, "--ro-bind", path, path)
	}

	for _, path := range h.config.WritePaths {
		bwrapArgs = append(bwrapArgs, "--bind", path, path)
	}

	bwrapArgs = append(bwrapArgs, "--unshare-net")

	bwrapArgs = append(bwrapArgs, cmd)
	bwrapArgs = append(bwrapArgs, args...)

	c := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	err := c.Run()

	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
		} else {
			exitCode = -1
		}
	}

	if telemetry.HarnessExecutionsTotal != nil {
		telemetry.HarnessExecutionsTotal.Add(ctx, 1, metric.WithAttributes(
			attribute.String("cmd", cmd),
			attribute.Int("exit_code", exitCode),
		))
	}

	return err
}
