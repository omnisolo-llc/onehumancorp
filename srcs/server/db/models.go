package db

import (
	"time"
)

// Product represents a business product within a tenant's scope.
type Product struct {
	ID        string    `json:"id" db:"id"`
	TenantID  string    `json:"tenant_id" db:"tenant_id"`
	Name      string    `json:"name" db:"name"`
	Price     float64   `json:"price" db:"price"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
}

// Order represents a customer's order for a tenant.
type Order struct {
	ID         string    `json:"id" db:"id"`
	TenantID   string    `json:"tenant_id" db:"tenant_id"`
	CustomerID string    `json:"customer_id" db:"customer_id"`
	Status     string    `json:"status" db:"status"`
	Total      float64   `json:"total" db:"total"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
}

// Customer represents a customer for a specific tenant.
type Customer struct {
	ID        string    `json:"id" db:"id"`
	TenantID  string    `json:"tenant_id" db:"tenant_id"`
	Name      string    `json:"name" db:"name"`
	Email     string    `json:"email" db:"email"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
}

// Booking represents a scheduled appointment or booking for a tenant.
type Booking struct {
	ID         string    `json:"id" db:"id"`
	TenantID   string    `json:"tenant_id" db:"tenant_id"`
	CustomerID string    `json:"customer_id" db:"customer_id"`
	Status     string    `json:"status" db:"status"`
	StartTime  time.Time `json:"start_time" db:"start_time"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
}

// Page represents a storefront or site page for a tenant.
type Page struct {
	ID        string    `json:"id" db:"id"`
	TenantID  string    `json:"tenant_id" db:"tenant_id"`
	Title     string    `json:"title" db:"title"`
	Content   string    `json:"content" db:"content"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
}

// Memory represents an embedded agent memory for a tenant.
type Memory struct {
	ID        string    `json:"id" db:"id"`
	TenantID  string    `json:"tenant_id" db:"tenant_id"`
	AgentID   string    `json:"agent_id" db:"agent_id"`
	Content   string    `json:"content" db:"content"`
	Embedding []float32 `json:"embedding" db:"embedding"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
}
