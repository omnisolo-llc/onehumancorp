package mcp_webhook_tunnel

import (
	"context"
	"crypto/x509"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"
	"sync"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/peer"
	"google.golang.org/grpc/status"
)

var tracer = otel.Tracer("mcp_webhook_tunnel")

// activeStream wraps the gRPC stream with a mutex since SendMsg is not thread-safe.
type activeStream struct {
	mu     sync.Mutex
	stream WebhookTunnel_ConnectStreamServer
}

func (a *activeStream) Send(payload *WebhookPayloadMessage) error {
	a.mu.Lock()
	defer a.mu.Unlock()
	return a.stream.Send(payload)
}

// CloudRelay implements the WebhookTunnel gRPC service.
type CloudRelay struct {
	UnimplementedWebhookTunnelServer
	mu      sync.RWMutex
	streams map[string]*activeStream
}

// NewCloudRelay creates a new CloudRelay instance.
func NewCloudRelay() *CloudRelay {
	return &CloudRelay{
		streams: make(map[string]*activeStream),
	}
}

// ConnectStream handles incoming gRPC stream connections from Local Tunnel clients.
func (r *CloudRelay) ConnectStream(req *TunnelRequest, stream WebhookTunnel_ConnectStreamServer) error {
	agentID := req.GetAgentId()
	if agentID == "" {
		return status.Error(codes.InvalidArgument, "agent_id is required")
	}

	// Verify SPIFFE SVID
	p, ok := peer.FromContext(stream.Context())
	if !ok {
		return status.Error(codes.Unauthenticated, "no peer information found")
	}

	tlsAuth, ok := p.AuthInfo.(credentials.TLSInfo)
	if !ok {
		return status.Error(codes.Unauthenticated, "not using TLS")
	}

	if len(tlsAuth.State.PeerCertificates) == 0 {
		return status.Error(codes.Unauthenticated, "no peer certificates found")
	}

	cert := tlsAuth.State.PeerCertificates[0]
	spiffeID, err := extractSPIFFEFromCert(cert)
	if err != nil {
		return status.Errorf(codes.Unauthenticated, "SPIFFE: %v", err)
	}

	// Verify that the SPIFFE ID is valid
	if err := validateSPIFFEID(spiffeID); err != nil {
		return err
	}

	// Map the agent ID to the SPIFFE ID to prevent IDOR
	expectedSPIFFEID := fmt.Sprintf("spiffe://onehumancorp.com/agent/%s", agentID)
	if spiffeID != expectedSPIFFEID {
		return status.Errorf(codes.PermissionDenied, "SPIFFE ID mismatch: expected %s, got %s", expectedSPIFFEID, spiffeID)
	}

	active := &activeStream{stream: stream}

	r.mu.Lock()
	r.streams[agentID] = active
	r.mu.Unlock()

	// Keep the stream alive until the client disconnects or context is canceled
	<-stream.Context().Done()

	r.mu.Lock()
	if r.streams[agentID] == active {
		delete(r.streams, agentID)
	}
	r.mu.Unlock()

	return stream.Context().Err()
}

// HandleWebhook routes a received webhook payload to the appropriate agent stream.
func (r *CloudRelay) HandleWebhook(ctx context.Context, payload *WebhookPayloadMessage) error {
	ctx, span := tracer.Start(ctx, "HandleWebhook")
	defer span.End()

	agentID := payload.GetAgentId()
	span.SetAttributes(attribute.String("agent_id", agentID))

	r.mu.RLock()
	stream, exists := r.streams[agentID]
	r.mu.RUnlock()

	if !exists {
		err := errors.New("agent stream not found or disconnected")
		span.RecordError(err)
		return err
	}

	err := stream.Send(payload)
	if err != nil {
		span.RecordError(err)
	}
	return err
}

// ServeHTTP handles incoming REST webhooks.
func (r *CloudRelay) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	ctx := req.Context()
	ctx, span := tracer.Start(ctx, "ServeHTTP")
	defer span.End()

	if req.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Route: /api/v1/relay/webhook/{agent_id}
	pathParts := strings.Split(strings.Trim(req.URL.Path, "/"), "/")
	if len(pathParts) < 5 || pathParts[0] != "api" || pathParts[1] != "v1" || pathParts[2] != "relay" || pathParts[3] != "webhook" {
		http.Error(w, "Invalid route", http.StatusNotFound)
		return
	}
	agentID := pathParts[4]

	span.SetAttributes(attribute.String("agent_id", agentID))

	// Prevent Denial of Service (DoS) by limiting the payload size to 1MB and returning error if exceeded
	req.Body = http.MaxBytesReader(w, req.Body, 1024*1024)
	bodyBytes, err := io.ReadAll(req.Body)
	if err != nil {
		if errors.As(err, new(*http.MaxBytesError)) {
			http.Error(w, "Payload too large", http.StatusRequestEntityTooLarge)
			return
		}
		http.Error(w, "Failed to read body", http.StatusBadRequest)
		return
	}
	defer req.Body.Close()

	headers := make(map[string]string)
	for k, v := range req.Header {
		if len(v) > 0 {
			headers[k] = v[0]
		}
	}

	payload := &WebhookPayloadMessage{
		AgentId: agentID,
		Headers: headers,
		Body:    bodyBytes,
	}

	err = r.HandleWebhook(ctx, payload)
	if err != nil {
		http.Error(w, fmt.Sprintf("Failed to route webhook: %v", err), http.StatusServiceUnavailable)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

// extractSPIFFEFromCert returns the SPIFFE URI from the certificate's SAN field.
func extractSPIFFEFromCert(cert *x509.Certificate) (string, error) {
	for _, u := range cert.URIs {
		if u.Scheme == "spiffe" {
			return u.String(), nil
		}
	}
	return "", fmt.Errorf("no SPIFFE URI in peer certificate")
}

func validateSPIFFEID(id string) error {
	lower := strings.ToLower(id)
	if strings.Contains(lower, "%2f") || strings.Contains(lower, "%25") {
		return status.Errorf(codes.PermissionDenied, "invalid SPIFFE ID: encoded slashes: %s", id)
	}
	if !strings.HasPrefix(id, "spiffe://") {
		return status.Errorf(codes.PermissionDenied, "invalid SPIFFE ID: missing spiffe:// prefix")
	}
	return nil
}
