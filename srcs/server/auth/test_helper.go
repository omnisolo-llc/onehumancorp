package auth

import "context"

// ContextWithClaims creates a new context with the given claims for testing purposes
func ContextWithClaims(ctx context.Context, claims *Claims) context.Context {
	return context.WithValue(ctx, claimsContextKey, claims)
}
