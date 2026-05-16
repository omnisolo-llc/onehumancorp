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

// NetworkProxy is an HTTP proxy that filters network requests based on an allowed domains list.
type NetworkProxy struct {
	AllowedDomains []string
	Port           int
	server         *http.Server
	listener       net.Listener
}

// NewNetworkProxy creates a new NetworkProxy.
func NewNetworkProxy(allowedDomains []string, port int) *NetworkProxy {
	return &NetworkProxy{
		AllowedDomains: allowedDomains,
		Port:           port,
	}
}

// Start starts the proxy server in a background goroutine.
func (p *NetworkProxy) Start() error {
	p.server = &http.Server{
		Handler: p,
	}

	listener, err := net.Listen("tcp", fmt.Sprintf("127.0.0.1:%d", p.Port))
	if err != nil {
		return err
	}
	p.listener = listener
	p.Port = listener.Addr().(*net.TCPAddr).Port

	go func() {
		_ = p.server.Serve(listener)
	}()
	return nil
}

// Stop stops the proxy server.
func (p *NetworkProxy) Stop(ctx context.Context) error {
	if p.server != nil {
		return p.server.Shutdown(ctx)
	}
	return nil
}

func (p *NetworkProxy) isAllowed(host string) bool {
	domain := host
	if strings.Contains(host, ":") {
		domain = strings.Split(host, ":")[0]
	}

	for _, allowed := range p.AllowedDomains {
		if domain == allowed || strings.HasSuffix(domain, "."+allowed) {
			return true
		}
	}
	return false
}

// ServeHTTP handles proxy requests.
func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if !p.isAllowed(r.Host) {
		telemetry.RecordSandboxViolation(context.Background(), "network_denied")
		http.Error(w, "Access to domain denied by sandbox policy", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleHTTPS(w, r)
		return
	}

	p.handleHTTP(w, r)
}

func (p *NetworkProxy) handleHTTPS(w http.ResponseWriter, r *http.Request) {
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

	clientConn, clientReadWriter, err := hijacker.Hijack()
	if err != nil {
		destConn.Close()
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

	clientReadWriter.WriteString("HTTP/1.1 200 Connection Established\r\n\r\n")
	clientReadWriter.Flush()

	go func() {
		io.Copy(destConn, clientReadWriter)
		if tc, ok := destConn.(*net.TCPConn); ok {
			tc.CloseWrite()
		} else {
			destConn.Close()
		}
	}()
	go func() {
		io.Copy(clientConn, destConn)
		if tc, ok := clientConn.(*net.TCPConn); ok {
			tc.CloseWrite()
		} else {
			clientConn.Close()
		}
	}()
}

func (p *NetworkProxy) handleHTTP(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{}
	r.RequestURI = ""

	// Create a new request based on the incoming one to forward the body.
	proxyReq, err := http.NewRequest(r.Method, r.URL.String(), r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	proxyReq.Header = r.Header

	resp, err := client.Do(proxyReq)
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
