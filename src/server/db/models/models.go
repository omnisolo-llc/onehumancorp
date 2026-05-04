package models

import (
	"time"
)

// Tenant represents a business organization in OHC
type Tenant struct {
	ID               string    `json:"id" db:"id"`
	BusinessName     string    `json:"business_name" db:"business_name"`
	OwnerEmail       string    `json:"owner_email" db:"owner_email"`
	SubscriptionTier string    `json:"subscription_tier" db:"subscription_tier"`
	CreatedAt        time.Time `json:"created_at" db:"created_at"`
	UpdatedAt        time.Time `json:"updated_at" db:"updated_at"`
	SyncStatus       string    `json:"_sync_status" db:"_sync_status"`
	Version          int       `json:"version" db:"version"`
}

// Product represents physical goods, digital products, or services
type Product struct {
	TenantID        string    `json:"tenant_id" db:"tenant_id"`
	ID              string    `json:"id" db:"id"`
	Type            string    `json:"type" db:"type"`
	Title           string    `json:"title" db:"title"`
	PriceCents      int64     `json:"price_cents" db:"price_cents"`
	StockLevel      int       `json:"stock_level" db:"stock_level"`
	IsActive        bool      `json:"is_active" db:"is_active"`
	CreatedAt       time.Time `json:"created_at" db:"created_at"`
	UpdatedAt       time.Time `json:"updated_at" db:"updated_at"`
	SyncStatus      string    `json:"_sync_status" db:"_sync_status"`
	Version         int       `json:"version" db:"version"`
}

// Customer represents a person who interacts with a business
type Customer struct {
	TenantID   string    `json:"tenant_id" db:"tenant_id"`
	ID         string    `json:"id" db:"id"`
	Name       string    `json:"name" db:"name"`
	Email      string    `json:"email" db:"email"`
	Phone      string    `json:"phone" db:"phone"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
	UpdatedAt  time.Time `json:"updated_at" db:"updated_at"`
	SyncStatus string    `json:"_sync_status" db:"_sync_status"`
	Version    int       `json:"version" db:"version"`
}

// OrderBooking represents a purchase or a scheduled service
type OrderBooking struct {
	TenantID         string     `json:"tenant_id" db:"tenant_id"`
	ID               string     `json:"id" db:"id"`
	CustomerID       string     `json:"customer_id" db:"customer_id"`
	Status           string     `json:"status" db:"status"`
	TotalAmountCents int64      `json:"total_amount_cents" db:"total_amount_cents"`
	ScheduledFor     *time.Time `json:"scheduled_for,omitempty" db:"scheduled_for"`
	CreatedAt        time.Time  `json:"created_at" db:"created_at"`
	UpdatedAt        time.Time  `json:"updated_at" db:"updated_at"`
	SyncStatus       string     `json:"_sync_status" db:"_sync_status"`
	Version          int        `json:"version" db:"version"`
}

// OrderItem is a line item within an order
type OrderItem struct {
	TenantID       string    `json:"tenant_id" db:"tenant_id"`
	ID             string    `json:"id" db:"id"`
	OrderID        string    `json:"order_id" db:"order_id"`
	ProductID      string    `json:"product_id" db:"product_id"`
	Quantity       int       `json:"quantity" db:"quantity"`
	UnitPriceCents int64     `json:"unit_price_cents" db:"unit_price_cents"`
	CreatedAt      time.Time `json:"created_at" db:"created_at"`
	UpdatedAt      time.Time `json:"updated_at" db:"updated_at"`
	SyncStatus     string    `json:"_sync_status" db:"_sync_status"`
	Version        int       `json:"version" db:"version"`
}

// AIAgentMemory represents contextual facts stored by AI agents
type AIAgentMemory struct {
	TenantID       string    `json:"tenant_id" db:"tenant_id"`
	ID             string    `json:"id" db:"id"`
	Department     string    `json:"department" db:"department"`
	ContextSummary string    `json:"context_summary" db:"context_summary"`
	Embedding      []float32 `json:"embedding,omitempty" db:"embedding"`
	CreatedAt      time.Time `json:"created_at" db:"created_at"`
	UpdatedAt      time.Time `json:"updated_at" db:"updated_at"`
	SyncStatus     string    `json:"_sync_status" db:"_sync_status"`
	Version        int       `json:"version" db:"version"`
}
