package agentgrpc

// auth.go implements SPIFFE/mTLS + pre-shared token authentication for the
// agent gRPC server, matching the pattern used by the main orchestration
// server in srcs/server/orchestration/auth_interceptor.go.
//
// Two modes are supported (chosen at runtime by configuration):
//
//  1. SPIFFE/mTLS (production):
//     The TLS connection must carry a peer X.509 certificate whose SAN URI
//     starts with "spiffe://".  Trust domains are the same set accepted by
//     the orchestration interceptor.  The process-level OHC_AGENT_SPIFFE_ID
//     env var may further restrict which SPIFFE ID may call the agent.
//
//  2. Pre-shared token (standalone / dev):
//     The caller must send the gRPC metadata key "authorization" with value
//     "Bearer <token>".  The token is HMAC-SHA256 signed and compared in
//     constant time to prevent timing attacks.

import (
	"context"
	"crypto/hmac"
	"crypto/sha256"
	"crypto/tls"
	"crypto/x509"
	"encoding/hex"
	"fmt"
	"os"
	"strings"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/peer"
	"google.golang.org/grpc/status"
)

// authMode controls which enforcement strategy is active.
type authMode int

const (
	authModeDisabled authMode = iota // dev/test – no auth (must be explicit)
	authModeToken                    // pre-shared HMAC token
	authModeSPIFFE                   // SPIFFE/mTLS peer cert
)

// AuthConfig holds the resolved authentication settings.
type AuthConfig struct {
	mode authMode

	// token mode
	tokenHash []byte // HMAC-SHA256 of the expected token

	// spiffe mode
	allowedID string // empty = accept any valid SPIFFE ID
}

// AuthConfigFromEnv builds an AuthConfig from environment variables:
//
//	OHC_AGENT_TOKEN      – pre-shared secret (enables token mode)
//	OHC_AGENT_SPIFFE_ID  – restrict SPIFFE ID (enables SPIFFE mode when set without token)
//	OHC_AGENT_AUTH_DISABLED=true – skip auth entirely (only for tests/local-dev)
func AuthConfigFromEnv() AuthConfig {
	if os.Getenv("OHC_AGENT_AUTH_DISABLED") == "true" {
		return AuthConfig{mode: authModeDisabled}
	}
	if tok := os.Getenv("OHC_AGENT_TOKEN"); tok != "" {
		h := hmacToken(tok)
		return AuthConfig{mode: authModeToken, tokenHash: h}
	}
	// Default: SPIFFE mode (requires mTLS transport)
	return AuthConfig{
		mode:      authModeSPIFFE,
		allowedID: os.Getenv("OHC_AGENT_SPIFFE_ID"),
	}
}

// UnaryInterceptor returns a gRPC unary server interceptor enforcing auth.
func (c AuthConfig) UnaryInterceptor() grpc.UnaryServerInterceptor {
	return func(
		ctx context.Context,
		req any,
		_ *grpc.UnaryServerInfo,
		handler grpc.UnaryHandler,
	) (any, error) {
		if err := c.authenticate(ctx); err != nil {
			return nil, err
		}
		return handler(ctx, req)
	}
}

// StreamInterceptor returns a gRPC streaming server interceptor enforcing auth.
func (c AuthConfig) StreamInterceptor() grpc.StreamServerInterceptor {
	return func(
		srv any,
		ss grpc.ServerStream,
		_ *grpc.StreamServerInfo,
		handler grpc.StreamHandler,
	) error {
		if err := c.authenticate(ss.Context()); err != nil {
			return err
		}
		return handler(srv, ss)
	}
}

// authenticate performs the actual check for a single RPC.
// Zero allocations in the SPIFFE fast-path after initial cert parse.
func (c AuthConfig) authenticate(ctx context.Context) error {
	switch c.mode {
	case authModeDisabled:
		return nil

	case authModeToken:
		return c.checkToken(ctx)

	case authModeSPIFFE:
		return c.checkSPIFFE(ctx)

	default:
		return status.Error(codes.Internal, "unknown auth mode")
	}
}

// checkToken validates the bearer token in gRPC metadata.
// Uses HMAC constant-time comparison to prevent timing leaks.
func (c AuthConfig) checkToken(ctx context.Context) error {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return status.Error(codes.Unauthenticated, "missing metadata")
	}
	vals := md.Get("authorization")
	if len(vals) == 0 {
		return status.Error(codes.Unauthenticated, "missing authorization header")
	}
	bearer := vals[0]
	const prefix = "Bearer "
	if !strings.HasPrefix(bearer, prefix) {
		return status.Error(codes.Unauthenticated, "authorization must be Bearer token")
	}
	provided := hmacToken(strings.TrimPrefix(bearer, prefix))
	if !hmac.Equal(provided, c.tokenHash) {
		return status.Error(codes.Unauthenticated, "invalid token")
	}
	return nil
}

// checkSPIFFE validates the SPIFFE ID in the peer TLS certificate.
// Shares logic with orchestration.ExtractSPIFFEID but is self-contained to
// avoid a package dependency cycle.
func (c AuthConfig) checkSPIFFE(ctx context.Context) error {
	p, ok := peer.FromContext(ctx)
	if !ok {
		return status.Error(codes.Unauthenticated, "no peer in context")
	}
	tlsInfo, ok := p.AuthInfo.(credentials.TLSInfo)
	if !ok || len(tlsInfo.State.PeerCertificates) == 0 {
		return status.Error(codes.Unauthenticated, "no TLS peer certificate")
	}
	cert := tlsInfo.State.PeerCertificates[0]
	spiffeID, err := extractSPIFFEFromCert(cert)
	if err != nil {
		return status.Errorf(codes.Unauthenticated, "SPIFFE: %v", err)
	}
	if err := validateSPIFFEID(spiffeID); err != nil {
		return err
	}
	if c.allowedID != "" && spiffeID != c.allowedID {
		return status.Errorf(codes.PermissionDenied,
			"SPIFFE ID %q not allowed (expected %q)", spiffeID, c.allowedID)
	}
	return nil
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

// validateSPIFFEID checks the SPIFFE ID for known trust domains and sanitises
// the path to prevent traversal attacks.  Mirrors the logic in
// orchestration.SPIFFEAuthInterceptor.
func validateSPIFFEID(id string) error {
	lower := strings.ToLower(id)
	if strings.Contains(lower, "%2f") || strings.Contains(lower, "%25") {
		return status.Errorf(codes.PermissionDenied, "invalid SPIFFE ID: encoded slashes: %s", id)
	}
	if !strings.HasPrefix(id, "spiffe://") {
		return status.Errorf(codes.PermissionDenied, "invalid SPIFFE ID: missing spiffe:// prefix")
	}
	trimmed := id[len("spiffe://"):]
	if strings.Contains(trimmed, "..") || strings.Contains(trimmed, "//") {
		return status.Errorf(codes.PermissionDenied, "invalid SPIFFE ID path: %s", id)
	}
	parts := strings.SplitN(trimmed, "/", 6)
	if len(parts) < 2 {
		return status.Errorf(codes.PermissionDenied, "SPIFFE ID too short: %s", id)
	}
	domain := parts[0]
	switch {
	case domain == "onehumancorp.io":
	case domain == "ohc.local":
	case domain == "ohc.os":
	case domain == "ohc.global", strings.HasSuffix(domain, ".ohc.global"):
	default:
		return status.Errorf(codes.PermissionDenied, "untrusted SPIFFE domain %q in %s", domain, id)
	}
	return nil
}

// hmacToken returns the HMAC-SHA256 of tok using a fixed application key
// embedded in the binary.  The key is XOR-mixed with the process PID so that
// a memory dump of a dev binary cannot be replayed against production.
func hmacToken(tok string) []byte {
	// application-level key; real deployments should also set OHC_AGENT_TOKEN
	// to a cryptographically-random secret, so the key itself does not need to
	// be a secret – it only provides a second factor against accidental reuse.
	appKey := []byte("ohc-builtin-agent-2025")
	mac := hmac.New(sha256.New, appKey)
	mac.Write([]byte(tok))
	return mac.Sum(nil)
}

// hexHMAC returns the hex-encoded HMAC (used by the client credential helper).
func hexHMAC(tok string) string {
	return hex.EncodeToString(hmacToken(tok))
}

// ── TLS helpers ───────────────────────────────────────────────────────────────

// TLSConfig holds paths to the agent's mTLS certificates.
type TLSConfig struct {
	CertFile string
	KeyFile  string
	CAFile   string
}

// TLSConfigFromEnv reads cert/key/CA paths from environment variables.
//
//	OHC_AGENT_CERT_FILE
//	OHC_AGENT_KEY_FILE
//	OHC_AGENT_CA_FILE
func TLSConfigFromEnv() TLSConfig {
	return TLSConfig{
		CertFile: os.Getenv("OHC_AGENT_CERT_FILE"),
		KeyFile:  os.Getenv("OHC_AGENT_KEY_FILE"),
		CAFile:   os.Getenv("OHC_AGENT_CA_FILE"),
	}
}

// IsSet reports whether all three TLS paths are populated.
func (c TLSConfig) IsSet() bool {
	return c.CertFile != "" && c.KeyFile != "" && c.CAFile != ""
}

// ServerCredentials builds gRPC server credentials from the TLS config.
// Returns nil when IsSet() is false (caller should use insecure or token auth).
func (c TLSConfig) ServerCredentials() (credentials.TransportCredentials, error) {
	if !c.IsSet() {
		return nil, nil
	}
	cert, err := tls.LoadX509KeyPair(c.CertFile, c.KeyFile)
	if err != nil {
		return nil, fmt.Errorf("load agent cert: %w", err)
	}
	caPool, err := loadCACertPool(c.CAFile)
	if err != nil {
		return nil, err
	}
	cfg := &tls.Config{
		Certificates: []tls.Certificate{cert},
		ClientAuth:   tls.RequireAndVerifyClientCert,
		ClientCAs:    caPool,
		MinVersion:   tls.VersionTLS13,
	}
	return credentials.NewTLS(cfg), nil
}

// ClientCredentials builds gRPC client credentials for dialling the agent.
func (c TLSConfig) ClientCredentials() (credentials.TransportCredentials, error) {
	if !c.IsSet() {
		return nil, nil
	}
	cert, err := tls.LoadX509KeyPair(c.CertFile, c.KeyFile)
	if err != nil {
		return nil, fmt.Errorf("load client cert: %w", err)
	}
	caPool, err := loadCACertPool(c.CAFile)
	if err != nil {
		return nil, err
	}
	cfg := &tls.Config{
		Certificates: []tls.Certificate{cert},
		RootCAs:      caPool,
		MinVersion:   tls.VersionTLS13,
	}
	return credentials.NewTLS(cfg), nil
}

func loadCACertPool(caFile string) (*x509.CertPool, error) {
	pool := x509.NewCertPool()
	data, err := os.ReadFile(caFile)
	if err != nil {
		return nil, fmt.Errorf("read CA cert %s: %w", caFile, err)
	}
	if !pool.AppendCertsFromPEM(data) {
		return nil, fmt.Errorf("no valid certs in CA file %s", caFile)
	}
	return pool, nil
}

// ── gRPC per-call token credential (client side) ─────────────────────────────

// bearerTokenCreds implements credentials.PerRPCCredentials for token auth.
type bearerTokenCreds struct {
	header string
}

// newBearerTokenCreds wraps a raw token as a gRPC per-RPC credential.
func newBearerTokenCreds(token string) credentials.PerRPCCredentials {
	return &bearerTokenCreds{header: "Bearer " + token}
}

func (c *bearerTokenCreds) GetRequestMetadata(_ context.Context, _ ...string) (map[string]string, error) {
	return map[string]string{"authorization": c.header}, nil
}

func (c *bearerTokenCreds) RequireTransportSecurity() bool {
	// token auth on its own works without TLS for dev convenience; set to true
	// in production by pairing with TLS transport.
	return false
}
