package harness

import (
	"context"
	"fmt"
	"io"
	"log"
	"net"
	"net/http"
	"net/http/httputil"
	"sync"
	"time"
)

type NetworkProxy struct {
	AllowedDomains map[string]bool
	Telemetry      SandboxTelemetryEmitter
	Port           int
	server         *http.Server
	mu             sync.Mutex
}

func NewNetworkProxy(port int, allowedDomains []string, telemetry SandboxTelemetryEmitter) *NetworkProxy {
	proxy := &NetworkProxy{
		AllowedDomains: make(map[string]bool),
		Telemetry:      telemetry,
		Port:           port,
	}
	for _, domain := range allowedDomains {
		proxy.AllowedDomains[domain] = true
	}
	return proxy
}

func (p *NetworkProxy) Start() error {
	p.server = &http.Server{
		Addr:    fmt.Sprintf(":%d", p.Port),
		Handler: p,
	}

	listener, err := net.Listen("tcp", p.server.Addr)
	if err != nil {
		return fmt.Errorf("failed to listen on port %d: %w", p.Port, err)
	}

	go func() {
		if err := p.server.Serve(listener); err != nil && err != http.ErrServerClosed {
			log.Printf("Proxy server error: %v", err)
		}
	}()

	return nil
}

func (p *NetworkProxy) Stop(ctx context.Context) error {
	if p.server != nil {
		return p.server.Shutdown(ctx)
	}
	return nil
}

func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	// Strictly check req.Host (which defines where we're actually sending the traffic)
	host := req.Host
	if host == "" {
		host = req.URL.Hostname()
	}

	h, _, err := net.SplitHostPort(host)
	if err == nil {
		host = h
	}

	if !p.isDomainAllowed(host) {
		if p.Telemetry != nil {
			_ = p.Telemetry.RecordViolation(req.Context(), "network_violation", fmt.Sprintf("Blocked access to %s", host))
		}
		http.Error(w, "Forbidden: Domain not allowed", http.StatusForbidden)
		return
	}

	if req.Method == http.MethodConnect {
		p.handleHTTPS(w, req)
		return
	}

	proxy := &httputil.ReverseProxy{
		Director: func(r *http.Request) {
			r.URL.Scheme = "http"
			r.URL.Host = req.Host
			r.Host = req.Host // Ensure the Host header matches what was validated
		},
	}
	proxy.ServeHTTP(w, req)
}

func (p *NetworkProxy) handleHTTPS(w http.ResponseWriter, r *http.Request) {
	destConn, err := net.DialTimeout("tcp", r.Host, 10*time.Second)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

	w.WriteHeader(http.StatusOK)
	hijacker, ok := w.(http.Hijacker)
	if !ok {
		destConn.Close()
		http.Error(w, "Hijacking not supported", http.StatusInternalServerError)
		return
	}
	clientConn, _, err := hijacker.Hijack()
	if err != nil {
		destConn.Close()
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		io.Copy(destConn, clientConn)
		if tcpConn, ok := destConn.(*net.TCPConn); ok {
			tcpConn.CloseWrite()
		}
	}()

	go func() {
		defer wg.Done()
		io.Copy(clientConn, destConn)
		if tcpConn, ok := clientConn.(*net.TCPConn); ok {
			tcpConn.CloseWrite()
		}
	}()

	go func() {
		wg.Wait()
		destConn.Close()
		clientConn.Close()
	}()
}


func (p *NetworkProxy) isDomainAllowed(domain string) bool {
	if domain == "" {
		return false
	}
	p.mu.Lock()
	defer p.mu.Unlock()
	return p.AllowedDomains[domain]
}
