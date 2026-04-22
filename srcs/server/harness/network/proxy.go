package network

import (
	"context"
	"io"
	"net"
	"net/http"
	"strings"
	"sync"
	"time"
)

type NetworkBridgeProxy struct {
	AllowedDomains []string
	Port           int
	server         *http.Server
	mu             sync.Mutex
}

func NewNetworkBridgeProxy(allowedDomains []string) *NetworkBridgeProxy {
	return &NetworkBridgeProxy{
		AllowedDomains: allowedDomains,
	}
}

func (p *NetworkBridgeProxy) Start() error {
	p.mu.Lock()
	defer p.mu.Unlock()

	p.server = &http.Server{
		Handler: http.HandlerFunc(p.handleRequest),
	}

	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return err
	}
	p.Port = listener.Addr().(*net.TCPAddr).Port

	go p.server.Serve(listener)
	return nil
}

func (p *NetworkBridgeProxy) Stop(ctx context.Context) error {
	p.mu.Lock()
	defer p.mu.Unlock()
	if p.server != nil {
		return p.server.Shutdown(ctx)
	}
	return nil
}

func (p *NetworkBridgeProxy) handleRequest(w http.ResponseWriter, r *http.Request) {
	host := r.Host
	if strings.Contains(host, ":") {
		host, _, _ = net.SplitHostPort(host)
	}

	allowed := false
	for _, domain := range p.AllowedDomains {
		if host == domain {
			allowed = true
			break
		}
	}

	if !allowed {
		http.Error(w, "Forbidden", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleTunneling(w, r)
	} else {
		p.handleHTTP(w, r)
	}
}

func (p *NetworkBridgeProxy) handleTunneling(w http.ResponseWriter, r *http.Request) {
	dest_conn, err := net.DialTimeout("tcp", r.Host, 10*time.Second)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer dest_conn.Close()

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
	defer client_conn.Close()

	go p.transfer(dest_conn, client_conn)
	p.transfer(client_conn, dest_conn)
}

func (p *NetworkBridgeProxy) transfer(destination io.WriteCloser, source io.ReadCloser) {
	defer destination.Close()
	defer source.Close()
	io.Copy(destination, source)
}

func (p *NetworkBridgeProxy) handleHTTP(w http.ResponseWriter, req *http.Request) {
	req.RequestURI = ""
	client := &http.Client{
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			return http.ErrUseLastResponse
		},
	}

	resp, err := client.Do(req)
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
