package harness

import (
	"fmt"
	"io"
	"net"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/src/server/telemetry"
)

// NetworkProxy acts as an HTTP proxy server that intercepts and validates
// network requests against an AllowedDomains list.
type NetworkProxy struct {
	agentID        string
	allowedDomains []string
	server         *http.Server
	listener       net.Listener
	url            string
}

// NewNetworkProxy creates a new NetworkProxy instance.
func NewNetworkProxy(agentID string, allowedDomains []string) *NetworkProxy {
	return &NetworkProxy{
		agentID:        agentID,
		allowedDomains: allowedDomains,
	}
}

// isAllowed checks if a host is within the allowed domains list.
func (p *NetworkProxy) isAllowed(host string) bool {
	for _, domain := range p.allowedDomains {
		if host == domain || strings.HasSuffix(host, "."+domain) {
			return true
		}
	}
	return false
}

// ServeHTTP handles the incoming HTTP requests and proxies them.
func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	host := r.URL.Hostname()
	if host == "" {
		host = r.Host
		if strings.Contains(host, ":") {
			host = strings.Split(host, ":")[0]
		}
	}

	if !p.isAllowed(host) {
		telemetry.RecordSandboxViolation(ctx, "network_violation", p.agentID, r.URL.String())
		http.Error(w, "Forbidden by Sandbox Proxy", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleConnect(w, r)
		return
	}

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

// Start binds the proxy server to a dynamic local port and starts serving.
func (p *NetworkProxy) Start() error {
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return fmt.Errorf("failed to bind proxy: %w", err)
	}
	p.listener = l
	p.url = "http://" + l.Addr().String()

	p.server = &http.Server{Handler: p}
	go p.server.Serve(l)

	return nil
}

// URL returns the dynamic URL of the local proxy server.
func (p *NetworkProxy) URL() string {
	return p.url
}

// Close stops the proxy server.
func (p *NetworkProxy) Close() error {
	if p.server != nil {
		return p.server.Close()
	}
	return nil
}
