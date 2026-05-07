package harness

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"strings"
	"sync"

	"onehumancorp/srcs/server/telemetry"
)

// NetworkProxyHandler handles HTTP and CONNECT requests, enforcing the SandboxPolicy AllowedDomains.
type NetworkProxyHandler struct {
	policy SandboxPolicy
}

func isAllowed(domain string, allowedDomains []string) bool {
	host, _, err := net.SplitHostPort(domain)
	if err != nil {
		// If there is no port, SplitHostPort returns an error. We use the original domain.
		host = domain
	}

	for _, allowed := range allowedDomains {
		if host == allowed {
			return true
		}
		// Allow subdomains (e.g., .example.com matches api.example.com)
		if strings.HasPrefix(allowed, ".") && strings.HasSuffix(host, allowed) {
			return true
		}
	}
	return false
}

func (h *NetworkProxyHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if !isAllowed(r.Host, h.policy.AllowedDomains) {
		telemetry.RecordHarnessViolation(context.Background(), "network_proxy_denied")
		http.Error(w, "Forbidden by Sandbox Policy", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		h.handleTunneling(w, r)
	} else {
		h.handleHTTP(w, r)
	}
}

func (h *NetworkProxyHandler) handleTunneling(w http.ResponseWriter, r *http.Request) {
	dialer := &net.Dialer{}
	dest_conn, err := dialer.DialContext(r.Context(), "tcp", r.Host)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

	hijacker, ok := w.(http.Hijacker)
	if !ok {
		dest_conn.Close()
		http.Error(w, "Hijacking not supported", http.StatusInternalServerError)
		return
	}
	client_conn, _, err := hijacker.Hijack()
	if err != nil {
		dest_conn.Close()
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

	// Manually write the 200 Connection Established response directly to the hijacked connection.
	// w.WriteHeader should not be used before hijacking.
	_, err = client_conn.Write([]byte("HTTP/1.1 200 Connection Established\r\n\r\n"))
	if err != nil {
		dest_conn.Close()
		client_conn.Close()
		return
	}

	var wg sync.WaitGroup
	wg.Add(2)
	go h.transfer(&wg, dest_conn, client_conn)
	go h.transfer(&wg, client_conn, dest_conn)

	// Wait in a separate goroutine so we don't block the handler thread indefinitely
	go func() {
		wg.Wait()
		dest_conn.Close()
		client_conn.Close()
	}()
}

func (h *NetworkProxyHandler) handleHTTP(w http.ResponseWriter, r *http.Request) {
	// Strip proxy-specific headers
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

func (h *NetworkProxyHandler) transfer(wg *sync.WaitGroup, destination io.WriteCloser, source io.ReadCloser) {
	defer wg.Done()
	// Half-close implementation: copy stream, but don't close until WaitGroup completes or one side closes.
	// For TCP connections, we can cast to TCPConn to perform CloseWrite if needed, but io.Copy will return on EOF.
	io.Copy(destination, source)
	if tc, ok := destination.(*net.TCPConn); ok {
		tc.CloseWrite()
	}
}

// ProxyServer encapsulates a running net/http server
type ProxyServer struct {
	server     *http.Server
	listener   net.Listener
	SocketPath string
}

// Close gracefully shuts down the proxy server and removes the socket file
func (p *ProxyServer) Close() error {
	err := p.server.Close()
	os.Remove(p.SocketPath)
	return err
}

// StartProxy starts an HTTP proxy server on a unix socket and returns it.
func StartProxy(policy SandboxPolicy, socketPath string) (*ProxyServer, error) {
	handler := &NetworkProxyHandler{policy: policy}
	listener, err := net.Listen("unix", socketPath)
	if err != nil {
		return nil, fmt.Errorf("failed to bind proxy socket: %w", err)
	}

	server := &http.Server{Handler: handler}

	go func() {
		_ = server.Serve(listener)
	}()

	return &ProxyServer{
		server:     server,
		listener:   listener,
		SocketPath: socketPath,
	}, nil
}
