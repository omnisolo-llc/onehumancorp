package harness

import (
	"context"
	"io"
	"net"
	"net/http"
	"strings"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"
)

// ProxyServer is an HTTP proxy server that filters requests based on denied domains.
type ProxyServer struct {
	server        *http.Server
	deniedDomains []string
}

// NewProxyServer creates a new proxy server.
func NewProxyServer(deniedDomains []string) *ProxyServer {
	return &ProxyServer{
		deniedDomains: deniedDomains,
	}
}

// Start starts the proxy server on the specified address.
func (p *ProxyServer) Start(addr string, started chan struct{}) error {
	listener, err := net.Listen("tcp", addr)
	if err != nil {
		return err
	}

	p.server = &http.Server{
		Addr:    listener.Addr().String(),
		Handler: p,
	}

	if started != nil {
		close(started)
	}

	err = p.server.Serve(listener)
	if err == http.ErrServerClosed {
		return nil
	}
	return err
}

// GetAddr returns the actual address the server is listening on.
func (p *ProxyServer) GetAddr() string {
	if p.server != nil {
		return p.server.Addr
	}
	return ""
}

// Stop stops the proxy server.
func (p *ProxyServer) Stop(ctx context.Context) error {
	if p.server != nil {
		return p.server.Shutdown(ctx)
	}
	return nil
}

func (p *ProxyServer) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	ctx, span := tracer.Start(req.Context(), "ProxyServer.ServeHTTP", trace.WithAttributes(
		attribute.String("http.method", req.Method),
		attribute.String("http.url", req.URL.String()),
		attribute.String("http.host", req.Host),
	))
	defer span.End()

	req = req.WithContext(ctx)

	host := req.Host
	if strings.Contains(host, ":") {
		host, _, _ = net.SplitHostPort(req.Host)
	}

	for _, domain := range p.deniedDomains {
		if strings.EqualFold(host, domain) || strings.HasSuffix(strings.ToLower(host), "."+strings.ToLower(domain)) {
			http.Error(w, "Forbidden domain", http.StatusForbidden)
			return
		}
	}

	if req.Method == http.MethodConnect {
		p.handleHTTPS(w, req)
	} else {
		p.handleHTTP(w, req)
	}
}

func (p *ProxyServer) handleHTTP(w http.ResponseWriter, req *http.Request) {
	req = req.Clone(req.Context())
	// RoundTrip explicitly rejects client requests with a non-empty RequestURI.
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

func (p *ProxyServer) handleHTTPS(w http.ResponseWriter, req *http.Request) {
	destConn, err := net.Dial("tcp", req.Host)
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

	clientConn.Write([]byte("HTTP/1.1 200 OK\r\n\r\n"))

	go p.transfer(destConn, clientConn)
	go p.transfer(clientConn, destConn)
}

func (p *ProxyServer) transfer(destination io.WriteCloser, source io.ReadCloser) {
	defer destination.Close()
	defer source.Close()
	io.Copy(destination, source)
}
