package builder

import (
	"time"
)

// Site represents an entire generated website for a tenant.
type Site struct {
	ID           string    `json:"id"`
	TenantID     string    `json:"tenant_id"`
	Domain       string    `json:"domain"`
	CustomDomain string    `json:"custom_domain"`
	Status       string    `json:"status"` // e.g. DRAFT, PUBLISHED
	CreatedAt    time.Time `json:"created_at"`
	UpdatedAt    time.Time `json:"updated_at"`
}

// Page represents a single page on a Site.
type Page struct {
	ID        string    `json:"id"`
	SiteID    string    `json:"site_id"`
	TenantID  string    `json:"tenant_id"` // Denormalized for RLS
	Path      string    `json:"path"`
	Title     string    `json:"title"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

// Block represents a single visual block on a page.
type Block struct {
	ID        string    `json:"id"`
	PageID    string    `json:"page_id"`
	TenantID  string    `json:"tenant_id"` // Denormalized for RLS
	Type      string    `json:"type"` // e.g. HeroBlock, ProductGridBlock
	OrderIdx  int       `json:"order_idx"`
	Content   string    `json:"content"` // JSONB content string
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}
