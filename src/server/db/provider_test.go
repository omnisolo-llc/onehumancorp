package db

import (
	"context"
	"errors"
	"testing"
	"onehumancorp/src/server/db/models"
)

// MockProvider implements the Provider interface for testing purposes.
type MockProvider struct {
	CurrentTenant string
	Tenants       map[string]*models.Tenant
	Products      map[string]*models.Product // key: tenantID:id
}

func (m *MockProvider) GetTenant(ctx context.Context, id string) (*models.Tenant, error) {
	if m.CurrentTenant != "system" && m.CurrentTenant != id {
		return nil, errors.New("access denied: tenant isolation violation")
	}
	t, ok := m.Tenants[id]
	if !ok {
		return nil, errors.New("not found")
	}
	return t, nil
}

func (m *MockProvider) CreateTenant(ctx context.Context, tenant *models.Tenant) error {
	m.Tenants[tenant.ID] = tenant
	return nil
}

func (m *MockProvider) GetProduct(ctx context.Context, tenantID, id string) (*models.Product, error) {
	if m.CurrentTenant != "system" && m.CurrentTenant != tenantID {
		return nil, errors.New("access denied: tenant isolation violation")
	}
	p, ok := m.Products[tenantID+":"+id]
	if !ok {
		return nil, errors.New("not found")
	}
	return p, nil
}

func (m *MockProvider) ListProducts(ctx context.Context, tenantID string) ([]models.Product, error) {
	if m.CurrentTenant != "system" && m.CurrentTenant != tenantID {
		return nil, errors.New("access denied: tenant isolation violation")
	}
	var res []models.Product
	for _, p := range m.Products {
		if p.TenantID == tenantID {
			res = append(res, *p)
		}
	}
	return res, nil
}

func (m *MockProvider) SaveProduct(ctx context.Context, product *models.Product) error {
	if m.CurrentTenant != "system" && m.CurrentTenant != product.TenantID {
		return errors.New("access denied: tenant isolation violation")
	}
	m.Products[product.TenantID+":"+product.ID] = product
	return nil
}

func (m *MockProvider) GetCustomer(ctx context.Context, tenantID, id string) (*models.Customer, error) { return nil, nil }
func (m *MockProvider) SaveCustomer(ctx context.Context, customer *models.Customer) error { return nil }
func (m *MockProvider) GetOrder(ctx context.Context, tenantID, id string) (*models.OrderBooking, error) { return nil, nil }
func (m *MockProvider) SaveOrder(ctx context.Context, order *models.OrderBooking, items []models.OrderItem) error { return nil }
func (m *MockProvider) QueryMemories(ctx context.Context, tenantID, department string, embedding []float32, limit int) ([]models.AIAgentMemory, error) { return nil, nil }
func (m *MockProvider) SaveMemory(ctx context.Context, memory *models.AIAgentMemory) error { return nil }
func (m *MockProvider) IsSQLite() bool { return false }

func TestTenantIsolation(t *testing.T) {
	p := &MockProvider{
		Tenants:  make(map[string]*models.Tenant),
		Products: make(map[string]*models.Product),
	}

	ctx := context.Background()

	// Setup data
	tenant1 := &models.Tenant{ID: "tenant-1", BusinessName: "Maya's Bakery"}
	tenant2 := &models.Tenant{ID: "tenant-2", BusinessName: "Leo's Music"}
	p.CreateTenant(ctx, tenant1)
	p.CreateTenant(ctx, tenant2)

	prod1 := &models.Product{TenantID: "tenant-1", ID: "cake-1", Title: "Chocolate Cake"}
	p.CurrentTenant = "system"
	p.SaveProduct(ctx, prod1)

	// Test Case 1: Tenant 1 accesses their own product - Success
	p.CurrentTenant = "tenant-1"
	_, err := p.GetProduct(ctx, "tenant-1", "cake-1")
	if err != nil {
		t.Errorf("Expected success for tenant-1 accessing own product, got: %v", err)
	}

	// Test Case 2: Tenant 2 tries to access Tenant 1's product - Failure
	p.CurrentTenant = "tenant-2"
	_, err = p.GetProduct(ctx, "tenant-1", "cake-1")
	if err == nil {
		t.Error("Expected error for tenant-2 accessing tenant-1's product, but got nil")
	} else if err.Error() != "access denied: tenant isolation violation" {
		t.Errorf("Expected isolation violation error, got: %v", err)
	}

	// Test Case 3: System accesses any product - Success
	p.CurrentTenant = "system"
	_, err = p.GetProduct(ctx, "tenant-1", "cake-1")
	if err != nil {
		t.Errorf("Expected system to have access to any product, got: %v", err)
	}
}
