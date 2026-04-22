package network

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"time"
	"os"
	"os/exec"
	"strings"
	"sync"
)

type closeWriter interface {
	CloseWrite() error
}

type NetworkBridgeProxy struct {
	SocketPath     string
	AllowedDomains []string
	socatCmd       *exec.Cmd
	proxyServer    *http.Server
	listener       net.Listener
	wg             sync.WaitGroup
}

func NewNetworkBridgeProxy(socketPath string, allowedDomains []string) *NetworkBridgeProxy {
	return &NetworkBridgeProxy{
		SocketPath:     socketPath,
		AllowedDomains: allowedDomains,
	}
}

func (p *NetworkBridgeProxy) isDomainAllowed(host string) bool {
	// Strip port if present
	hostname := host
	if h, _, err := net.SplitHostPort(host); err == nil {
		hostname = h
	}

	for _, domain := range p.AllowedDomains {
		if hostname == domain || strings.HasSuffix(hostname, "."+domain) {
			return true
		}
	}
	return false
}

func (p *NetworkBridgeProxy) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	if req.Method == http.MethodConnect {
		p.handleConnect(w, req)
	} else {
		p.handleHTTP(w, req)
	}
}

func (p *NetworkBridgeProxy) handleHTTP(w http.ResponseWriter, req *http.Request) {
	if !p.isDomainAllowed(req.Host) {
		http.Error(w, "Forbidden", http.StatusForbidden)
		return
	}

	transport := http.DefaultTransport

	// req.RequestURI can't be set for client requests
	outReq := req.Clone(req.Context())
	outReq.RequestURI = ""

	if outReq.URL.Scheme == "" {
		outReq.URL.Scheme = "http"
	}
	if outReq.URL.Host == "" {
		outReq.URL.Host = req.Host
	}

	res, err := transport.RoundTrip(outReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer res.Body.Close()

	for k, vv := range res.Header {
		for _, v := range vv {
			w.Header().Add(k, v)
		}
	}
	w.WriteHeader(res.StatusCode)
	io.Copy(w, res.Body)
}

func (p *NetworkBridgeProxy) handleConnect(w http.ResponseWriter, req *http.Request) {
	if !p.isDomainAllowed(req.Host) {
		http.Error(w, "Forbidden", http.StatusForbidden)
		return
	}

	destConn, err := net.Dial("tcp", req.Host)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer destConn.Close()

	hijacker, ok := w.(http.Hijacker)
	if !ok {
		http.Error(w, "Hijacking not supported", http.StatusInternalServerError)
		return
	}
	clientConn, bufrw, err := hijacker.Hijack()
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer clientConn.Close()

	bufrw.WriteString("HTTP/1.1 200 Connection Established\r\n\r\n")
	bufrw.Flush()

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		io.Copy(destConn, clientConn)
		if cw, ok := destConn.(closeWriter); ok {
			cw.CloseWrite()
		}
	}()

	go func() {
		defer wg.Done()
		io.Copy(clientConn, destConn)
		if cw, ok := clientConn.(closeWriter); ok {
			cw.CloseWrite()
		}
	}()

	wg.Wait()
}

func (p *NetworkBridgeProxy) Start() error {
	// 1. Start Go HTTP Proxy on random port
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return fmt.Errorf("failed to bind to random port: %w", err)
	}
	p.listener = l

	p.proxyServer = &http.Server{Handler: p}

	p.wg.Add(1)
	go func() {
		defer p.wg.Done()
		p.proxyServer.Serve(p.listener)
	}()

	// 2. Start socat to bridge Unix socket to HTTP Proxy
	// Ensure the socket directory exists and remove any stale socket
	os.Remove(p.SocketPath)

	proxyAddr := l.Addr().(*net.TCPAddr)
	socatArgs := []string{
		fmt.Sprintf("UNIX-LISTEN:%s,fork", p.SocketPath),
		fmt.Sprintf("TCP:127.0.0.1:%d", proxyAddr.Port),
	}

	p.socatCmd = exec.Command("socat", socatArgs...)
	if err := p.socatCmd.Start(); err != nil {
		p.Stop()
		return fmt.Errorf("failed to start socat: %w", err)
	}

	// Wait briefly for socat to create the socket
	for i := 0; i < 50; i++ {
		if _, err := os.Stat(p.SocketPath); err == nil {
			return nil
		}
		time.Sleep(10 * time.Millisecond)
	}

	return fmt.Errorf("socat failed to create socket %s in time", p.SocketPath)
}

func (p *NetworkBridgeProxy) Stop() error {
	var errs []string

	if p.socatCmd != nil && p.socatCmd.Process != nil {
		if err := p.socatCmd.Process.Kill(); err != nil {
			errs = append(errs, fmt.Sprintf("socat kill err: %v", err))
		}
		p.socatCmd.Wait() // wait to prevent zombie process
	}

	if p.proxyServer != nil {
		if err := p.proxyServer.Shutdown(context.Background()); err != nil {
			errs = append(errs, fmt.Sprintf("proxy shutdown err: %v", err))
		}
	}

	if p.listener != nil {
		p.listener.Close()
	}

	p.wg.Wait()

	if p.SocketPath != "" {
		os.Remove(p.SocketPath)
	}

	if len(errs) > 0 {
		return fmt.Errorf("errors stopping proxy: %s", strings.Join(errs, ", "))
	}
	return nil
}
