package autodream

import (
    "context"
    "encoding/json"
    "fmt"
    "strings"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type PGVectorStore struct {
    provider db.Provider
}

func NewPGVectorStore(provider db.Provider) *PGVectorStore {
    return &PGVectorStore{provider: provider}
}

func (s *PGVectorStore) Store(ctx context.Context, id string, vector []float32, metadata map[string]interface{}) error {
    var vecStrs []string
    for _, v := range vector {
        vecStrs = append(vecStrs, fmt.Sprintf("%f", v))
    }
    vecStr := "[" + strings.Join(vecStrs, ",") + "]"

    metaJSON, err := json.Marshal(metadata)
    if err != nil {
        return err
    }

    query := `
        INSERT INTO knowledge_base (id, embedding, metadata)
        VALUES ($1, $2::vector, $3)
        ON CONFLICT (id) DO UPDATE SET embedding = EXCLUDED.embedding, metadata = EXCLUDED.metadata
    `
    _, err = s.provider.Exec(ctx, query, id, vecStr, metaJSON)
    return err
}

func (s *PGVectorStore) Search(ctx context.Context, vector []float32, limit int) ([]SearchResult, error) {
    var vecStrs []string
    for _, v := range vector {
        vecStrs = append(vecStrs, fmt.Sprintf("%f", v))
    }
    vecStr := "[" + strings.Join(vecStrs, ",") + "]"

    query := `
        SELECT id, metadata, embedding <=> $1::vector as distance
        FROM knowledge_base
        ORDER BY distance ASC
        LIMIT $2
    `
    rows, err := s.provider.Query(ctx, query, vecStr, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var results []SearchResult
    for rows.Next() {
        var res SearchResult
        var metaStr string
        if err := rows.Scan(&res.ID, &metaStr, &res.Distance); err != nil {
            continue
        }
        _ = json.Unmarshal([]byte(metaStr), &res.Metadata)
        results = append(results, res)
    }
    return results, nil
}
