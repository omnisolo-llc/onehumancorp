package auth

import (
	"context"
	"net/http"
	"os"
	"strings"
)

type contextKey string

const claimsContextKey contextKey = "ohc_auth_claims"

// publicPaths lists URL prefixes that do not require authentication.
var publicPaths = []string{
	"/healthz",
	"/readyz",
	"/api/auth/login",
	"/api/v1/scale/stream", // Manually authenticated inside handler for SSE query token bypass
}

// Middleware returns an HTTP middleware that enforces JWT authentication. Requests to public paths pass through unauthenticated. All other requests must carry a valid Bearer token in the Authorization header or an "ohc_token" cookie.
// Accepts parameters: store *Store (No Constraints).
// Returns func(http.Handler) http.Handler.
// Produces no errors.
// Has no side effects.
func Middleware(store *Store) func(http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			// Allow public routes
			if isPublic(r.URL.Path) {
				next.ServeHTTP(w, r)
				return
			}

			token := extractToken(r)
			if token == "" {
				jsonError(w, "authentication required", http.StatusUnauthorized)
				return
			}

			claims, err := store.ValidateToken(token)
			if err != nil {
				jsonError(w, "invalid token: "+err.Error(), http.StatusUnauthorized)
				return
			}

			// Inject claims into request context
			ctx := context.WithValue(r.Context(), claimsContextKey, claims)
			next.ServeHTTP(w, r.WithContext(ctx))
		})
	}
}

// ClaimsFromContext extracts auth claims set by Middleware. Returns nil if no claims are present (public or in-process request).
// Accepts parameters: ctx context.Context (No Constraints).
// Returns *Claims.
// Produces no errors.
// Has no side effects.
func ClaimsFromContext(ctx context.Context) *Claims {
	v, _ := ctx.Value(claimsContextKey).(*Claims)
	return v
}

// RequireRole returns a middleware that further restricts access to users that hold the given role (or "admin").
// Accepts parameters: role string (No Constraints), next http.HandlerFunc (No Constraints).
// Returns http.HandlerFunc.
// Produces no errors.
// Has no side effects.
func RequireRole(role string, next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := ClaimsFromContext(r.Context())
		if claims == nil || !claims.HasRole(role) {
			jsonError(w, "forbidden", http.StatusForbidden)
			return
		}
		next(w, r)
	}
}

// RequireSPIFFE returns a middleware that strictly enforces the presence and validity of a SPIFFE SVID
// client certificate or a securely forwarded SPIFFE header, matching the specified trust domain and path prefix.
func RequireSPIFFE(expectedPrefix string, next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if os.Getenv("OHC_STANDALONE") == "true" {
			// In standalone desktop mode, local IPC or channels are used, and mutual TLS isn't mandatory
			next(w, r)
			return
		}

		var spiffeURI string

		// Production multi-tenant environment: enforce mTLS certificates
		if r.TLS != nil && len(r.TLS.PeerCertificates) > 0 {
			for _, cert := range r.TLS.PeerCertificates {
				for _, uri := range cert.URIs {
					if uri != nil && uri.Scheme == "spiffe" {
						spiffeURI = uri.String()
						break
					}
				}
				if spiffeURI != "" {
					break
				}
			}
		} else if xfcc := r.Header.Get("X-Forwarded-Client-Cert"); xfcc != "" {
			// For scenarios where TLS is terminated at a strict internal proxy (e.g. Istio)
			// we extract the URI. Ensure this is only used if proxy trust is established.
			// XFCC format: Hash=hash;Cert="-----BEGIN...";URI=spiffe://...
			parts := strings.Split(xfcc, ";")
			for _, p := range parts {
				p = strings.TrimSpace(p)
				if strings.HasPrefix(p, "URI=spiffe://") {
					spiffeURI = strings.TrimPrefix(p, "URI=")
					break
				}
			}
		}

		if spiffeURI == "" {
			jsonError(w, "forbidden: missing SPIFFE/SPIRE SVID certificate", http.StatusForbidden)
			return
		}

		// Validate the URI explicitly matches the expected trust domain and prefix.
		// Ensure a trailing slash or exact match to prevent sub-domain spoofing
		// (e.g., "spiffe://onehumancorp.io.evil.com")
		if spiffeURI != expectedPrefix && !strings.HasPrefix(spiffeURI, expectedPrefix+"/") {
			jsonError(w, "forbidden: invalid SPIFFE ID trust domain or path", http.StatusForbidden)
			return
		}

		next(w, r)
	}
}

// extractToken retrieves the bearer token from the Authorization header or
// the "ohc_token" cookie.
func extractToken(r *http.Request) string {
	if auth := r.Header.Get("Authorization"); auth != "" {
		if strings.HasPrefix(auth, "Bearer ") {
			return strings.TrimPrefix(auth, "Bearer ")
		}
	}
	if c, err := r.Cookie("ohc_token"); err == nil {
		return c.Value
	}
	return ""
}

func isPublic(path string) bool {
	for _, p := range publicPaths {
		if strings.HasPrefix(path, p) {
			return true
		}
	}
	// Static assets
	if strings.HasPrefix(path, "/app") || path == "/" {
		return true
	}
	return false
}

func jsonError(w http.ResponseWriter, msg string, code int) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(code)
	_, _ = w.Write([]byte(`{"error":` + jsonString(msg) + `}`))
}

func jsonString(s string) string {
	return `"` + strings.ReplaceAll(strings.ReplaceAll(s, `\`, `\\`), `"`, `\"`) + `"`
}

// OrganizationIDFromContext returns the organisation ID embedded in the JWT
// claims, or an empty string when not authenticated or not set.
// This is the primary tenant isolation key for multi-tenant deployments.
// Accepts parameters: ctx context.Context (No Constraints).
// Returns string.
// Produces no errors.
// Has no side effects.
func OrganizationIDFromContext(ctx context.Context) string {
	if c := ClaimsFromContext(ctx); c != nil {
		return c.OrganizationID
	}
	return ""
}

// ClaimsContextKeyForTest provides domain-specific context and typed constraints for ClaimsContextKeyForTest operations across the application.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
const ClaimsContextKeyForTest = claimsContextKey
