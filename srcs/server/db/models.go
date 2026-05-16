package db

import (
	"encoding/json"
	"time"
)

// Tenant represents a business owner on the OHC platform.
type Tenant struct {
	ID        string    `json:"id" db:"id"`
	Name      string    `json:"name" db:"name"`
	Domain    string    `json:"domain" db:"domain"`
	Tier      string    `json:"tier" db:"tier"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
}

// Customer represents a customer of a specific tenant.
type Customer struct {
	ID          string          `json:"id" db:"id"`
	TenantID    string          `json:"tenant_id" db:"tenant_id"`
	Email       string          `json:"email" db:"email"`
	Phone       *string         `json:"phone,omitempty" db:"phone"`
	Preferences json.RawMessage `json:"preferences" db:"preferences"`
	LastActive  *time.Time      `json:"last_active,omitempty" db:"last_active"`
	CreatedAt   time.Time       `json:"created_at" db:"created_at"`
}

// CatalogItem represents a unified item (product, service, digital, subscription) offered by a tenant.
type CatalogItem struct {
	ID          string    `json:"id" db:"id"`
	TenantID    string    `json:"tenant_id" db:"tenant_id"`
	Title       string    `json:"title" db:"title"`
	Description *string   `json:"description,omitempty" db:"description"`
	ItemType    string    `json:"item_type" db:"item_type"` // "product | service | digital | subscription"
	IsActive    bool      `json:"is_active" db:"is_active"`
	CreatedAt   time.Time `json:"created_at" db:"created_at"`
}

// ItemVariant represents a variant of a CatalogItem.
type ItemVariant struct {
	ID             string          `json:"id" db:"id"`
	TenantID       string          `json:"tenant_id" db:"tenant_id"`
	CatalogItemID  string          `json:"catalog_item_id" db:"catalog_item_id"`
	SKU            string          `json:"sku" db:"sku"`
	Price          float64         `json:"price" db:"price"`
	InventoryCount int             `json:"inventory_count" db:"inventory_count"`
	Attributes     json.RawMessage `json:"attributes" db:"attributes"`
	CreatedAt      time.Time       `json:"created_at" db:"created_at"`
}

// Order represents a customer's order for a tenant.
type Order struct {
	ID          string    `json:"id" db:"id"`
	TenantID    string    `json:"tenant_id" db:"tenant_id"`
	CustomerID  string    `json:"customer_id" db:"customer_id"`
	Status      string    `json:"status" db:"status"` // "draft | pending_payment | confirmed | fulfilled | cancelled"
	TotalAmount float64   `json:"total_amount" db:"total_amount"`
	CreatedAt   time.Time `json:"created_at" db:"created_at"`
}

// OrderLineItem represents an individual line item within an Order.
type OrderLineItem struct {
	ID         string    `json:"id" db:"id"`
	TenantID   string    `json:"tenant_id" db:"tenant_id"`
	OrderID    string    `json:"order_id" db:"order_id"`
	VariantID  string    `json:"variant_id" db:"variant_id"`
	Quantity   int       `json:"quantity" db:"quantity"`
	UnitPrice  float64   `json:"unit_price" db:"unit_price"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
}

// AgentMemory represents an embedded agent memory associated with a tenant and optionally a customer.
type AgentMemory struct {
	ID         string          `json:"id" db:"id"`
	TenantID   string          `json:"tenant_id" db:"tenant_id"`
	CustomerID *string         `json:"customer_id,omitempty" db:"customer_id"`
	Department string          `json:"department" db:"department"`
	Embedding  interface{}     `json:"embedding" db:"embedding"` // Assuming specific library type or string representation
	RawContext json.RawMessage `json:"raw_context" db:"raw_context"`
	CreatedAt  time.Time       `json:"created_at" db:"created_at"`
}
// Task represents a task in the Shared Task List State Machine.
type Task struct {
	ID        string    `json:"id" db:"id"`
	Status    string    `json:"status" db:"status"` // "PENDING", "IN_PROGRESS", "COMPLETED", "FAILED"
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`
}
