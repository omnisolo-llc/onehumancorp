package vector

import (
	"database/sql"
	"fmt"
	"strings"
)

type pgvectorProvider struct {
	db *sql.DB
}

func NewPGVectorProvider(db *sql.DB) VectorStorageProvider {
	return &pgvectorProvider{db: db}
}

func (p *pgvectorProvider) Store(namespace, id string, vector []float32, metadata string) error {
	vecStrs := make([]string, len(vector))
	for i, v := range vector {
		vecStrs[i] = fmt.Sprintf("%f", v)
	}
	vecStr := "[" + strings.Join(vecStrs, ",") + "]"

	query := `
		INSERT INTO vectors (namespace, id, vector, metadata)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT (namespace, id) DO UPDATE
		SET vector = EXCLUDED.vector, metadata = EXCLUDED.metadata
	`
	_, err := p.db.Exec(query, namespace, id, vecStr, metadata)
	return err
}

func (p *pgvectorProvider) Search(namespace string, queryVector []float32, topK int) ([]SearchResult, error) {
	vecStrs := make([]string, len(queryVector))
	for i, v := range queryVector {
		vecStrs[i] = fmt.Sprintf("%f", v)
	}
	vecStr := "[" + strings.Join(vecStrs, ",") + "]"

	// Using L2 distance (<->) for similarity search
	query := `
		SELECT id, metadata, vector <-> $1 AS distance
		FROM vectors
		WHERE namespace = $2
		ORDER BY distance
		LIMIT $3
	`

	rows, err := p.db.Query(query, vecStr, namespace, topK)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var res SearchResult
		if err := rows.Scan(&res.ID, &res.Metadata, &res.Distance); err != nil {
			return nil, err
		}
		results = append(results, res)
	}

	return results, rows.Err()
}
