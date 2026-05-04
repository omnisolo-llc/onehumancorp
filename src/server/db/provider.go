package db

import (
	"context"
	"onehumancorp/src/server/db/models"
)

// Provider defines the interface for database interactions across Cloud and Standalone modes.
type Provider interface {
	// Tenant operations
	GetTenant(ctx context.Context, id string) (*models.Tenant, error)
	CreateTenant(ctx context.Context, tenant *models.Tenant) error

	// Product operations
	GetProduct(ctx context.Context, tenantID, id string) (*models.Product, error)
	ListProducts(ctx context.Context, tenantID string) ([]models.Product, error)
	SaveProduct(ctx context.Context, product *models.Product) error

	// Customer operations
	GetCustomer(ctx context.Context, tenantID, id string) (*models.Customer, error)
	SaveCustomer(ctx context.Context, customer *models.Customer) error

	// Order/Booking operations
	GetOrder(ctx context.Context, tenantID, id string) (*models.OrderBooking, error)
	SaveOrder(ctx context.Context, order *models.OrderBooking, items []models.OrderItem) error

	// AI Memory operations
	QueryMemories(ctx context.Context, tenantID, department string, embedding []float32, limit int) ([]models.AIAgentMemory, error)
	SaveMemory(ctx context.Context, memory *models.AIAgentMemory) error

	// Utility
	IsSQLite() bool
}
