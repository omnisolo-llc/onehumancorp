package proxy

import (
	"context"
	"io"
	"log"
	"net"
	"net/http"
	"strings"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter           = otel.Meter("onehumancorp/mono/srcs/server/agents/proxy")
	requestsCounter metric.Int64Counter
	latencyHisto    metric.Float64Histogram
)

func init() {
	var err error
	requestsCounter, err = meter.Int64Counter(
		"ohc_agent_outbound_requests_total",
		metric.WithDescription("Total number of outbound HTTP requests made by KAIROS agents"),
	)
	if err != nil {
		log.Printf("Failed to create requestsCounter: %v", err)
	}

	latencyHisto, err = meter.Float64Histogram(
		"ohc_agent_outbound_request_latency_seconds",
		metric.WithDescription("Latency of outbound HTTP requests made by KAIROS agents"),
	)
	if err != nil {
		log.Printf("Failed to create latencyHisto: %v", err)
	}
}

// ProxyCapture is an HTTP handler that proxies requests and records telemetry.
type ProxyCapture struct {
	Transport http.RoundTripper
}

// NewProxyCapture creates a new ProxyCapture.
func NewProxyCapture() *ProxyCapture {
	return &ProxyCapture{
		Transport: http.DefaultTransport,
	}
}

func copyHeaders(dst, src http.Header) {
	for k, vv := range src {
		for _, v := range vv {
			dst.Add(k, v)
		}
	}
}

func removeHopByHopHeaders(h http.Header) {
	hopHeaders := []string{
		"Connection",
		"Keep-Alive",
		"Proxy-Authenticate",
		"Proxy-Authorization",
		"Te",
		"Trailer",
		"Transfer-Encoding",
		"Upgrade",
	}

	if c := h.Get("Connection"); c != "" {
		for _, f := range strings.Split(c, ",") {
			if f = strings.TrimSpace(f); f != "" {
				h.Del(f)
			}
		}
	}

	for _, hh := range hopHeaders {
		h.Del(hh)
	}
}

func (p *ProxyCapture) handleConnect(w http.ResponseWriter, r *http.Request) {
	start := time.Now()

	// Hijack the connection
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
	defer clientConn.Close()

	// Connect to the target
	targetConn, err := net.DialTimeout("tcp", r.Host, 10*time.Second)
	if err != nil {
		http.Error(w, err.Error(), http.StatusServiceUnavailable)
		return
	}
	defer targetConn.Close()

	// Send 200 OK to the client
	clientConn.Write([]byte("HTTP/1.1 200 Connection Established\r\n\r\n"))

	// Record telemetry (for CONNECT, duration is connection lifetime, we can record it early or after)
	// We'll record it after the tunnel closes
	defer func() {
		duration := time.Since(start).Seconds()
		ctx := context.Background()
		if requestsCounter != nil {
			requestsCounter.Add(ctx, 1)
		}
		if latencyHisto != nil {
			latencyHisto.Record(ctx, duration)
		}
	}()

	// Tunnel data
	errc := make(chan error, 2)
	go func() {
		_, err := io.Copy(targetConn, clientConn)
		errc <- err
	}()
	go func() {
		_, err := io.Copy(clientConn, targetConn)
		errc <- err
	}()

	<-errc
}

// ServeHTTP implements the http.Handler interface.
func (p *ProxyCapture) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodConnect {
		p.handleConnect(w, r)
		return
	}

	start := time.Now()

	outReq := new(http.Request)
	*outReq = *r

	if outReq.URL.Scheme == "" {
		if outReq.TLS != nil {
			outReq.URL.Scheme = "https"
		} else {
			outReq.URL.Scheme = "http"
		}
	}
	if outReq.URL.Host == "" {
		outReq.URL.Host = r.Host
	}
	outReq.RequestURI = ""

	removeHopByHopHeaders(outReq.Header)

	res, err := p.Transport.RoundTrip(outReq)

	duration := time.Since(start).Seconds()

	if err != nil {
		log.Printf("Proxy error: %v", err)
		http.Error(w, err.Error(), http.StatusBadGateway)
	} else {
		defer res.Body.Close()

		removeHopByHopHeaders(res.Header)
		copyHeaders(w.Header(), res.Header)
		w.WriteHeader(res.StatusCode)
		io.Copy(w, res.Body)
	}

	ctx := context.Background()
	if requestsCounter != nil {
		requestsCounter.Add(ctx, 1)
	}
	if latencyHisto != nil {
		latencyHisto.Record(ctx, duration)
	}
}
