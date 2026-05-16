package crdtmcp

import (
	"context"
	"database/sql"
	"fmt"
	"time"
)

type CrdtDelta struct {
	ID        string    `json:"id"`
	EntityID  string    `json:"entity_id"`
	Data      string    `json:"data"`
	UpdatedAt time.Time `json:"updated_at"`
}

type Provider interface {
	Pull(ctx context.Context) ([]CrdtDelta, error)
	Push(ctx context.Context, deltas []CrdtDelta) error
}

type LocalProvider struct {
	DB *sql.DB
}

func (p *LocalProvider) Pull(ctx context.Context) ([]CrdtDelta, error) {
	rows, err := p.DB.QueryContext(ctx, "SELECT id, entity_id, data, updated_at FROM crdt_deltas")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var deltas []CrdtDelta
	for rows.Next() {
		var d CrdtDelta
		var dataStr string
		if err := rows.Scan(&d.ID, &d.EntityID, &dataStr, &d.UpdatedAt); err != nil {
			return nil, err
		}
		d.Data = dataStr
		deltas = append(deltas, d)
	}
	return deltas, nil
}

func (p *LocalProvider) Push(ctx context.Context, deltas []CrdtDelta) error {
	for _, d := range deltas {
		_, err := p.DB.ExecContext(ctx, `
			INSERT INTO crdt_deltas (id, entity_id, data, updated_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE
			SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at
			WHERE crdt_deltas.updated_at < EXCLUDED.updated_at
		`, d.ID, d.EntityID, d.Data, d.UpdatedAt)
		if err != nil {
			return err
		}
	}
	return nil
}

type CrdtMCP struct {
	provider Provider
}

func NewCrdtMCP(provider Provider) *CrdtMCP {
	return &CrdtMCP{provider: provider}
}

func (m *CrdtMCP) CallTool(ctx context.Context, name string, args map[string]interface{}) (interface{}, error) {
	switch name {
	case "crdt_pull":
		deltas, err := m.provider.Pull(ctx)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"deltas": deltas,
		}, nil
	case "crdt_push":
		// parse args
		deltasRaw, ok := args["deltas"].([]interface{})
		if !ok {
			return nil, fmt.Errorf("missing or invalid deltas argument")
		}
		var deltas []CrdtDelta
		for _, raw := range deltasRaw {
			m, ok := raw.(map[string]interface{})
			if !ok {
				continue
			}
			id, _ := m["id"].(string)
			entityID, _ := m["entity_id"].(string)
			data, _ := m["data"].(string)
			updatedAtStr, _ := m["updated_at"].(string)
			updatedAt, _ := time.Parse(time.RFC3339, updatedAtStr)

			deltas = append(deltas, CrdtDelta{
				ID:        id,
				EntityID:  entityID,
				Data:      data,
				UpdatedAt: updatedAt,
			})
		}
		err := m.provider.Push(ctx, deltas)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

func (m *CrdtMCP) ListTools() []map[string]interface{} {
	return []map[string]interface{}{
		{
			"name":        "crdt_pull",
			"description": "Pull CRDT deltas from the local database.",
			"parameters": map[string]interface{}{
				"type":       "object",
				"properties": map[string]interface{}{},
			},
		},
		{
			"name":        "crdt_push",
			"description": "Push CRDT deltas to the local database.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"deltas": map[string]interface{}{
						"type": "array",
						"items": map[string]interface{}{
							"type": "object",
							"properties": map[string]interface{}{
								"id": map[string]interface{}{
									"type": "string",
								},
								"entity_id": map[string]interface{}{
									"type": "string",
								},
								"data": map[string]interface{}{
									"type": "string",
								},
								"updated_at": map[string]interface{}{
									"type": "string",
								},
							},
						},
					},
				},
			},
		},
	}
}
