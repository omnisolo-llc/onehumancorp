package harness

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"strings"

	"onehumancorp/srcs/server/telemetry"
)

// NetworkProxy represents the local HTTP MITM proxy.
type NetworkProxy struct {
	server *http.Server
	policy *SandboxPolicy
}

// NewNetworkProxy initializes a new proxy with the given policy.
func NewNetworkProxy(policy *SandboxPolicy) *NetworkProxy {
	return &NetworkProxy{
		policy: policy,
	}
}

// Start starts the proxy on the specified address.
func (p *NetworkProxy) Start(addr string) error {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		p.handleRequest(w, r)
	})

	p.server = &http.Server{
		Addr:    addr,
		Handler: handler,
	}

	return p.server.ListenAndServe()
}

// Stop shuts down the proxy server.
func (p *NetworkProxy) Stop(ctx context.Context) error {
	if p.server != nil {
		return p.server.Shutdown(ctx)
	}
	return nil
}

func (p *NetworkProxy) isAllowed(domain string) bool {
	// Strip port if present
	host := domain
	if h, _, err := net.SplitHostPort(domain); err == nil {
		host = h
	}

	for _, blocked := range p.policy.BlockedDomains {
		if strings.EqualFold(host, blocked) {
			return false
		}
	}

	if len(p.policy.AllowedDomains) == 0 {
		return true // Allow all if not explicitly blocked and allowlist is empty
	}

	for _, allowed := range p.policy.AllowedDomains {
		if strings.EqualFold(host, allowed) {
			return true
		}
	}
	return false
}

func (p *NetworkProxy) handleRequest(w http.ResponseWriter, r *http.Request) {
	if !p.isAllowed(r.Host) {
		telemetry.RecordHarnessViolation(context.Background(), "proxy_denied")
		http.Error(w, fmt.Sprintf("Domain %s is denied by sandbox policy", r.Host), http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleTunneling(w, r)
	} else {
		p.handleHTTP(w, r)
	}
}

func (p *NetworkProxy) handleTunneling(w http.ResponseWriter, r *http.Request) {
	destConn, err := net.Dial("tcp", r.Host)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

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

	clientConn.Write([]byte("HTTP/1.1 200 Connection established\r\n\r\n"))

	go p.transfer(destConn, clientConn)
	go p.transfer(clientConn, destConn)
}

func (p *NetworkProxy) transfer(destination io.WriteCloser, source io.ReadCloser) {
	defer func() {
		// Only close half of the connection if supported, or rely on GC. We'll simply let io.Copy finish.
		if cw, ok := destination.(interface{ CloseWrite() error }); ok {
			cw.CloseWrite()
		} else {
			destination.Close()
		}
		if cr, ok := source.(interface{ CloseRead() error }); ok {
			cr.CloseRead()
		} else {
			source.Close()
		}
	}()
	io.Copy(destination, source)
}

func (p *NetworkProxy) handleHTTP(w http.ResponseWriter, req *http.Request) {
	// RoundTrip explicitly forbids RequestURI
	req.RequestURI = ""

	resp, err := http.DefaultTransport.RoundTrip(req)
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
