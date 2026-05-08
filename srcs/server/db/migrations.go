package db

import (
	"database/sql"
)

// RunBuilderMigrations sets up postgres schema and RLS for builder tables.
func RunBuilderMigrations(db *sql.DB) error {
	queries := []string{
		`CREATE TABLE IF NOT EXISTS builder_sites (
			id VARCHAR(255) PRIMARY KEY,
			tenant_id VARCHAR(255) NOT NULL,
			domain VARCHAR(255),
			custom_domain VARCHAR(255),
			status VARCHAR(50) DEFAULT 'DRAFT',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);`,
		`ALTER TABLE builder_sites ENABLE ROW LEVEL SECURITY;`,
		`DROP POLICY IF EXISTS tenant_isolation_sites ON builder_sites;`,
		`CREATE POLICY tenant_isolation_sites ON builder_sites
			USING (tenant_id = current_setting('app.tenant_id', TRUE));`,

		`CREATE TABLE IF NOT EXISTS builder_pages (
			id VARCHAR(255) PRIMARY KEY,
			site_id VARCHAR(255) NOT NULL REFERENCES builder_sites(id) ON DELETE CASCADE,
			tenant_id VARCHAR(255) NOT NULL,
			path VARCHAR(255) NOT NULL,
			title VARCHAR(255),
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);`,
		`ALTER TABLE builder_pages ENABLE ROW LEVEL SECURITY;`,
		`DROP POLICY IF EXISTS tenant_isolation_pages ON builder_pages;`,
		`CREATE POLICY tenant_isolation_pages ON builder_pages
			USING (tenant_id = current_setting('app.tenant_id', TRUE));`,

		`CREATE TABLE IF NOT EXISTS builder_blocks (
			id VARCHAR(255) PRIMARY KEY,
			page_id VARCHAR(255) NOT NULL REFERENCES builder_pages(id) ON DELETE CASCADE,
			tenant_id VARCHAR(255) NOT NULL,
			type VARCHAR(255) NOT NULL,
			order_idx INTEGER NOT NULL DEFAULT 0,
			content JSONB,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);`,
		`ALTER TABLE builder_blocks ENABLE ROW LEVEL SECURITY;`,
		`DROP POLICY IF EXISTS tenant_isolation_blocks ON builder_blocks;`,
		`CREATE POLICY tenant_isolation_blocks ON builder_blocks
			USING (tenant_id = current_setting('app.tenant_id', TRUE));`,
	}

	for _, query := range queries {
		_, err := db.Exec(query)
		if err != nil {
			return err
		}
	}
	return nil
}
