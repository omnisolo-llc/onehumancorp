package sandbox

import (
	"encoding/binary"
	"fmt"
	"io"
	"net"
	"net/http"
	"strings"
	"sync"
)

type ProxyServer struct {
	addr           string
	allowedDomains []string
	server         *http.Server
	mu             sync.Mutex
	socksListener  net.Listener
}

func NewProxyServer(allowedDomains []string) *ProxyServer {
	return &ProxyServer{
		allowedDomains: allowedDomains,
	}
}

func (p *ProxyServer) isDomainAllowed(host string) bool {
	p.mu.Lock()
	defer p.mu.Unlock()

	if len(p.allowedDomains) == 0 {
		return false // Secure default: if domains are specified but empty, deny all. Wait, if none specified, deny.
	}

	domain := host
	if strings.Contains(host, ":") {
		domain, _, _ = strings.Cut(host, ":")
	}

	for _, allowed := range p.allowedDomains {
		if domain == allowed || strings.HasSuffix(domain, "."+allowed) {
			return true
		}
	}
	return false
}

func (p *ProxyServer) handleHTTP(w http.ResponseWriter, r *http.Request) {
	if !p.isDomainAllowed(r.Host) {
		http.Error(w, "Forbidden Domain", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleHTTPS(w, r)
		return
	}

	resp, err := http.DefaultTransport.RoundTrip(r)
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

func (p *ProxyServer) handleHTTPS(w http.ResponseWriter, r *http.Request) {
	destConn, err := net.Dial("tcp", r.Host)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	w.WriteHeader(http.StatusOK)
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
	go io.Copy(destConn, clientConn)
	go io.Copy(clientConn, destConn)
}

func (p *ProxyServer) Start() (string, error) {
	// Start HTTP Proxy
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return "", err
	}
	p.addr = listener.Addr().String()

	p.server = &http.Server{
		Handler: http.HandlerFunc(p.handleHTTP),
	}

	go p.server.Serve(listener)

	// Start SOCKS5 Proxy
	socksListener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		p.Stop()
		return "", err
	}
	p.socksListener = socksListener
	go p.serveSOCKS5()

	// Return the HTTP proxy address for backward compatibility in this interface,
	// or we could return both. For now, returning the HTTP proxy address is sufficient.
	return p.addr, nil
}

func (p *ProxyServer) serveSOCKS5() {
	for {
		conn, err := p.socksListener.Accept()
		if err != nil {
			return
		}
		go p.handleSOCKS5(conn)
	}
}

func (p *ProxyServer) handleSOCKS5(conn net.Conn) {
	defer conn.Close()

	// Version identifier/method selection message
	buf := make([]byte, 256)
	if _, err := io.ReadFull(conn, buf[:2]); err != nil {
		return
	}
	if buf[0] != 0x05 { // SOCKS5
		return
	}
	numMethods := int(buf[1])
	if _, err := io.ReadFull(conn, buf[:numMethods]); err != nil {
		return
	}

	// Send method selection response (No authentication required)
	conn.Write([]byte{0x05, 0x00})

	// Read SOCKS5 request
	if _, err := io.ReadFull(conn, buf[:4]); err != nil {
		return
	}
	if buf[0] != 0x05 || buf[1] != 0x01 || buf[2] != 0x00 { // Only support CONNECT
		return
	}

	addrType := buf[3]
	var destAddr string

	switch addrType {
	case 0x01: // IPv4
		if _, err := io.ReadFull(conn, buf[:4]); err != nil {
			return
		}
		destAddr = net.IP(buf[:4]).String()
	case 0x03: // Domain name
		if _, err := io.ReadFull(conn, buf[:1]); err != nil {
			return
		}
		domainLen := int(buf[0])
		if _, err := io.ReadFull(conn, buf[:domainLen]); err != nil {
			return
		}
		destAddr = string(buf[:domainLen])
	case 0x04: // IPv6
		if _, err := io.ReadFull(conn, buf[:16]); err != nil {
			return
		}
		destAddr = net.IP(buf[:16]).String()
	default:
		return
	}

	// Read port
	if _, err := io.ReadFull(conn, buf[:2]); err != nil {
		return
	}
	port := binary.BigEndian.Uint16(buf[:2])
	destHostPort := fmt.Sprintf("%s:%d", destAddr, port)

	if !p.isDomainAllowed(destAddr) {
		// Connection not allowed by ruleset
		conn.Write([]byte{0x05, 0x02, 0x00, 0x01, 0, 0, 0, 0, 0, 0})
		return
	}

	destConn, err := net.Dial("tcp", destHostPort)
	if err != nil {
		// Host unreachable
		conn.Write([]byte{0x05, 0x04, 0x00, 0x01, 0, 0, 0, 0, 0, 0})
		return
	}
	defer destConn.Close()

	// Reply Success
	conn.Write([]byte{0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0})

	go io.Copy(destConn, conn)
	io.Copy(conn, destConn)
}

func (p *ProxyServer) Stop() error {
	var err error
	if p.server != nil {
		err = p.server.Close()
	}
	if p.socksListener != nil {
		p.socksListener.Close()
	}
	return err
}
