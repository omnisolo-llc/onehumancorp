package harness

import (
	"io"
	"net/http"
	"strings"
	"sync"
)

type SandboxTelemetryEmitter interface {
	EmitViolation(domain string)
}

type MockTelemetryEmitter struct {
	mu         sync.Mutex
	Violations []string
}

func (m *MockTelemetryEmitter) EmitViolation(domain string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.Violations = append(m.Violations, domain)
}

type NetworkProxy struct {
	AllowedDomains []string
	Telemetry      SandboxTelemetryEmitter
	server         *http.Server
}

func NewNetworkProxy(allowedDomains []string, telemetry SandboxTelemetryEmitter) *NetworkProxy {
	return &NetworkProxy{
		AllowedDomains: allowedDomains,
		Telemetry:      telemetry,
	}
}

func (p *NetworkProxy) isAllowed(domain string) bool {
	for _, d := range p.AllowedDomains {
		if strings.HasSuffix(domain, d) {
			return true
		}
	}
	return false
}

func (p *NetworkProxy) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	host := req.Host
	if !p.isAllowed(host) {
		p.Telemetry.EmitViolation(host)
		http.Error(w, "Forbidden by Sandbox Policy", http.StatusForbidden)
		return
	}

	if req.Method == http.MethodConnect {
		http.Error(w, "CONNECT not supported in this simple proxy", http.StatusMethodNotAllowed)
		return
	}

	req.RequestURI = ""

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		http.Error(w, "Proxy Error", http.StatusBadGateway)
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

func (p *NetworkProxy) Start(addr string) error {
	p.server = &http.Server{
		Addr:    addr,
		Handler: p,
	}
	return p.server.ListenAndServe()
}

func (p *NetworkProxy) Close() error {
	if p.server != nil {
		return p.server.Close()
	}
	return nil
}
