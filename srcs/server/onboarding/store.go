package onboarding

import (
	"context"
	"database/sql"
	"errors"
	"time"
)

type Tenant struct {
	ID          string
	OwnerEmail  string
	Tier        string
	Name        string
	Category    string
	Description string
	Status      string
	State       string
	CreatedAt   time.Time
	UpdatedAt   time.Time
}

type TenantStore interface {
	CreateTenant(ctx context.Context, tenant *Tenant) error
	GetTenant(ctx context.Context, id string) (*Tenant, error)
	UpdateTenantStatus(ctx context.Context, id string, status string) error
	UpdateTenantState(ctx context.Context, id string, state string) error
}







type SqliteTenantStore struct {
	db *sql.DB
}

func NewSqliteTenantStore(db *sql.DB) *SqliteTenantStore {
	return &SqliteTenantStore{db: db}
}

func (s *SqliteTenantStore) CreateTenant(ctx context.Context, tenant *Tenant) error {
	query := `
		INSERT INTO tenants (id, name, category, description, status, owner_email, tier, state, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	if tenant.ID == "" {
		tenant.ID = "id-" + time.Now().Format("20060102150405.000000") // Mock UUID
	}
	if tenant.Status == "" {
		tenant.Status = "PENDING"
	}
	if tenant.Tier == "" {
		tenant.Tier = "free"
	}

	_, err := s.db.ExecContext(ctx, query,
		tenant.ID, tenant.Name, tenant.Category, tenant.Description, tenant.Status,
		tenant.OwnerEmail, tenant.Tier, tenant.State,
	)

	if err == nil {
		tenant.CreatedAt = time.Now()
		tenant.UpdatedAt = time.Now()
	}

	return err
}

func (s *SqliteTenantStore) GetTenant(ctx context.Context, id string) (*Tenant, error) {
	query := `
		SELECT id, name, category, description, status, created_at, updated_at, owner_email, tier, COALESCE(state, '')
		FROM tenants
		WHERE id = ?
	`
	row := s.db.QueryRowContext(ctx, query, id)

	tenant := &Tenant{}
	var createdAtStr, updatedAtStr string
	err := row.Scan(
		&tenant.ID, &tenant.Name, &tenant.Category, &tenant.Description, &tenant.Status,
		&createdAtStr, &updatedAtStr, &tenant.OwnerEmail, &tenant.Tier, &tenant.State,
	)

	if err == sql.ErrNoRows {
		return nil, errors.New("tenant not found")
	} else if err != nil {
		return nil, err
	}

	if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
		tenant.CreatedAt = t
	}
	if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
		tenant.UpdatedAt = t
	}

	return tenant, nil
}

func (s *SqliteTenantStore) UpdateTenantStatus(ctx context.Context, id string, status string) error {
	query := `UPDATE tenants SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
	_, err := s.db.ExecContext(ctx, query, status, id)
	return err
}

func (s *SqliteTenantStore) UpdateTenantState(ctx context.Context, id string, state string) error {
	query := `UPDATE tenants SET state = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
	_, err := s.db.ExecContext(ctx, query, state, id)
	return err
}
