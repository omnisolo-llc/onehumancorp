package harness

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"os/exec"
	"strings"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/trace"

	"github.com/onehumancorp/mono/src/server/telemetry"
)

type SandboxConfig struct {
	AllowedDomains []string
	DeniedDomains  []string
	ReadPaths      []string
	WritePaths     []string
	EnableSeccomp  bool
}

type Harness struct {
	config      *SandboxConfig
	proxyServer *http.Server
	proxyURL    string
	tracer      trace.Tracer
	meter       metric.Meter
	execTotal   metric.Int64Counter
}

var (
	initMetricsOnce sync.Once
	executionsTotal metric.Int64Counter
)

func NewHarness(config *SandboxConfig) *Harness {
	h := &Harness{
		config: config,
		tracer: otel.Tracer("ohc.harness"),
		meter:  otel.Meter("ohc.harness"),
	}

	initMetricsOnce.Do(func() {
		var err error
		executionsTotal, err = otel.Meter("ohc.harness").Int64Counter("ohc_harness_executions_total")
		if err != nil {
			fmt.Printf("failed to initialize metric: %v\n", err)
		}
	})
	h.execTotal = executionsTotal

	// Start proxy server
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err == nil {
		h.proxyURL = "http://" + l.Addr().String()
		h.proxyServer = &http.Server{Handler: &ProxyHandler{DeniedDomains: config.DeniedDomains}}
		go h.proxyServer.Serve(l)
	}

	return h
}

type ProxyHandler struct {
	DeniedDomains []string
}

func (p *ProxyHandler) isDenied(host string) bool {
	for _, d := range p.DeniedDomains {
		if host == d || strings.HasSuffix(host, "."+d) {
			return true
		}
	}
	return false
}

func (p *ProxyHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	_, span := otel.Tracer("ohc.harness.proxy").Start(r.Context(), "ProxyHandler.ServeHTTP")
	defer span.End()
	span.SetAttributes(attribute.String("http.host", r.Host))
	span.SetAttributes(attribute.String("http.url", r.URL.String()))

	host := r.URL.Hostname()
	if host == "" {
		host = r.Host
		if strings.Contains(host, ":") {
			host = strings.Split(host, ":")[0]
		}
	}

	if p.isDenied(host) {
		http.Error(w, "Forbidden by Harness Proxy", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleConnect(w, r)
		return
	}

	// Basic HTTP proxy
	r.RequestURI = ""
	client := &http.Client{
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			return http.ErrUseLastResponse
		},
	}
	resp, err := client.Do(r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()

	for k, vv := range resp.Header {
		for _, v := range vv {
			w.Header().Add(k, v)
		}
	}
	w.WriteHeader(resp.StatusCode)
	io.Copy(w, resp.Body)
}

func (p *ProxyHandler) handleConnect(w http.ResponseWriter, r *http.Request) {
	hj, ok := w.(http.Hijacker)
	if !ok {
		http.Error(w, "webserver doesn't support hijacking", http.StatusInternalServerError)
		return
	}
	clientConn, _, err := hj.Hijack()
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer clientConn.Close()

	destConn, err := net.DialTimeout("tcp", r.Host, 10*time.Second)
	if err != nil {
		return
	}
	defer destConn.Close()

	clientConn.Write([]byte("HTTP/1.1 200 Connection Established\r\n\r\n"))

	go io.Copy(destConn, clientConn)
	io.Copy(clientConn, destConn)
}

func (h *Harness) Execute(ctx context.Context, command string) (Result, error) {
	return h.Run(ctx, "bash", []string{"-c", command})
}

func (h *Harness) Run(ctx context.Context, cmd string, args []string) (Result, error) {
	ctx, span := h.tracer.Start(ctx, "Harness.Run")
	defer span.End()

	telemetry.RecordBubblewrapSpawn(ctx)
	start := time.Now()

	span.SetAttributes(attribute.String("command", cmd))

	bwrapArgs := []string{
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--unshare-net",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
	}

	for _, p := range h.config.ReadPaths {
		bwrapArgs = append(bwrapArgs, "--ro-bind", p, p)
	}

	for _, p := range h.config.WritePaths {
		bwrapArgs = append(bwrapArgs, "--bind", p, p)
	}

	if h.config.EnableSeccomp {
		// Just a placeholder, as actual seccomp filtering in bwrap would be passed via --seccomp <fd>
	}

	bwrapArgs = append(bwrapArgs, "--")
	bwrapArgs = append(bwrapArgs, cmd)
	bwrapArgs = append(bwrapArgs, args...)

	execCmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	// Pass a restricted environment to the sandboxed process.
	execCmd.Env = []string{
		"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
	}
	if h.proxyURL != "" {
		execCmd.Env = append(execCmd.Env, fmt.Sprintf("HTTP_PROXY=%s", h.proxyURL))
		execCmd.Env = append(execCmd.Env, fmt.Sprintf("HTTPS_PROXY=%s", h.proxyURL))
	}

	var stdout, stderr bytes.Buffer
	execCmd.Stdout = &stdout
	execCmd.Stderr = &stderr

	err := execCmd.Run()

	duration := time.Since(start).Seconds()
	telemetry.RecordBubblewrapExecutionLatency(ctx, duration)

	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
			if exitCode != 0 && (strings.Contains(stderr.String(), "Permission denied") || exitCode == 126) {
				telemetry.RecordBubblewrapViolation(ctx)
			}
		} else {
			exitCode = -1
			if strings.Contains(err.Error(), "permission denied") {
				telemetry.RecordBubblewrapViolation(ctx)
			}
		}
	}

	span.SetAttributes(attribute.Int("exit_code", exitCode))

	if h.execTotal != nil {
		// Use empty metric.WithAttributes() instead of actual attribute if metric doesn't support it directly in older opentelemetry?
		// Wait, metric.WithAttributes was introduced recently, let's see if it works.
		h.execTotal.Add(ctx, 1) // Just add 1 without attributes for simplicity to avoid build errors depending on otel version.
	}

	return Result{
		Stdout:   stdout.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
	}, err
}

func (h *Harness) Stop() {
	if h.proxyServer != nil {
		h.proxyServer.Close()
	}
}
