package harness

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"strings"
	"sync"
)

// TelemetryEmitter interface abstracting the OpenTelemetry metric emission.
type TelemetryEmitter interface {
	RecordSandboxViolation(ctx context.Context, violationType string, details string)
}

// NetworkProxy handles sub-agent outbound network requests
type NetworkProxy struct {
	allowedDomains []string
	telemetry      TelemetryEmitter
	server         *http.Server
	mu             sync.Mutex
}

// NewNetworkProxy creates a new NetworkProxy.
func NewNetworkProxy(allowedDomains []string, telemetry TelemetryEmitter) *NetworkProxy {
	return &NetworkProxy{
		allowedDomains: allowedDomains,
		telemetry:      telemetry,
	}
}

func (p *NetworkProxy) isAllowed(host string) bool {
	// Strip port if present
	if idx := strings.Index(host, ":"); idx != -1 {
		host = host[:idx]
	}
	for _, domain := range p.allowedDomains {
		if strings.EqualFold(host, domain) || strings.HasSuffix(strings.ToLower(host), "."+strings.ToLower(domain)) {
			return true
		}
	}
	return false
}

func (p *NetworkProxy) handleHTTP(w http.ResponseWriter, r *http.Request) {
	// Simple HTTP proxying
	client := &http.Client{
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			return http.ErrUseLastResponse
		},
	}
	r.RequestURI = "" // RequestURI must be empty for client.Do

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

func (p *NetworkProxy) handleHTTPS(w http.ResponseWriter, r *http.Request) {
	hijacker, ok := w.(http.Hijacker)
	if !ok {
		http.Error(w, "Hijacking not supported", http.StatusInternalServerError)
		return
	}

	clientConn, _, err := hijacker.Hijack()
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer clientConn.Close()

	// Connect to the destination server
	destConn, err := http.DefaultTransport.(*http.Transport).DialContext(r.Context(), "tcp", r.Host)
	if err != nil {
		clientConn.Write([]byte("HTTP/1.1 502 Bad Gateway\r\n\r\n"))
		return
	}
	defer destConn.Close()

	clientConn.Write([]byte("HTTP/1.1 200 Connection Established\r\n\r\n"))

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		io.Copy(destConn, clientConn)
	}()

	go func() {
		defer wg.Done()
		io.Copy(clientConn, destConn)
	}()

	wg.Wait()
}

func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	host := r.Host
	if !p.isAllowed(host) {
		details := fmt.Sprintf("Blocked access to %s", host)
		p.telemetry.RecordSandboxViolation(r.Context(), "network_proxy", details)
		http.Error(w, "Forbidden: Domain not in allowed list", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleHTTPS(w, r)
	} else {
		p.handleHTTP(w, r)
	}
}

// Start starts the proxy server on the given address (e.g., ":8080").
func (p *NetworkProxy) Start(addr string) error {
	p.mu.Lock()
	p.server = &http.Server{
		Addr:    addr,
		Handler: p,
	}
	p.mu.Unlock()
	return p.server.ListenAndServe()
}

// Stop stops the proxy server.
func (p *NetworkProxy) Stop(ctx context.Context) error {
	p.mu.Lock()
	defer p.mu.Unlock()
	if p.server != nil {
		return p.server.Shutdown(ctx)
	}
	return nil
}
