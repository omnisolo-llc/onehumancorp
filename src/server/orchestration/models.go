package orchestration

import (
	"time"
)

type Tenant struct {
	ID         string    `json:"id" db:"id"`
	OwnerEmail string    `json:"owner_email" db:"owner_email"`
	Tier       string    `json:"tier" db:"tier"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
}

type Business struct {
	ID        string    `json:"id" db:"id"`
	TenantID  string    `json:"tenant_id" db:"tenant_id"`
	Name      string    `json:"name" db:"name"`
	Type      string    `json:"type" db:"type"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`
}

type AgentMemory struct {
	ID         string    `json:"id" db:"id"`
	TenantID   string    `json:"tenant_id" db:"tenant_id"`
	BusinessID string    `json:"business_id" db:"business_id"`
	Department string    `json:"department" db:"department"`
	Embeddings []float32 `json:"embeddings" db:"embeddings"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
}
