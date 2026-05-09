package network

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/http/httputil"
	"os"
	"strings"
	"sync"
	"time"
)

type NetworkBridgeProxy struct {
	SocketPath      string
	AllowedDomains  []string
	listener        net.Listener
	server          *http.Server
	mu              sync.Mutex
	isReady         bool
}

func NewNetworkBridgeProxy(socketPath string, allowedDomains []string) *NetworkBridgeProxy {
	if socketPath == "" {
		socketPath = "/tmp/ohc-agent-http.sock"
	}
	return &NetworkBridgeProxy{
		SocketPath:     socketPath,
		AllowedDomains: allowedDomains,
	}
}

func (p *NetworkBridgeProxy) Start() error {
	p.mu.Lock()
	defer p.mu.Unlock()

	os.Remove(p.SocketPath)

	listener, err := net.Listen("unix", p.SocketPath)
	if err != nil {
		return fmt.Errorf("failed to listen on unix socket: %w", err)
	}
	p.listener = listener

	// Change permissions so other processes can access it if needed
	os.Chmod(p.SocketPath, 0777)

	proxy := &httputil.ReverseProxy{
		Director: func(req *http.Request) {
			// For forward proxies, we might need to handle scheme and host
			if req.URL.Scheme == "" {
				req.URL.Scheme = "http"
			}
			if req.URL.Host == "" && req.Host != "" {
				req.URL.Host = req.Host
			}
		},
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				host, _, err := net.SplitHostPort(addr)
				if err != nil {
					host = addr
				}

				if !p.isDomainAllowed(host) {
					return nil, fmt.Errorf("domain not allowed: %s", host)
				}

				dialer := &net.Dialer{
					Timeout:   30 * time.Second,
					KeepAlive: 30 * time.Second,
				}
				return dialer.DialContext(ctx, network, addr)
			},
		},
		ErrorHandler: func(w http.ResponseWriter, r *http.Request, err error) {
			if strings.Contains(err.Error(), "domain not allowed") {
				w.WriteHeader(http.StatusForbidden)
				io.WriteString(w, "Forbidden: "+err.Error())
				return
			}
			w.WriteHeader(http.StatusBadGateway)
			io.WriteString(w, "Bad Gateway: "+err.Error())
		},
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodConnect {
			p.handleHTTPS(w, r)
		} else {
			proxy.ServeHTTP(w, r)
		}
	})

	p.server = &http.Server{
		Handler: mux,
	}

	p.isReady = true

	go func() {
		_ = p.server.Serve(p.listener)
	}()

	return nil
}

func (p *NetworkBridgeProxy) handleHTTPS(w http.ResponseWriter, r *http.Request) {
	host, _, err := net.SplitHostPort(r.Host)
	if err != nil {
		host = r.Host
	}

	if !p.isDomainAllowed(host) {
		http.Error(w, "domain not allowed: "+host, http.StatusForbidden)
		return
	}

	destConn, err := net.DialTimeout("tcp", r.Host, 10*time.Second)
	if err != nil {
		http.Error(w, "dial destination error: "+err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer destConn.Close()

	hijacker, ok := w.(http.Hijacker)
	if !ok {
		http.Error(w, "hijacking not supported", http.StatusInternalServerError)
		return
	}

	clientConn, _, err := hijacker.Hijack()
	if err != nil {
		http.Error(w, "hijack error: "+err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer clientConn.Close()

	clientConn.Write([]byte("HTTP/1.1 200 Connection Established\r\n\r\n"))

	go transfer(destConn, clientConn)
	transfer(clientConn, destConn)
}

func transfer(destination io.WriteCloser, source io.ReadCloser) {
	defer destination.Close()
	defer source.Close()
	io.Copy(destination, source)
}

func (p *NetworkBridgeProxy) isDomainAllowed(domain string) bool {
	if len(p.AllowedDomains) == 0 {
		return true // Default to allow all if empty
	}

	// Strip port if present
	host, _, err := net.SplitHostPort(domain)
	if err != nil {
		host = domain
	}

	for _, allowed := range p.AllowedDomains {
		if host == allowed || strings.HasSuffix(host, "."+allowed) {
			return true
		}
	}
	return false
}

func (p *NetworkBridgeProxy) Stop() error {
	p.mu.Lock()
	defer p.mu.Unlock()

	p.isReady = false

	if p.server != nil {
		p.server.Close()
	}
	if p.listener != nil {
		p.listener.Close()
	}
	os.Remove(p.SocketPath)

	return nil
}

func (p *NetworkBridgeProxy) IsReady() bool {
	p.mu.Lock()
	defer p.mu.Unlock()
	return p.isReady
}
