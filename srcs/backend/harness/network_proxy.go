package harness

import (
	"context"
	"fmt"
	"io"
	"log"
	"net"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type TelemetryEmitter interface {
	RecordSandboxViolation(ctx context.Context, violationType, agentID, path string)
}

type DefaultTelemetryEmitter struct{}

func (e *DefaultTelemetryEmitter) RecordSandboxViolation(ctx context.Context, violationType, agentID, path string) {
	telemetry.RecordSandboxViolation(ctx, violationType, agentID, path)
}

type NetworkProxy struct {
	AllowedDomains []string
	Port           int
	AgentID        string
	Emitter        TelemetryEmitter
	listener       net.Listener
	httpClient     *http.Client
}

func NewNetworkProxy(port int, allowedDomains []string, agentID string) *NetworkProxy {
	// Create a client that does NOT follow redirects
	noRedirectClient := &http.Client{
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			return http.ErrUseLastResponse
		},
		Timeout: 30 * time.Second,
	}

	return &NetworkProxy{
		AllowedDomains: allowedDomains,
		Port:           port,
		AgentID:        agentID,
		Emitter:        &DefaultTelemetryEmitter{},
		httpClient:     noRedirectClient,
	}
}

func (p *NetworkProxy) isAllowed(domain string) bool {
	for _, allowed := range p.AllowedDomains {
		if domain == allowed || strings.HasSuffix(domain, "."+allowed) {
			return true
		}
	}
	return false
}

func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Host
	if host == "" {
		host = r.Host
	}

	hostWithoutPort := host
	if idx := strings.IndexByte(host, ':'); idx != -1 {
		hostWithoutPort = host[:idx]
	}

	if !p.isAllowed(hostWithoutPort) {
		p.Emitter.RecordSandboxViolation(r.Context(), "network", p.AgentID, host)
		http.Error(w, "Access denied by sandbox", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		destConn, err := net.Dial("tcp", host)
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
		clientConn, clientBuf, err := hijacker.Hijack()
		if err != nil {
			destConn.Close()
			http.Error(w, err.Error(), http.StatusServiceUnavailable)
			return
		}

		clientBuf.WriteString("HTTP/1.1 200 Connection Established\r\n\r\n")
		clientBuf.Flush()

		go transfer(destConn, clientConn)
		go transfer(clientConn, destConn)
	} else {
		proxyReq, err := http.NewRequest(r.Method, r.URL.String(), r.Body)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		proxyReq.Header = r.Header

		resp, err := p.httpClient.Do(proxyReq)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadGateway)
			return
		}
		defer resp.Body.Close()

		for k, v := range resp.Header {
			for _, val := range v {
				w.Header().Add(k, val)
			}
		}
		w.WriteHeader(resp.StatusCode)
		io.Copy(w, resp.Body)
	}
}

// Start listens on the configured port. If Port is 0, it asks the OS for an open port.
// It sets p.Port to the actual bound port.
func (p *NetworkProxy) Start(ctx context.Context) (*http.Server, error) {
	addr := fmt.Sprintf("127.0.0.1:%d", p.Port)
	listener, err := net.Listen("tcp", addr)
	if err != nil {
		return nil, err
	}
	p.listener = listener
	p.Port = listener.Addr().(*net.TCPAddr).Port

	server := &http.Server{
		Handler: p,
	}

	go func() {
		if err := server.Serve(listener); err != nil && err != http.ErrServerClosed {
			log.Printf("Proxy server error: %v", err)
		}
	}()

	return server, nil
}

func transfer(destination io.WriteCloser, source io.ReadCloser) {
	defer destination.Close()
	defer source.Close()
	io.Copy(destination, source)
}
