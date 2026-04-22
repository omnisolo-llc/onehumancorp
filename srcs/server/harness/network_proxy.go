package harness

import (
	"context"
	"io"
	"net"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type NetworkBridgeProxy struct {
	server         *http.Server
	AllowedDomains []string
	AgentID        string
	listener       net.Listener
}

func NewNetworkBridgeProxy(agentID string, allowedDomains []string) *NetworkBridgeProxy {
	return &NetworkBridgeProxy{
		AllowedDomains: allowedDomains,
		AgentID:        agentID,
	}
}

func (p *NetworkBridgeProxy) Start() (string, error) {
	socketPath := "/tmp/ohc_proxy_" + p.AgentID + ".sock"
	_ = os.Remove(socketPath)

	l, err := net.Listen("unix", socketPath)
	if err != nil {
		return "", err
	}
	p.listener = l

	p.server = &http.Server{Handler: p}
	go p.server.Serve(l)

	return "unix://" + socketPath, nil
}

func (p *NetworkBridgeProxy) Stop() error {
	if p.server != nil {
		return p.server.Close()
	}
	return nil
}

func (p *NetworkBridgeProxy) isAllowed(host string) bool {
	if len(p.AllowedDomains) == 0 {
		return false
	}

	for _, d := range p.AllowedDomains {
		if host == d || strings.HasSuffix(host, "."+d) {
			return true
		}
	}
	return false
}

func (p *NetworkBridgeProxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Hostname()
	if host == "" {
		host = r.Host
		if strings.Contains(host, ":") {
			host = strings.Split(host, ":")[0]
		}
	}

	if !p.isAllowed(host) {
		telemetry.RecordSandboxViolation(r.Context(), "network_violation", p.AgentID, host)
		http.Error(w, "Forbidden", http.StatusForbidden)
		return
	}

	if r.Method == http.MethodConnect {
		p.handleConnect(w, r)
		return
	}

	p.handleHTTP(w, r)
}

func (p *NetworkBridgeProxy) handleHTTP(w http.ResponseWriter, r *http.Request) {
	r.RequestURI = ""

	req := r.Clone(context.Background())
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

func (p *NetworkBridgeProxy) handleConnect(w http.ResponseWriter, r *http.Request) {
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

	destConn, err := net.DialTimeout("tcp", r.Host, 10*time.Second)
	if err != nil {
		clientConn.Close()
		return
	}

	clientConn.Write([]byte("HTTP/1.1 200 Connection Established\r\n\r\n"))

	errChan := make(chan error, 2)

	type closeWriter interface {
		CloseWrite() error
	}

	go func() {
		_, err := io.Copy(destConn, clientConn)
		if cw, ok := destConn.(closeWriter); ok {
			cw.CloseWrite()
		}
		errChan <- err
	}()

	go func() {
		_, err := io.Copy(clientConn, destConn)
		if cw, ok := clientConn.(closeWriter); ok {
			cw.CloseWrite()
		}
		errChan <- err
	}()

	<-errChan
	<-errChan

	clientConn.Close()
	destConn.Close()
}
