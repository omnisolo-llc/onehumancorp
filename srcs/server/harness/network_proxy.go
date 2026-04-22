package harness

import (
	"context"
	"io"
	"net"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)

// SandboxTelemetryEmitter interface for emitting telemetry events.
type SandboxTelemetryEmitter interface {
	EmitViolation(ctx context.Context, violationType string, agentID string, path string)
}

// DefaultSandboxTelemetryEmitter is the default implementation of SandboxTelemetryEmitter.
type DefaultSandboxTelemetryEmitter struct{}

// EmitViolation emits a sandbox violation metric.
func (d *DefaultSandboxTelemetryEmitter) EmitViolation(ctx context.Context, violationType string, agentID string, path string) {
	telemetry.RecordSandboxViolation(ctx, violationType, agentID, path)
}

// NetworkProxy is a local HTTP proxy server that intercepts network requests
// and gates them against an allowed domains list.
type NetworkProxy struct {
	AllowedDomains []string
	AgentID        string
	Emitter        SandboxTelemetryEmitter
	Server         *http.Server
	Address        string
}

// NewNetworkProxy creates a new NetworkProxy.
func NewNetworkProxy(allowedDomains []string, agentID string, emitter SandboxTelemetryEmitter) *NetworkProxy {
	if emitter == nil {
		emitter = &DefaultSandboxTelemetryEmitter{}
	}
	return &NetworkProxy{
		AllowedDomains: allowedDomains,
		AgentID:        agentID,
		Emitter:        emitter,
	}
}

// Start starts the HTTP proxy server on a random port.
func (p *NetworkProxy) Start() error {
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return err
	}
	p.Address = l.Addr().String()
	p.Server = &http.Server{Handler: p}
	go p.Server.Serve(l)
	return nil
}

// Stop stops the HTTP proxy server.
func (p *NetworkProxy) Stop() error {
	if p.Server != nil {
		return p.Server.Close()
	}
	return nil
}

// isAllowed checks if a host is in the allowed domains list.
func (p *NetworkProxy) isAllowed(host string) bool {
	if len(p.AllowedDomains) == 0 {
		return false // Default deny if empty? Or maybe default allow? Prompt: against an allowed domains list. So default deny.
	}
	for _, d := range p.AllowedDomains {
		if host == d || strings.HasSuffix(host, "."+d) {
			return true
		}
	}
	return false
}

func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	_, span := otel.Tracer("ohc.harness.proxy").Start(r.Context(), "NetworkProxy.ServeHTTP")
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

	if !p.isAllowed(host) {
		p.Emitter.EmitViolation(r.Context(), "network_denied", p.AgentID, r.URL.String())
		http.Error(w, "Forbidden by Harness Network Proxy", http.StatusForbidden)
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

func (p *NetworkProxy) handleConnect(w http.ResponseWriter, r *http.Request) {
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
