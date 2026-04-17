package autodream

import (
    "context"
    "encoding/json"
    "sort"
    "math"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type SQLiteVectorStore struct {
    provider db.Provider
}

func NewSQLiteVectorStore(provider db.Provider) *SQLiteVectorStore {
    return &SQLiteVectorStore{provider: provider}
}

func (s *SQLiteVectorStore) Store(ctx context.Context, id string, vector []float32, metadata map[string]interface{}) error {
    vecJSON, err := json.Marshal(vector)
    if err != nil {
        return err
    }
    metaJSON, err := json.Marshal(metadata)
    if err != nil {
        return err
    }

    query := `
        INSERT INTO knowledge_base (id, embedding, metadata)
        VALUES ($1, $2, $3)
        ON CONFLICT (id) DO UPDATE SET embedding = EXCLUDED.embedding, metadata = EXCLUDED.metadata
    `
    _, err = s.provider.Exec(ctx, query, id, vecJSON, metaJSON)
    return err
}

func cosineDistance(a, b []float32) float64 {
    var dot, magA, magB float64
    for i := 0; i < len(a) && i < len(b); i++ {
        valA := float64(a[i])
        valB := float64(b[i])
        dot += valA * valB
        magA += valA * valA
        magB += valB * valB
    }
    if magA == 0 || magB == 0 {
        return 1.0 // Max distance if one is zero vector
    }
    cosSim := dot / (math.Sqrt(magA) * math.Sqrt(magB))
    return 1.0 - cosSim
}

func (s *SQLiteVectorStore) Search(ctx context.Context, vector []float32, limit int) ([]SearchResult, error) {
    query := "SELECT id, embedding, metadata FROM knowledge_base"
    rows, err := s.provider.Query(ctx, query)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var all []SearchResult
    for rows.Next() {
        var res SearchResult
        var vecStr, metaStr string
        if err := rows.Scan(&res.ID, &vecStr, &metaStr); err != nil {
            continue
        }
        var dbVec []float32
        if err := json.Unmarshal([]byte(vecStr), &dbVec); err != nil {
            continue
        }
        _ = json.Unmarshal([]byte(metaStr), &res.Metadata)

        res.Distance = cosineDistance(vector, dbVec)

        all = append(all, res)
    }

    sort.Slice(all, func(i, j int) bool {
        return all[i].Distance < all[j].Distance
    })

    if len(all) > limit {
        all = all[:limit]
    }
    return all, nil
}
