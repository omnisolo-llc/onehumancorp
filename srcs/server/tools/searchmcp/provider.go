package searchmcp

import (
	"context"
)

type Document struct {
	ID      string
	Content string
	Metadata map[string]string
}

type SearchResult struct {
	ID      string
	Content string
	Score   float64
}

type SearchProvider interface {
	Search(ctx context.Context, query string) ([]SearchResult, error)
	Index(ctx context.Context, doc Document) error
}

type Claims struct {
	OrganizationID string
}

type contextKey string

const ClaimsKey = contextKey("claims")

func ClaimsFromContext(ctx context.Context) *Claims {
	claims, ok := ctx.Value(ClaimsKey).(*Claims)
	if !ok {
		return nil
	}
	return claims
}
