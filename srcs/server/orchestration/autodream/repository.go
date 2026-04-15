package autodream

import (
	"context"
	"fmt"
	"math"
	"sort"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type Memory struct {
	OrganizationID string
	ID        string
	TaskID    string
	Content   string
	Embedding []float32
	CreatedAt string
}

type Repository struct {
	Provider db.Provider
}

func NewRepository(provider db.Provider) *Repository {
	return &Repository{Provider: provider}
}

func formatVector(v []float32) string {
	strs := make([]string, len(v))
	for i, f := range v {
		strs[i] = fmt.Sprintf("%f", f)
	}
	return "[" + strings.Join(strs, ",") + "]"
}

func cosineDistance(a, b []float32) float32 {
	var dotProduct, normA, normB float32
	for i := range a {
		dotProduct += a[i] * b[i]
		normA += a[i] * a[i]
		normB += b[i] * b[i]
	}
	if normA == 0 || normB == 0 {
		return 1.0
	}
	return 1.0 - (dotProduct / (float32(math.Sqrt(float64(normA))) * float32(math.Sqrt(float64(normB)))))
}

func (r *Repository) Insert(ctx context.Context, mem *Memory) error {
	query := `INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding) VALUES ($1, $2, $3, $4, $5)`

	embeddingStr := formatVector(mem.Embedding)
	if r.Provider.IsSQLite() {
		query = `INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding) VALUES (?, ?, ?, ?, ?)`
		embeddingStr = fmt.Sprintf("%v", mem.Embedding)
	}

	_, err := r.Provider.Exec(ctx, query, mem.ID, mem.OrganizationID, mem.TaskID, mem.Content, embeddingStr)
	return err
}

func (r *Repository) Search(ctx context.Context, queryEmbedding []float32, limit int) ([]Memory, error) {
	var rows db.Rows
	var err error

	if r.Provider.IsSQLite() {
		query := `SELECT id, organization_id, task_id, content, created_at, embedding FROM autodream_memories`
		rows, err = r.Provider.Query(ctx, query)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		type memoryWithDist struct {
			Memory
			Dist float32
		}
		var all []memoryWithDist

		for rows.Next() {
			var mem Memory
			var taskID *string
			var embStr string
			if err := rows.Scan(&mem.ID, &mem.OrganizationID, &taskID, &mem.Content, &mem.CreatedAt, &embStr); err != nil {
				return nil, err
			}
			if taskID != nil {
				mem.TaskID = *taskID
			}

			// Parse embedding string back to []float32 for SQLite degradation
			var parsedEmb []float32
			embStr = strings.Trim(embStr, "[]")
			if len(embStr) > 0 {
				parts := strings.Split(embStr, " ")
				for _, p := range parts {
					var f float32
					fmt.Sscanf(p, "%f", &f)
					parsedEmb = append(parsedEmb, f)
				}
			}

			dist := cosineDistance(queryEmbedding, parsedEmb)
			all = append(all, memoryWithDist{Memory: mem, Dist: dist})
		}

		sort.Slice(all, func(i, j int) bool {
			return all[i].Dist < all[j].Dist
		})

		var result []Memory
		for i := 0; i < len(all) && i < limit; i++ {
			result = append(result, all[i].Memory)
		}
		return result, nil

	} else {
		query := `SELECT id, organization_id, task_id, content, created_at FROM autodream_memories ORDER BY embedding <=> $1 LIMIT $2`
		embeddingStr := formatVector(queryEmbedding)
		rows, err = r.Provider.Query(ctx, query, embeddingStr, limit)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		var memories []Memory
		for rows.Next() {
			var mem Memory
			var taskID *string
			if err := rows.Scan(&mem.ID, &mem.OrganizationID, &taskID, &mem.Content, &mem.CreatedAt); err != nil {
				return nil, err
			}
			if taskID != nil {
				mem.TaskID = *taskID
			}
			memories = append(memories, mem)
		}
		return memories, nil
	}
}
