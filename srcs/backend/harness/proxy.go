package harness

import (
	"context"
	"io"
	"net"
	"net/http"
	"strings"

	"go.opentelemetry.io/otel/attribute"
)

type ProxyServer struct {
	allowedDomains []string
	deniedDomains  []string
	server         *http.Server
	listener       net.Listener
}

func NewProxyServer(allowedDomains, deniedDomains []string) *ProxyServer {
	return &ProxyServer{
		allowedDomains: allowedDomains,
		deniedDomains:  deniedDomains,
	}
}

func (p *ProxyServer) Start() (string, error) {
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return "", err
	}
	p.listener = listener

	p.server = &http.Server{
		Handler: http.HandlerFunc(p.handleRequest),
	}

	go func() {
		_ = p.server.Serve(listener)
	}()

	return listener.Addr().String(), nil
}

func (p *ProxyServer) Stop() error {
	if p.server != nil {
		return p.server.Shutdown(context.Background())
	}
	return nil
}

func (p *ProxyServer) isDomainAllowed(host string) bool {
	// Remove port if present
	domain := host
	if strings.Contains(host, ":") {
		domain, _, _ = net.SplitHostPort(host)
	}

	for _, denied := range p.deniedDomains {
		if strings.EqualFold(domain, denied) || strings.HasSuffix(strings.ToLower(domain), "."+strings.ToLower(denied)) {
			return false
		}
	}

	// If allowed domains list is empty, treat as allow-all (except denied)
	if len(p.allowedDomains) == 0 {
		return true
	}

	for _, allowed := range p.allowedDomains {
		if strings.EqualFold(domain, allowed) || strings.HasSuffix(strings.ToLower(domain), "."+strings.ToLower(allowed)) {
			return true
		}
	}

	return false
}

func (p *ProxyServer) handleRequest(w http.ResponseWriter, r *http.Request) {
	ctx, span := tracer.Start(r.Context(), "proxy.Request")
	defer span.End()

	span.SetAttributes(
		attribute.String("http.method", r.Method),
		attribute.String("http.host", r.Host),
	)

	if !p.isDomainAllowed(r.Host) {
		span.SetAttributes(attribute.Bool("proxy.allowed", false))
		http.Error(w, "Forbidden", http.StatusForbidden)
		return
	}

	span.SetAttributes(attribute.Bool("proxy.allowed", true))

	if r.Method == http.MethodConnect {
		p.handleConnect(w, r)
		return
	}

	p.handleHTTP(w, r.WithContext(ctx))
}

func (p *ProxyServer) handleConnect(w http.ResponseWriter, r *http.Request) {
	destConn, err := net.Dial("tcp", r.Host)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

	hijacker, ok := w.(http.Hijacker)
	if !ok {
		http.Error(w, "Hijacking not supported", http.StatusInternalServerError)
		return
	}

	clientConn, _, err := hijacker.Hijack()
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}

	// Important: write the 200 OK directly to the hijacked connection
	_, err = clientConn.Write([]byte("HTTP/1.1 200 Connection established\r\n\r\n"))
	if err != nil {
		destConn.Close()
		clientConn.Close()
		return
	}

	go p.transfer(destConn, clientConn)
	go p.transfer(clientConn, destConn)
}

func (p *ProxyServer) transfer(destination io.WriteCloser, source io.ReadCloser) {
	defer destination.Close()
	defer source.Close()
	io.Copy(destination, source)
}

func (p *ProxyServer) handleHTTP(w http.ResponseWriter, r *http.Request) {
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
