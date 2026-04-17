package autodream

import "context"

type SearchResult struct {
    ID       string
    Distance float64
    Metadata map[string]interface{}
}

type VectorStore interface {
    Store(ctx context.Context, id string, vector []float32, metadata map[string]interface{}) error
    Search(ctx context.Context, vector []float32, limit int) ([]SearchResult, error)
}
