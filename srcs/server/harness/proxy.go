package harness

import (
	"net/http"
	"net/http/httputil"
	"net/url"
	"strings"
)

type Proxy struct {
	config *SandboxConfig
}

func NewProxy(config *SandboxConfig) *Proxy {
	return &Proxy{config: config}
}

func (p *Proxy) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Hostname()
	if host == "" {
		host = r.Host
	}

	// Remove port from host if present
	if strings.Contains(host, ":") {
		host = strings.Split(host, ":")[0]
	}

	// 1. Precise Deny Filter
	for _, denied := range p.config.DeniedDomains {
		if host == denied || strings.HasSuffix(host, "."+denied) {
			http.Error(w, "Forbidden", http.StatusForbidden)
			return
		}
	}

	// 2. Precise Allow Filter (if AllowedDomains is specified)
	if len(p.config.AllowedDomains) > 0 {
		allowed := false
		for _, allowedDomain := range p.config.AllowedDomains {
			if host == allowedDomain || strings.HasSuffix(host, "."+allowedDomain) {
				allowed = true
				break
			}
		}
		if !allowed {
			http.Error(w, "Forbidden", http.StatusForbidden)
			return
		}
	}

	// Forward allowed traffic properly
	targetURL := &url.URL{
		Scheme: "http",
		Host:   r.Host, // Include port if it was in the original request
	}
	if r.URL.Scheme != "" {
		targetURL.Scheme = r.URL.Scheme
	}

	proxy := httputil.NewSingleHostReverseProxy(targetURL)
	director := proxy.Director
	proxy.Director = func(req *http.Request) {
		director(req)
		req.URL.Scheme = targetURL.Scheme
		req.URL.Host = targetURL.Host

		// Remove the double path that SingleHostReverseProxy introduces
		req.URL.Path = r.URL.Path
	}

	proxy.ServeHTTP(w, r)
}

func (p *Proxy) Start() error {
	return http.ListenAndServe(":8080", p)
}
