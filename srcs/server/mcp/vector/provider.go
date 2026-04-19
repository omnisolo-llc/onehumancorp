package vector

type VectorStorageProvider interface {
	Store(namespace, id string, vector []float32, metadata string) error
	Search(namespace string, queryVector []float32, topK int) ([]SearchResult, error)
}

type SearchResult struct {
	ID       string
	Metadata string
	Distance float64
}
