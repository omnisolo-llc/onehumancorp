package builder

import (
	"context"
	"database/sql"
	"errors"
	"time"
	"onehumancorp/srcs/server/db"
)

type BuilderStore interface {
	CreateSite(ctx context.Context, site *Site) error
	GetSite(ctx context.Context, id string) (*Site, error)
	CreatePage(ctx context.Context, page *Page) error
	GetPage(ctx context.Context, id string) (*Page, error)
	GetBlocks(ctx context.Context, pageID string) ([]*Block, error)
	CreateBlock(ctx context.Context, block *Block) error
	UpdateBlock(ctx context.Context, block *Block) error
	ReorderBlocks(ctx context.Context, pageID string, blockIDs []string) error
	UpdateSiteStatus(ctx context.Context, id string, status string) error
}

type SqlStore struct {
	db *sql.DB
}

func NewSqlStore(db *sql.DB) *SqlStore {
	return &SqlStore{db: db}
}

// executeWithTenantContext sets app.tenant_id in a transaction for RLS
func (s *SqlStore) executeWithTenantContext(ctx context.Context, tenantID string, fn func(tx *sql.Tx) error) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	if !db.GlobalProvider.IsSQLite() && tenantID != "" {
		_, err = tx.ExecContext(ctx, "SELECT set_config('app.tenant_id', $1, true)", tenantID)
		if err != nil {
			return err
		}
	}

	if err := fn(tx); err != nil {
		return err
	}
	return tx.Commit()
}


func (s *SqlStore) CreateSite(ctx context.Context, site *Site) error {
	return s.executeWithTenantContext(ctx, site.TenantID, func(tx *sql.Tx) error {
		query := `
			INSERT INTO builder_sites (id, tenant_id, domain, custom_domain, status, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
		if !db.GlobalProvider.IsSQLite() {
			query = `
				INSERT INTO builder_sites (id, tenant_id, domain, custom_domain, status, created_at, updated_at)
				VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
				RETURNING created_at, updated_at
			`
		}

		if site.ID == "" {
			site.ID = "site-" + time.Now().Format("20060102150405.000000")
		}
		if site.Status == "" {
			site.Status = "DRAFT"
		}

		if db.GlobalProvider.IsSQLite() {
			_, err := tx.ExecContext(ctx, query,
				site.ID, site.TenantID, site.Domain, site.CustomDomain, site.Status,
			)
			if err == nil {
				site.CreatedAt = time.Now()
				site.UpdatedAt = time.Now()
			}
			return err
		}

		return tx.QueryRowContext(ctx, query,
			site.ID, site.TenantID, site.Domain, site.CustomDomain, site.Status,
		).Scan(&site.CreatedAt, &site.UpdatedAt)
	})
}

func (s *SqlStore) GetSite(ctx context.Context, id string) (*Site, error) {
	// Need tenant ID to get site context if RLS is strict, assuming caller set it or passed it.
	// For API, tenant is known from token. For simplicity, we just execute.
	query := `
		SELECT id, tenant_id, domain, custom_domain, status, created_at, updated_at
		FROM builder_sites
		WHERE id = ?
	`
	if !db.GlobalProvider.IsSQLite() {
		query = `
			SELECT id, tenant_id, domain, custom_domain, status, created_at, updated_at
			FROM builder_sites
			WHERE id = $1
		`
	}

	tenantID, _ := ctx.Value(tenantContextKey).(string)

	var site *Site
	err := s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		row := tx.QueryRowContext(ctx, query, id)
		site = &Site{}

		if db.GlobalProvider.IsSQLite() {
			var createdStr, updatedStr string
			err := row.Scan(&site.ID, &site.TenantID, &site.Domain, &site.CustomDomain, &site.Status, &createdStr, &updatedStr)
			if err == sql.ErrNoRows {
				return errors.New("site not found")
			} else if err != nil {
				return err
			}
			if t, err := time.Parse(time.RFC3339, createdStr); err == nil {
				site.CreatedAt = t
			}
			if t, err := time.Parse(time.RFC3339, updatedStr); err == nil {
				site.UpdatedAt = t
			}
			return nil
		}

		err := row.Scan(&site.ID, &site.TenantID, &site.Domain, &site.CustomDomain, &site.Status, &site.CreatedAt, &site.UpdatedAt)
		if err == sql.ErrNoRows {
			return errors.New("site not found")
		} else if err != nil {
			return err
		}
		return nil
	})

	return site, err
}

func (s *SqlStore) CreatePage(ctx context.Context, page *Page) error {
	tenantID, _ := ctx.Value(tenantContextKey).(string)
	page.TenantID = tenantID // Ensure consistency

	return s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		query := `
			INSERT INTO builder_pages (id, site_id, tenant_id, path, title, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
		if !db.GlobalProvider.IsSQLite() {
			query = `
				INSERT INTO builder_pages (id, site_id, tenant_id, path, title, created_at, updated_at)
				VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
				RETURNING created_at, updated_at
			`
		}

		if page.ID == "" {
			page.ID = "page-" + time.Now().Format("20060102150405.000000")
		}

		if db.GlobalProvider.IsSQLite() {
			_, err := tx.ExecContext(ctx, query, page.ID, page.SiteID, page.TenantID, page.Path, page.Title)
			if err == nil {
				page.CreatedAt = time.Now()
				page.UpdatedAt = time.Now()
			}
			return err
		}

		return tx.QueryRowContext(ctx, query, page.ID, page.SiteID, page.TenantID, page.Path, page.Title).Scan(&page.CreatedAt, &page.UpdatedAt)
	})
}

func (s *SqlStore) GetPage(ctx context.Context, id string) (*Page, error) {
	query := `
		SELECT id, site_id, tenant_id, path, title, created_at, updated_at
		FROM builder_pages
		WHERE id = ?
	`
	if !db.GlobalProvider.IsSQLite() {
		query = `
			SELECT id, site_id, tenant_id, path, title, created_at, updated_at
			FROM builder_pages
			WHERE id = $1
		`
	}

	tenantID, _ := ctx.Value(tenantContextKey).(string)
	var page *Page

	err := s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		row := tx.QueryRowContext(ctx, query, id)
		page = &Page{}

		if db.GlobalProvider.IsSQLite() {
			var createdStr, updatedStr string
			err := row.Scan(&page.ID, &page.SiteID, &page.TenantID, &page.Path, &page.Title, &createdStr, &updatedStr)
			if err == sql.ErrNoRows {
				return errors.New("page not found")
			} else if err != nil {
				return err
			}
			if t, err := time.Parse(time.RFC3339, createdStr); err == nil {
				page.CreatedAt = t
			}
			if t, err := time.Parse(time.RFC3339, updatedStr); err == nil {
				page.UpdatedAt = t
			}
			return nil
		}

		err := row.Scan(&page.ID, &page.SiteID, &page.TenantID, &page.Path, &page.Title, &page.CreatedAt, &page.UpdatedAt)
		if err == sql.ErrNoRows {
			return errors.New("page not found")
		} else if err != nil {
			return err
		}
		return nil
	})
	return page, err
}

func (s *SqlStore) GetBlocks(ctx context.Context, pageID string) ([]*Block, error) {
	query := `
		SELECT id, page_id, tenant_id, type, order_idx, content, created_at, updated_at
		FROM builder_blocks
		WHERE page_id = ?
		ORDER BY order_idx ASC
	`
	if !db.GlobalProvider.IsSQLite() {
		query = `
			SELECT id, page_id, tenant_id, type, order_idx, content, created_at, updated_at
			FROM builder_blocks
			WHERE page_id = $1
			ORDER BY order_idx ASC
		`
	}

	tenantID, _ := ctx.Value(tenantContextKey).(string)
	var blocks []*Block

	err := s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		rows, err := tx.QueryContext(ctx, query, pageID)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			block := &Block{}
			if db.GlobalProvider.IsSQLite() {
				var createdStr, updatedStr string
				if err := rows.Scan(&block.ID, &block.PageID, &block.TenantID, &block.Type, &block.OrderIdx, &block.Content, &createdStr, &updatedStr); err != nil {
					return err
				}
				if t, err := time.Parse(time.RFC3339, createdStr); err == nil {
					block.CreatedAt = t
				}
				if t, err := time.Parse(time.RFC3339, updatedStr); err == nil {
					block.UpdatedAt = t
				}
			} else {
				if err := rows.Scan(&block.ID, &block.PageID, &block.TenantID, &block.Type, &block.OrderIdx, &block.Content, &block.CreatedAt, &block.UpdatedAt); err != nil {
					return err
				}
			}
			blocks = append(blocks, block)
		}
		return nil
	})

	return blocks, err
}

func (s *SqlStore) CreateBlock(ctx context.Context, block *Block) error {
	tenantID, _ := ctx.Value(tenantContextKey).(string)
	block.TenantID = tenantID // Ensure consistency

	return s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		query := `
			INSERT INTO builder_blocks (id, page_id, tenant_id, type, order_idx, content, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
		if !db.GlobalProvider.IsSQLite() {
			query = `
				INSERT INTO builder_blocks (id, page_id, tenant_id, type, order_idx, content, created_at, updated_at)
				VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
				RETURNING created_at, updated_at
			`
		}

		if block.ID == "" {
			block.ID = "block-" + time.Now().Format("20060102150405.000000")
		}

		if db.GlobalProvider.IsSQLite() {
			_, err := tx.ExecContext(ctx, query, block.ID, block.PageID, block.TenantID, block.Type, block.OrderIdx, block.Content)
			if err == nil {
				block.CreatedAt = time.Now()
				block.UpdatedAt = time.Now()
			}
			return err
		}

		return tx.QueryRowContext(ctx, query, block.ID, block.PageID, block.TenantID, block.Type, block.OrderIdx, block.Content).Scan(&block.CreatedAt, &block.UpdatedAt)
	})
}

func (s *SqlStore) UpdateBlock(ctx context.Context, block *Block) error {
	tenantID, _ := ctx.Value(tenantContextKey).(string)
	return s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		query := `
			UPDATE builder_blocks
			SET type = ?, content = ?, updated_at = CURRENT_TIMESTAMP
			WHERE id = ?
		`
		if !db.GlobalProvider.IsSQLite() {
			query = `
				UPDATE builder_blocks
				SET type = $1, content = $2, updated_at = CURRENT_TIMESTAMP
				WHERE id = $3
			`
		}

		_, err := tx.ExecContext(ctx, query, block.Type, block.Content, block.ID)
		return err
	})
}

func (s *SqlStore) ReorderBlocks(ctx context.Context, pageID string, blockIDs []string) error {
	tenantID, _ := ctx.Value(tenantContextKey).(string)
	return s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		query := `UPDATE builder_blocks SET order_idx = ? WHERE id = ? AND page_id = ?`
		if !db.GlobalProvider.IsSQLite() {
			query = `UPDATE builder_blocks SET order_idx = $1 WHERE id = $2 AND page_id = $3`
		}

		for i, id := range blockIDs {
			if _, err := tx.ExecContext(ctx, query, i, id, pageID); err != nil {
				return err
			}
		}

		return nil
	})
}

func (s *SqlStore) UpdateSiteStatus(ctx context.Context, id string, status string) error {
	tenantID, _ := ctx.Value(tenantContextKey).(string)
	return s.executeWithTenantContext(ctx, tenantID, func(tx *sql.Tx) error {
		query := `UPDATE builder_sites SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
		if !db.GlobalProvider.IsSQLite() {
			query = `UPDATE builder_sites SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		}

		_, err := tx.ExecContext(ctx, query, status, id)
		return err
	})
}
