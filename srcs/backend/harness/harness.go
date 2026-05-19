package harness

import (
	"context"
	"encoding/binary"
	"fmt"
	"os"
	"os/exec"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var tracer = otel.Tracer("ohc-harness")
var meter = otel.Meter("ohc-harness")

type SandboxConfig struct {
	AllowedDomains []string
	DeniedDomains  []string
	ReadPaths      []string
	WritePaths     []string
	EnableSeccomp  bool
}

type Harness struct {
	config     *SandboxConfig
	executions metric.Int64Counter
}

func NewHarness(config *SandboxConfig) (*Harness, error) {
	executions, err := meter.Int64Counter("ohc_harness_executions_total", metric.WithDescription("Total number of harness executions"))
	if err != nil {
		return nil, fmt.Errorf("failed to create metric: %w", err)
	}

	return &Harness{
		config:     config,
		executions: executions,
	}, nil
}

func createSeccompFilter() (*os.File, error) {
	// A simple BPF program that allows all syscalls.
	// Demonstrates dynamic Seccomp filtering generation.
	insts := []struct {
		Code uint16
		Jt   uint8
		Jf   uint8
		K    uint32
	}{
		{Code: 0x06, Jt: 0, Jf: 0, K: 0x7fff0000}, // BPF_RET | BPF_K, SECCOMP_RET_ALLOW
	}

	f, err := os.CreateTemp("", "seccomp-*.bpf")
	if err != nil {
		return nil, err
	}

	for _, inst := range insts {
		if err := binary.Write(f, binary.LittleEndian, inst); err != nil {
			f.Close()
			return nil, err
		}
	}

	if _, err := f.Seek(0, 0); err != nil {
		f.Close()
		return nil, err
	}

	return f, nil
}

func (h *Harness) Run(ctx context.Context, cmdName string, args []string) ([]byte, error) {
	ctx, span := tracer.Start(ctx, "harness.Run")
	defer span.End()

	h.executions.Add(ctx, 1, metric.WithAttributes(attribute.String("cmd", cmdName)))

	proxy := NewProxyServer(h.config.AllowedDomains, h.config.DeniedDomains)
	proxyAddr, err := proxy.Start()
	if err != nil {
		span.RecordError(err)
		return nil, fmt.Errorf("failed to start proxy: %w", err)
	}
	defer proxy.Stop()

	proxyUrl := fmt.Sprintf("http://%s", proxyAddr)

	bwrapArgs := []string{
		"--ro-bind", "/usr", "/usr",
		"--ro-bind", "/bin", "/bin",
		"--ro-bind", "/lib", "/lib",
		"--ro-bind", "/lib64", "/lib64",
		"--ro-bind", "/etc/resolv.conf", "/etc/resolv.conf",
		"--ro-bind", "/etc/ssl/certs", "/etc/ssl/certs",
		"--proc", "/proc",
		"--dev", "/dev",
		"--unshare-all",
		"--share-net",
		"--die-with-parent",
	}

	for _, path := range h.config.ReadPaths {
		bwrapArgs = append(bwrapArgs, "--ro-bind", path, path)
	}

	for _, path := range h.config.WritePaths {
		bwrapArgs = append(bwrapArgs, "--bind", path, path)
	}

	var extraFiles []*os.File
	if h.config.EnableSeccomp {
		seccompFile, err := createSeccompFilter()
		if err != nil {
			span.RecordError(err)
			return nil, fmt.Errorf("failed to create seccomp filter: %w", err)
		}
		defer os.Remove(seccompFile.Name())
		defer seccompFile.Close()

		extraFiles = append(extraFiles, seccompFile)
		// First ExtraFile gets file descriptor 3
		bwrapArgs = append(bwrapArgs, "--seccomp", "3")
	}

	bwrapArgs = append(bwrapArgs, "--", cmdName)
	bwrapArgs = append(bwrapArgs, args...)

	cmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)
	cmd.ExtraFiles = extraFiles
	cmd.Env = append(cmd.Environ(), "HTTP_PROXY="+proxyUrl, "HTTPS_PROXY="+proxyUrl, "http_proxy="+proxyUrl, "https_proxy="+proxyUrl)

	output, err := cmd.CombinedOutput()
	if err != nil {
		span.RecordError(err)
		if exitError, ok := err.(*exec.ExitError); ok {
			span.SetAttributes(attribute.Int("exit_code", exitError.ExitCode()))
		}
		return output, err
	}

	span.SetAttributes(attribute.Int("exit_code", 0))

	return output, nil
}
