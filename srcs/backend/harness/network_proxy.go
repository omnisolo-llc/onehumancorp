package harness

import (
	"context"
	"io"
	"net"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type NetworkProxy struct {
	AllowedDomains []string
	Telemetry      orchestration.SandboxAdapter
	AgentID        string
}

func NewNetworkProxy(allowedDomains []string, telemetry orchestration.SandboxAdapter, agentID string) *NetworkProxy {
	return &NetworkProxy{
		AllowedDomains: allowedDomains,
		Telemetry:      telemetry,
		AgentID:        agentID,
	}
}

func (p *NetworkProxy) isAllowed(host string) bool {
	// Strip port if present
	if h, _, err := net.SplitHostPort(host); err == nil {
		host = h
	}

	for _, domain := range p.AllowedDomains {
		if strings.HasSuffix(host, domain) {
			return true
		}
	}
	return false
}

func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	host := r.Host
	if !p.isAllowed(host) {
		p.Telemetry.EmitViolation(r.Context(), "network_denied", p.AgentID, r.URL.String())
		http.Error(w, "Forbidden", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleTunneling(w, r)
	} else {
		p.handleHTTP(w, r)
	}
}

func (p *NetworkProxy) handleTunneling(w http.ResponseWriter, r *http.Request) {
	dest_conn, err := net.DialTimeout("tcp", r.Host, 10 * time.Second)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	w.WriteHeader(http.StatusOK)
	hijacker, ok := w.(http.Hijacker)
	if !ok {
		http.Error(w, "Hijacking not supported", http.StatusInternalServerError)
		return
	}
	client_conn, _, err := hijacker.Hijack()
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	go p.transfer(dest_conn, client_conn)
	go p.transfer(client_conn, dest_conn)
}

func (p *NetworkProxy) transfer(destination io.WriteCloser, source io.ReadCloser) {
	defer destination.Close()
	defer source.Close()
	io.Copy(destination, source)
}

func (p *NetworkProxy) handleHTTP(w http.ResponseWriter, r *http.Request) {
	r.RequestURI = ""
	resp, err := http.DefaultTransport.RoundTrip(r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
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

// StartProxy starts the HTTP proxy server on the given address.
func StartProxy(ctx context.Context, addr string, allowedDomains []string, telemetry orchestration.SandboxAdapter, agentID string) error {
	proxy := NewNetworkProxy(allowedDomains, telemetry, agentID)
	server := &http.Server{
		Addr:    addr,
		Handler: proxy,
	}

	go func() {
		<-ctx.Done()
		server.Shutdown(context.Background())
	}()

	return server.ListenAndServe()
}
