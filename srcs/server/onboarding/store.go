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
	CreatedAt   time.Time
	UpdatedAt   time.Time
}

type TenantStore interface {
	CreateTenant(ctx context.Context, tenant *Tenant) error
	GetTenant(ctx context.Context, id string) (*Tenant, error)
	UpdateTenantStatus(ctx context.Context, id string, status string) error
}

type PostgresTenantStore struct {
	db *sql.DB
}

func NewPostgresTenantStore(db *sql.DB) *PostgresTenantStore {
	return &PostgresTenantStore{db: db}
}

func (s *PostgresTenantStore) CreateTenant(ctx context.Context, tenant *Tenant) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `
		INSERT INTO tenants (name, category, description, status, owner_email, tier)
		VALUES ($1, $2, $3, $4, $5, $6)
		RETURNING id, created_at, updated_at
	`

	if tenant.Status == "" {
		tenant.Status = "PENDING"
	}

	err = tx.QueryRowContext(ctx, query,
		tenant.Name, tenant.Category, tenant.Description, tenant.Status,
		tenant.OwnerEmail, tenant.Tier,
	).Scan(&tenant.ID, &tenant.CreatedAt, &tenant.UpdatedAt)

	if err != nil {
		return err
	}
	return tx.Commit()
}

func (s *PostgresTenantStore) GetTenant(ctx context.Context, id string) (*Tenant, error) {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, name, category, description, status, created_at, updated_at, owner_email, tier
		FROM tenants
		WHERE id = $1
	`
	row := tx.QueryRowContext(ctx, query, id)

	tenant := &Tenant{}
	err = row.Scan(
		&tenant.ID, &tenant.Name, &tenant.Category, &tenant.Description, &tenant.Status,
		&tenant.CreatedAt, &tenant.UpdatedAt, &tenant.OwnerEmail, &tenant.Tier,
	)

	if err == sql.ErrNoRows {
		return nil, errors.New("tenant not found")
	} else if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}
	return tenant, nil
}

func (s *PostgresTenantStore) UpdateTenantStatus(ctx context.Context, id string, status string) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `UPDATE tenants SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	_, err = tx.ExecContext(ctx, query, status, id)
	if err != nil {
		return err
	}
	return tx.Commit()
}

type SqliteTenantStore struct {
	db *sql.DB
}

func NewSqliteTenantStore(db *sql.DB) *SqliteTenantStore {
	return &SqliteTenantStore{db: db}
}

func (s *SqliteTenantStore) CreateTenant(ctx context.Context, tenant *Tenant) error {
	query := `
		INSERT INTO tenants (id, name, category, description, status, owner_email, tier, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	if tenant.ID == "" {
		tenant.ID = "id-" + time.Now().Format("20060102150405.000000") // Mock UUID
	}
	if tenant.Status == "" {
		tenant.Status = "PENDING"
	}

	_, err := s.db.ExecContext(ctx, query,
		tenant.ID, tenant.Name, tenant.Category, tenant.Description, tenant.Status,
		tenant.OwnerEmail, tenant.Tier,
	)

	if err == nil {
		tenant.CreatedAt = time.Now()
		tenant.UpdatedAt = time.Now()
	}

	return err
}

func (s *SqliteTenantStore) GetTenant(ctx context.Context, id string) (*Tenant, error) {
	query := `
		SELECT id, name, category, description, status, created_at, updated_at, owner_email, tier
		FROM tenants
		WHERE id = ?
	`
	row := s.db.QueryRowContext(ctx, query, id)

	tenant := &Tenant{}
	var createdAtStr, updatedAtStr string
	err := row.Scan(
		&tenant.ID, &tenant.Name, &tenant.Category, &tenant.Description, &tenant.Status,
		&createdAtStr, &updatedAtStr, &tenant.OwnerEmail, &tenant.Tier,
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
